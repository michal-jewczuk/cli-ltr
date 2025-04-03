use tui::widgets::ListState;

pub struct Menu<'a> {
    pub state: ListState,
    pub items: Vec<&'a str>,
}


impl<'a> Menu<'a> {
    pub fn new(items: Vec<&'a str>) -> Self {
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

