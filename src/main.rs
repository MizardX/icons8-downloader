mod events;
mod ui;

use anyhow::Result;
use events::{EventType, Events};
use std::io;
use ui::{draw_icon_list, draw_icon_pack_list};

use crossterm::{
    event::KeyCode,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Paragraph, Wrap},
    Terminal,
};

#[tokio::main]
async fn main() -> Result<()> {
    start_ui().await?;
    Ok(())
}

async fn start_ui() -> Result<()> {
    let stdout = io::stdout();
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();

    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        terminal.autoresize()?;

        terminal.draw(|f| {
            let size = f.size();

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(1), Constraint::Min(2)])
                .split(size);

            let body_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(15), Constraint::Percentage(100)])
                .split(layout[1]);

            let header = Paragraph::new("Icons8 Downloader")
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });

            f.render_widget(header, layout[0]);
            draw_icon_pack_list(f, &body_layout[0], Some(0));
            draw_icon_list(f, &body_layout[1], Some(0));
        })?;

        // EventType::Tick will be implemented, although it is not needed so far
        // TODO: Next/Previous navigation in lists
        if let EventType::Input(KeyCode::Char('q')) = events.next() {
            break;
        }
    }

    disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())
}
