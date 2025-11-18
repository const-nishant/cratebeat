use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use anyhow::Result;

use crate::audio::player::Player;
use crate::sequencer::engine::{SequencerCommand, SequencerEvent, SequencerEngine};

pub struct App {
    pub ui: crate::ui::Ui,
    player: Player,
    seq_tx: Sender<SequencerCommand>,
    seq_rx: Receiver<SequencerEvent>,
}

impl App {
    pub fn new() -> Result<Self> {
        // UI
        let ui = crate::ui::Ui::new()?;

        // Audio player (loads sounds from sounds/)
        let mut player = Player::new()?;
        // Try load known sample names (will ignore if file missing)
        player.load("kick", "sounds/kick.wav").ok();
        player.load("snare", "sounds/snare.wav").ok();
        player.load("hat", "sounds/hat.wav").ok();
        player.load("clap", "sounds/clap.wav").ok();

        // Sequencer channels
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (evt_tx, evt_rx) = mpsc::channel();

        // Create and run sequencer engine in another thread
        let mut engine = SequencerEngine::new(evt_tx)?;
        thread::spawn(move || {
            engine.run(cmd_rx);
        });

        Ok(Self {
            ui,
            player,
            seq_tx: cmd_tx,
            seq_rx: evt_rx,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        // Initial app state: create a default pattern
        // Pattern of 4 tracks with 8 steps
        let tracks = vec!["kick", "snare", "hat", "clap"];
        let mut pattern = vec![vec![false; 8]; tracks.len()];

        // Example default pattern
        pattern[0][0] = true; // kick on step 0
        pattern[0][2] = true;
        pattern[1][1] = true; // snare on step 1
        for i in 0..8 {
            pattern[2][i] = i % 2 == 0; // hi-hat on even steps
        }

        // Set sequencer initial config
        self.seq_tx
            .send(SequencerCommand::Configure { bpm: 120, steps: 8 })
            .ok();
        self.seq_tx
            .send(SequencerCommand::SetPattern { pattern: pattern.clone() })
            .ok();

        // Ask UI to enter main loop. UI will return user actions which we forward to sequencer/player.
        loop {
            // Poll UI for next user action (blocking until available or UI requests exit)
            match self.ui.tick()? {
                crate::ui::UiEvent::Exit => {
                    // Stop sequencer cleanly
                    self.seq_tx.send(SequencerCommand::Stop).ok();
                    break;
                }
                crate::ui::UiEvent::PlayToggle => {
                    self.seq_tx.send(SequencerCommand::TogglePlay).ok();
                }
                crate::ui::UiEvent::BpmChange(delta) => {
                    self.seq_tx.send(SequencerCommand::AdjustBpm(delta)).ok();
                }
                crate::ui::UiEvent::ToggleStep(track_idx, step) => {
                    // forward to engine
                    self.seq_tx
                        .send(SequencerCommand::ToggleStep { track: track_idx, step })
                        .ok();
                }
                crate::ui::UiEvent::ManualHit(opt_name) => {
                    if let Some(name) = opt_name {
                        self.player.play(&name);
                    }
                }
                crate::ui::UiEvent::Noop => {}
            }

            // Handle incoming sequencer events: play sounds when engine says to
            while let Ok(ev) = self.seq_rx.try_recv() {
                match ev {
                    SequencerEvent::Step { step, hits } => {
                        // hits: Vec<Option<String>> or bool per track
                        for (maybe_name, should_play) in hits.iter().enumerate() {
                            let should_play = should_play;
                            if *should_play {
                                if let Some(name) = tracks.get(maybe_name) {
                                    self.player.play(name);
                                }
                            }
                        }
                        // Tell UI to update playhead
                        self.ui.set_playhead(step);
                    }
                    SequencerEvent::PlaybackState(_running) => {
                        // could reflect to UI
                    }
                }
            }
        }

        // Clean up UI (restore terminal)
        self.ui.shutdown()?;
        Ok(())
    }
}
