use crate::core::app::{App, AppState};

use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub fn draw_icon_pack_list<B: Backend>(frame: &mut Frame<B>, rect_chunk: &Rect, app: &App) {
    let mut state = ListState::default();
    state.select(Some(app.icon_packs.icon_packs_idx));

    let block = Block::default().title("Icon Packs").borders(Borders::ALL);

    if app.icon_packs.list.is_empty() {
        let result = Paragraph::new("Fetching icon packs..")
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(result, *rect_chunk);
        return;
    }

    let items: Vec<ListItem> = app
        .icon_packs
        .list
        .iter()
        .map(|i| ListItem::new(Span::raw(&i.title)))
        .collect();

    let result = List::new(items)
        .block(block)
        .highlight_symbol("> ")
        .highlight_style(Style::default().fg(Color::Yellow));

    match app.state {
        AppState::IconPacks => frame.render_stateful_widget(result, *rect_chunk, &mut state),
        AppState::IconList => frame.render_widget(result, *rect_chunk),
    }
}

pub fn draw_icon_list<B: Backend>(frame: &mut Frame<B>, rect_chunk: &Rect, app: &App) {
    let block = Block::default().title("Icon List").borders(Borders::ALL);

    if app.icon_packs.list.is_empty() {
        let result = Paragraph::new("Fetching icon packs..")
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(result, *rect_chunk);
        return;
    }

    let selected = app.icon_packs.icon_packs_idx;
    let icon_pack = &app.icon_packs.list[selected];

    if icon_pack.icons.is_empty() {
        let result = Paragraph::new("Fetching icons..")
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(result, *rect_chunk);
        return;
    }

    let mut state = ListState::default();
    state.select(Some(app.icon_packs.icon_list_idx));

    let items: Vec<ListItem> = icon_pack
        .icons
        .iter()
        .map(|i| ListItem::new(Span::raw(&i.name)))
        .collect();

    let result = List::new(items)
        .block(block)
        .highlight_symbol("> ")
        .highlight_style(Style::default().fg(Color::Yellow));

    match app.state {
        AppState::IconPacks => frame.render_widget(result, *rect_chunk),
        AppState::IconList => frame.render_stateful_widget(result, *rect_chunk, &mut state),
    }
}
