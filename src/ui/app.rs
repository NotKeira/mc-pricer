pub struct App {
    pub fields: Vec<(&'static str, String)>,
    pub selected: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            fields: vec![
                ("Num Players", String::new()),
                ("World Size (GB)", String::new()),
                ("RAM (GB)", String::new()),
                ("CPU Cores", String::new()),
                ("Plugins", String::new()),
                ("Bandwidth (Mbps)", String::new()),
            ],
            selected: 0,
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected < self.fields.len() - 1 {
            self.selected += 1;
        }
    }

    pub fn append_digit(&mut self, c: char) {
        self.fields[self.selected].1.push(c);
    }

    pub fn backspace(&mut self) {
        self.fields[self.selected].1.pop();
    }

    pub fn current_value_as_u32(&self, index: usize) -> u32 {
        self.fields[index].1.parse().unwrap_or(0)
    }
}
