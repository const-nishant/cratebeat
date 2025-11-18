// Playback engine
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};

use crate::sequencer::clock::step_duration_millis;
use crate::sequencer::steps::{empty_pattern, Pattern};
use anyhow::Result;

/// Commands sent to the engine thread
#[derive(Debug)]
pub enum SequencerCommand {
    Configure { bpm: u32, steps: usize },
    SetPattern { pattern: Pattern },
    ToggleStep { track: usize, step: usize },
    TogglePlay,
    Play,
    Stop,
    AdjustBpm(i32),
}

/// Events emitted by the engine (for UI / app)
#[derive(Debug)]
pub enum SequencerEvent {
    Step { step: usize, hits: Vec<bool> },
    PlaybackState(bool),
}

/// Small sequencer engine that runs in its own thread and responds to commands.
/// It uses a blocking receive for commands but still maintains timing using precise sleep.
pub struct SequencerEngine {
    evt_tx: Sender<SequencerEvent>,

    // internal state
    bpm: u32,
    steps: usize,
    pattern: Pattern,
    playing: bool,
}

impl SequencerEngine {
    pub fn new(evt_tx: Sender<SequencerEvent>) -> Result<Self> {
        Ok(Self {
            evt_tx,
            bpm: 120,
            steps: 8,
            pattern: empty_pattern(4, 8),
            playing: false,
        })
    }

    /// Run the engine. This method blocks. It expects a receiver for commands.
    /// It contains a loop that checks for commands and when playing, advances steps using precise timing.
    pub fn run(&mut self, cmd_rx: Receiver<SequencerCommand>) {
        let mut step_idx = 0usize;
        // last tick time
        let mut last_instant = Instant::now();

        loop {
            // Calculate step duration in ms (assume 4 steps per beat for 16th notes)
            let step_ms = step_duration_millis(self.bpm, 4);

            // Non-blocking try_recv to handle commands quickly
            while let Ok(cmd) = cmd_rx.try_recv() {
                match cmd {
                    SequencerCommand::Configure { bpm, steps } => {
                        self.bpm = bpm;
                        self.steps = steps;
                        self.pattern = empty_pattern(4, steps);
                    }
                    SequencerCommand::SetPattern { pattern } => {
                        self.pattern = pattern;
                        self.steps = self.pattern.get(0).map(|r| r.len()).unwrap_or(self.steps);
                    }
                    SequencerCommand::ToggleStep { track, step } => {
                        if let Some(row) = self.pattern.get_mut(track) {
                            if step < row.len() {
                                row[step] = !row[step];
                            }
                        }
                    }
                    SequencerCommand::TogglePlay => {
                        self.playing = !self.playing;
                        let _ = self
                            .evt_tx
                            .send(SequencerEvent::PlaybackState(self.playing));
                    }
                    SequencerCommand::Play => {
                        self.playing = true;
                        let _ = self.evt_tx.send(SequencerEvent::PlaybackState(true));
                    }
                    SequencerCommand::Stop => {
                        self.playing = false;
                        let _ = self.evt_tx.send(SequencerEvent::PlaybackState(false));
                    }
                    SequencerCommand::AdjustBpm(delta) => {
                        let new_bpm = (self.bpm as i32 + delta).clamp(20, 300) as u32;
                        self.bpm = new_bpm;
                    }
                }
            }

            if self.playing {
                let now = Instant::now();
                if now.duration_since(last_instant) >= Duration::from_millis(step_ms) {
                    // advance step
                    last_instant = now;
                    // collect hits for this step
                    let mut hits = vec![];
                    for track in 0..self.pattern.len() {
                        let val = self.pattern[track][step_idx % self.steps];
                        hits.push(val);
                    }
                    let _ = self.evt_tx.send(SequencerEvent::Step {
                        step: step_idx % self.steps,
                        hits,
                    });
                    step_idx = (step_idx + 1) % self.steps;
                } else {
                    // small sleep to avoid busy loop
                    std::thread::sleep(Duration::from_millis(1));
                }
            } else {
                // when not playing we block waiting for commands to avoid CPU spin
                match cmd_rx.recv() {
                    Ok(cmd) => {
                        // re-send the command into the regular handling path by pushing to a short buffer:
                        // we simply handle it right here (duplicate of above)
                        match cmd {
                            SequencerCommand::Configure { bpm, steps } => {
                                self.bpm = bpm;
                                self.steps = steps;
                                self.pattern = empty_pattern(4, steps);
                            }
                            SequencerCommand::SetPattern { pattern } => {
                                self.pattern = pattern;
                                self.steps =
                                    self.pattern.get(0).map(|r| r.len()).unwrap_or(self.steps);
                            }
                            SequencerCommand::ToggleStep { track, step } => {
                                if let Some(row) = self.pattern.get_mut(track) {
                                    if step < row.len() {
                                        row[step] = !row[step];
                                    }
                                }
                            }
                            SequencerCommand::TogglePlay => {
                                self.playing = !self.playing;
                                let _ = self
                                    .evt_tx
                                    .send(SequencerEvent::PlaybackState(self.playing));
                            }
                            SequencerCommand::Play => {
                                self.playing = true;
                                let _ = self.evt_tx.send(SequencerEvent::PlaybackState(true));
                            }
                            SequencerCommand::Stop => {
                                self.playing = false;
                                let _ = self.evt_tx.send(SequencerEvent::PlaybackState(false));
                            }
                            SequencerCommand::AdjustBpm(delta) => {
                                let new_bpm = (self.bpm as i32 + delta).clamp(20, 300) as u32;
                                self.bpm = new_bpm;
                            }
                        }
                    }
                    Err(_) => {
                        // channel closed => exit thread
                        break;
                    }
                }
            }
        }
    }
}
