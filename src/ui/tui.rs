use super::app::App;
use super::event;
use crate::pricing::PricingConfig;
use crossterm::{
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
            self.render()?;
            if event::handle_events(&mut self.app)? {
                break;
            }
        }
        Ok(())
    }

    fn render(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(3)
                .constraints([
                    Constraint::Length(3), // Title
                    Constraint::Length(self.app.fields.len() as u16 * 3),
                    Constraint::Length(3), // Cost
                    Constraint::Length(7), // Help section (increased for vertical list)
                ])
                .split(size);

            // Title section
            let title = Paragraph::new("Minecraft Server Price Estimator")
                .style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            // Input fields
            let input_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(3); self.app.fields.len()])
                .split(chunks[1]);

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
            f.render_widget(cost_paragraph, chunks[2]);

            // Help section as vertical list
            let help = Paragraph::new(
                "Controls:\n\
                ↑/↓ - Select field\n\
                0-9 - Edit field\n\
                Backspace - Delete\n\
                q - Quit",
            )
            .style(Style::default().fg(Color::LightCyan))
            .block(Block::default().borders(Borders::ALL).title("Instructions"));
            f.render_widget(help, chunks[3]);
        })?;
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
