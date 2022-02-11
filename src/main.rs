mod core;
mod events;
mod ui;

use crate::core::api;
use crate::core::app::{App, AppEvent, AppState};
use anyhow::Result;
use events::{EventType, Events};
use std::{io, sync::Arc};
use tokio::sync::{mpsc, Mutex};
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
    let (tx, rx) = mpsc::channel::<AppEvent>(100);
    let app = Arc::new(Mutex::new(App::new(tx)));
    let cloned_app = Arc::clone(&app);

    tokio::spawn(async move {
        start_events(rx, app).await;
    });

    start_ui(&cloned_app).await?;

    Ok(())
}

async fn start_events(mut rx: mpsc::Receiver<AppEvent>, app: Arc<Mutex<App>>) {
    while let Some(event) = rx.recv().await {
        match event {
            AppEvent::FetchIconPacks => api::fetch_icon_packs(&app).await,
        }
    }
}

async fn start_ui(app: &Arc<Mutex<App>>) -> Result<()> {
    let stdout = io::stdout();
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();
    let mut first_render = true;

    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        let mut app = app.lock().await;

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

            draw_icon_pack_list(f, &body_layout[0], &app);
            draw_icon_list(f, &body_layout[1], &app);
        })?;

        // EventType::Tick will be implemented, although it is not needed so far
        if let EventType::Input(i) = events.next() {
            match i {
                KeyCode::Char('q') => break,
                KeyCode::Up => app.previous(),
                KeyCode::Down => app.next(),
                KeyCode::Enter => app.switch_list(AppState::IconList),
                KeyCode::Esc => {
                    app.icon_packs.icon_list_idx = 0;
                    app.switch_list(AppState::IconPacks);
                }
                _ => (),
            }
        }

        if first_render {
            if app.tx.send(AppEvent::FetchIconPacks).await.is_err() {
                panic!("Unable to send event");
            }

            first_render = false;
        }
    }

    disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())
}
