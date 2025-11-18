// UI module
pub mod ascii;
pub mod input;
pub mod layout;

use anyhow::Result;
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io::{stdout, Stdout};

use crate::ui::ascii::BANNER;

#[derive(Debug)]
pub enum UiEvent {
    Exit,
    PlayToggle,
    BpmChange(i32),
    ToggleStep(usize, usize), // track, step
    ManualHit(Option<String>),
    Noop,
}

pub struct Ui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    playhead: usize,
    // UI state (simple)
    pub cursor: (usize, usize), // track, step
    pub playing: bool,
    pub bpm: u32,
}

impl Ui {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            playhead: 0,
            cursor: (0, 0),
            playing: false,
            bpm: 120,
        })
    }

    /// Called by app to advance visual playhead
    pub fn set_playhead(&mut self, idx: usize) {
        self.playhead = idx;
    }

    /// Ticks the UI: render and handle one input cycle.
    /// Blocking but with a short timeout for drawing smoother UI.
    pub fn tick(&mut self) -> Result<UiEvent> {
        // Render UI
        self.terminal.draw(|f| {
            crate::ui::layout::render_layout(f, self.playhead, self.cursor, self.playing, self.bpm);
        })?;

        // Poll for input with timeout
        use crossterm::event::{poll, read};
        use std::time::Duration;

        if poll(Duration::from_millis(200))? {
            if let Event::Key(k) = read()? {
                return Ok(self.handle_key(k));
            }
        }
        Ok(UiEvent::Noop)
    }

    fn handle_key(&mut self, key: KeyEvent) -> UiEvent {
        match key.code {
            KeyCode::Char('q') => UiEvent::Exit,
            KeyCode::Char('p') => {
                self.playing = !self.playing;
                UiEvent::PlayToggle
            }
            KeyCode::Char('+') => {
                self.bpm = (self.bpm + 5).min(300);
                UiEvent::BpmChange(5)
            }
            KeyCode::Char('-') => {
                self.bpm = (self.bpm.saturating_sub(5)).max(20);
                UiEvent::BpmChange(-5)
            }
            KeyCode::Right => {
                self.cursor.1 = (self.cursor.1 + 1) % 8;
                UiEvent::Noop
            }
            KeyCode::Left => {
                self.cursor.1 = (8 + self.cursor.1 - 1) % 8;
                UiEvent::Noop
            }
            KeyCode::Down => {
                self.cursor.0 = (self.cursor.0 + 1) % 4;
                UiEvent::Noop
            }
            KeyCode::Up => {
                self.cursor.0 = (4 + self.cursor.0 - 1) % 4;
                UiEvent::Noop
            }
            KeyCode::Char(' ') => {
                // toggle step under cursor
                UiEvent::ToggleStep(self.cursor.0, self.cursor.1)
            }
            KeyCode::Char('a') => UiEvent::ManualHit(Some("kick".into())),
            KeyCode::Char('s') => UiEvent::ManualHit(Some("snare".into())),
            KeyCode::Char('d') => UiEvent::ManualHit(Some("hat".into())),
            KeyCode::Char('f') => UiEvent::ManualHit(Some("clap".into())),
            _ => UiEvent::Noop,
        }
    }

    pub fn shutdown(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
