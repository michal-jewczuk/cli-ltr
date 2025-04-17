use tui::widgets::ListState;

pub struct Menu {
    pub state: ListState,
    pub items: Vec<String>,
}


impl Menu {
    pub fn home() -> Self {
        let items = vec![
            String::from("[t] Tests"),
            String::from("[r] Results"),
            String::from("[d] Redo"),
            String::from("[h] Help"),
            String::from("[q] Exit"),
        ];
        Self::new(items)
    }


    pub fn new(items: Vec<String>) -> Self {
        let mut state = ListState::default();
        if items.len() > 0 {
            state.select(Some(0));
        }
        Menu { state, items }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == self.items.len() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

