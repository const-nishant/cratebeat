// Layout rendering
use ratatui::text::{Span, Spans};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Row, Table},
    Frame,
};

use crate::ui::ascii::BANNER;

pub fn render_layout<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    playhead: usize,
    cursor: (usize, usize),
    playing: bool,
    bpm: u32,
) {
    let size = f.size();

    // Split: top banner, middle grid, bottom info
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(6),
                Constraint::Min(8),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size);

    // Banner
    let banner =
        Paragraph::new(BANNER).block(Block::default().borders(Borders::ALL).title("CrateBeat"));
    f.render_widget(banner, chunks[0]);

    // Sequencer grid
    let grid_block = Block::default()
        .borders(Borders::ALL)
        .title("Sequencer (use arrow keys, space to toggle)");
    let inner = grid_block.inner(chunks[1]);
    f.render_widget(grid_block, chunks[1]);

    render_grid(f, inner, playhead, cursor);

    // Bottom info: BPM and play status
    let status = if playing {
        "Playing ▶"
    } else {
        "Stopped ■"
    };
    let footer = Paragraph::new(vec![Spans::from(vec![Span::raw(format!(
        "Status: {}  |  BPM: {}  |  Controls: p Play/Stop  q Quit  a/s/d/f Pads  +/- BPM",
        status, bpm
    ))])]);
    f.render_widget(footer, chunks[2]);
}

fn render_grid<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    playhead: usize,
    cursor: (usize, usize),
) {
    use ratatui::widgets::Cell;
    let tracks = vec!["Kick", "Snare", "Hat", "Clap"];
    let steps = 8;

    let mut rows = Vec::new();
    for (ti, name) in tracks.iter().enumerate() {
        let mut cells = vec![Cell::from(*name)];
        for si in 0..steps {
            let mut label = "[ ]".to_string();
            if si == playhead {
                // show playhead marker inside
                label = format!("[{}]", if si == playhead { "•" } else { " " });
            }
            // indicate cursor
            if cursor == (ti, si) {
                label = format!("({})", si);
            }
            cells.push(Cell::from(label));
        }
        rows.push(Row::new(cells));
    }

    let mut table = Table::new(rows, None)
        .widths(
            &std::iter::once(Constraint::Length(8))
                .chain(std::iter::repeat(Constraint::Length(6)).take(steps))
                .collect::<Vec<_>>(),
        )
        .column_spacing(1)
        .block(Block::default());

    f.render_widget(table, area);
}
