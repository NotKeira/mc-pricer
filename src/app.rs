pub struct App {
    pub fields: Vec<(&'static str, String)>,
    pub selected: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            fields: vec![
                ("RAM (GB)", "8".to_string()),
                ("Player Slots", "20".to_string()),
                ("Worlds", "1".to_string()),
                ("Plugins", "10".to_string()),
                ("Mods", "0".to_string()),
                ("Servers", "1".to_string()),
            ],
            selected: 0,
        }
    }

    pub fn current_value_as_u32(&self, index: usize) -> u32 {
        self.fields
            .get(index)
            .and_then(|(_, val)| val.parse::<u32>().ok())
            .unwrap_or(0)
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.fields.len() {
            self.selected += 1;
        }
    }

    pub fn append_digit(&mut self, c: char) {
        if c.is_ascii_digit() {
            self.fields[self.selected].1.push(c);
        }
    }

    pub fn backspace(&mut self) {
        self.fields[self.selected].1.pop();
    }
}
