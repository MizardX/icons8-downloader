#[derive(Default)]
pub struct IconPacks {
    pub icon_packs_idx: usize,
    pub icon_list_idx: usize,
    pub list: Vec<IconPack>,
}

pub struct IconPack {
    pub title: String,
    pub icons: Vec<Icon>,
}

pub struct Icon {
    pub name: String,
    pub id: String,
}

impl IconPack {
    pub fn new(title: String) -> IconPack {
        let mut icons = vec![];

        // This is only mock data, until we add api support.
        for i in 1..50 {
            icons.push(Icon {
                name: format!("Mock Icon {i}"),
                id: String::from("0"),
            });
        }

        IconPack { title, icons }
    }
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
}

impl App {
    pub fn new() -> App {
        let mut icon_packs = IconPacks::default();
        let state = AppState::default();

        // This is only mock data, until we add api support.
        for i in 1..16 {
            icon_packs
                .list
                .push(IconPack::new(format!("Mock Icon Pack {i}")));
        }

        App { icon_packs, state }
    }
    
    pub fn switch_list(&mut self, state: AppState) {
        if self.state != state {
            self.state = state;
        }
    }

    pub fn next(&mut self) {
        match self.state {
            AppState::IconPacks => self.next_pack(),
            AppState::IconList => self.next_icon()
        }
    }

    pub fn previous(&mut self) {
        match self.state {
            AppState::IconPacks => self.previous_pack(),
            AppState::IconList => self.previous_icon()
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
