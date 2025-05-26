use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};
use std::{error::Error, io};
use crate::app::App;
use crate::pricing::PricingConfig;

pub struct UI {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    app: App,
    pricing: PricingConfig,
}

impl UI {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self {
            terminal,
            app: App::new(),
            pricing: PricingConfig::new(),
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            self.terminal.draw(|f| {
                let size = f.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(3)
                    .constraints(
                        [
                            Constraint::Length(self.app.fields.len() as u16 * 3),
                            Constraint::Length(3),
                            Constraint::Min(1),
                        ]
                        .as_ref(),
                    )
                    .split(size);

                // Input fields
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![Constraint::Length(3); self.app.fields.len()])
                    .split(chunks[0]);

                for (i, (label, value)) in self.app.fields.iter().enumerate() {
                    let style = if i == self.app.selected {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };

                    let input = Paragraph::new(value.as_str())
                        .style(style)
                        .block(Block::default().borders(Borders::ALL).title(*label));
                    f.render_widget(input, input_chunks[i]);
                }

                // Calculate cost
                let cost = self.pricing.calculate_cost(
                    self.app.current_value_as_u32(0),
                    self.app.current_value_as_u32(1),
                    self.app.current_value_as_u32(2),
                    self.app.current_value_as_u32(3),
                    self.app.current_value_as_u32(4),
                    self.app.current_value_as_u32(5),
                );
                let cost_paragraph = Paragraph::new(format!("Estimated Cost: £{:.2}", cost))
                    .block(Block::default().borders(Borders::ALL).title("Cost"));
                f.render_widget(cost_paragraph, chunks[1]);

                let help = Paragraph::new(
                    "Up/Down to select field, digits to edit, Backspace to delete, q to quit",
                )
                .style(Style::default().fg(Color::LightCyan))
                .block(Block::default().borders(Borders::ALL).title("Instructions"));
                f.render_widget(help, chunks[2]);
            })?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Up => self.app.move_up(),
                        KeyCode::Down => self.app.move_down(),
                        KeyCode::Backspace => self.app.backspace(),
                        KeyCode::Char(c) if c.is_ascii_digit() => self.app.append_digit(c),
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        disable_raw_mode().ok();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            crossterm::event::DisableMouseCapture
        )
        .ok();
        self.terminal.show_cursor().ok();
    }
}
