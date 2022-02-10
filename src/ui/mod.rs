use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

pub fn draw_icon_pack_list<B: Backend>(
    frame: &mut Frame<B>,
    rect_chunk: &Rect,
    selected_index: Option<usize>,
) {
    let mut state = ListState::default();
    state.select(selected_index);

    let block = Block::default().title("Icon Packs").borders(Borders::ALL);

    // This is just mock data, for the time being
    let items: Vec<ListItem> = vec![ListItem::new("IconPack 1"), ListItem::new("IconPack 2")];

    let result = List::new(items)
        .block(block)
        .highlight_symbol("> ")
        .highlight_style(Style::default().fg(Color::Yellow));

    frame.render_stateful_widget(result, *rect_chunk, &mut state);
}

pub fn draw_icon_list<B: Backend>(
    frame: &mut Frame<B>,
    rect_chunk: &Rect,
    selected_index: Option<usize>,
) {
    let mut state = ListState::default();
    state.select(selected_index);

    let block = Block::default().title("Icon List").borders(Borders::ALL);

    // This is just mock data, for the time being
    let items: Vec<ListItem> = vec![ListItem::new("Icon 1"), ListItem::new("Icon 2")];

    let result = List::new(items)
        .block(block)
        .highlight_symbol("> ")
        .highlight_style(Style::default().fg(Color::Yellow));

    frame.render_stateful_widget(result, *rect_chunk, &mut state);
}
