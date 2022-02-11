use tokio::sync::mpsc;

#[derive(Default)]
pub struct IconPacks {
    pub icon_packs_idx: usize,
    pub icon_list_idx: usize,
    pub list: Vec<IconPack>,
}

pub struct IconPack {
    pub title: String,
    pub api_code: String,
    pub icons: Vec<Icon>,
}

pub struct Icon {
    pub name: String,
    pub id: String,
}

impl IconPack {
    pub fn new(title: String, api_code: String) -> IconPack {
        IconPack {
            title,
            api_code,
            icons: vec![],
        }
    }
}

pub enum AppEvent {
    FetchIconPacks,
}

#[derive(PartialEq, Eq)]
pub enum AppState {
    IconPacks,
    IconList,
}

impl AppState {
    pub fn default() -> AppState {
        AppState::IconPacks
    }
}

pub struct App {
    pub icon_packs: IconPacks,
    pub state: AppState,
    pub tx: mpsc::Sender<AppEvent>,
}

impl App {
    pub fn new(tx: mpsc::Sender<AppEvent>) -> App {
        let icon_packs = IconPacks::default();
        let state = AppState::default();

        App {
            icon_packs,
            state,
            tx,
        }
    }

    pub fn switch_list(&mut self, state: AppState) {
        if self.state != state {
            match state {
                AppState::IconList => {
                    if !self.icon_packs.list.is_empty() {
                        let selected = self.icon_packs.icon_list_idx;
                        let icon_pack = &self.icon_packs.list[selected];

                        if icon_pack.icons.is_empty() {
                            return;
                        }

                        self.state = state;
                    }
                }
                _ => self.state = state,
            }
        }
    }

    pub fn next(&mut self) {
        match self.state {
            AppState::IconPacks => self.next_pack(),
            AppState::IconList => self.next_icon(),
        }
    }

    pub fn previous(&mut self) {
        match self.state {
            AppState::IconPacks => self.previous_pack(),
            AppState::IconList => self.previous_icon(),
        }
    }

    fn next_pack(&mut self) {
        let mut icon_packs = &mut self.icon_packs;

        if icon_packs.list.is_empty() {
            return;
        }

        if icon_packs.icon_packs_idx >= icon_packs.list.len() - 1 {
            icon_packs.icon_packs_idx = 0;
            return;
        }

        icon_packs.icon_packs_idx += 1;
    }

    fn next_icon(&mut self) {
        let mut icon_packs = &mut self.icon_packs;
        let selected = icon_packs.icon_packs_idx;
        let icons = &icon_packs.list[selected].icons;

        if icons.is_empty() {
            return;
        }

        if icon_packs.icon_list_idx >= icons.len() - 1 {
            icon_packs.icon_list_idx = 0;
            return;
        }

        icon_packs.icon_list_idx += 1;
    }

    fn previous_pack(&mut self) {
        let mut icon_packs = &mut self.icon_packs;

        if icon_packs.list.is_empty() {
            return;
        }

        if icon_packs.icon_packs_idx != 0 {
            icon_packs.icon_packs_idx -= 1;
            return;
        }

        icon_packs.icon_packs_idx = icon_packs.list.len() - 1;
    }

    fn previous_icon(&mut self) {
        let mut icon_packs = &mut self.icon_packs;
        let selected = icon_packs.icon_packs_idx;
        let icons = &icon_packs.list[selected].icons;

        if icons.is_empty() {
            return;
        }

        if icon_packs.icon_list_idx != 0 {
            icon_packs.icon_list_idx -= 1;
            return;
        }

        icon_packs.icon_list_idx = icons.len() - 1;
    }
}
