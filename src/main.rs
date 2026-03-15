use std::{io, vec};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal)) // Init tui
}

#[derive(Debug, Default)] // Tell the compiler to create a default instance with default values, and
//tell it to show logs
pub struct App {
    // Class
    counter: u8, // 8 bit unsigned (non negative) integer
    exit: bool,
}

impl App {
    // Impl lets you define functions for a struct (class)
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        //public function with a mutable reference to itself and to the terminal's location in memory

        while !self.exit {
            // While true
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area()); // Draw App as a widget
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            // Switch case
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left | KeyCode::Char('h') => self.decrement_counter(),
            KeyCode::Right | KeyCode::Char('l') => self.increment_counter(),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // See if there was an event
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                // For a key event, see if it was of type KeyPress
                self.handle_key_event(key_event) // Pass the input into handle function
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_error() {
        todo!();
    }
}

impl Widget for &App {
    //Reference to the App struct
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Counter App".bold());
        let instructions = Line::from(vec![
            //Define a row of styled text
            "Decrement".into(),
            "<H,Left>".blue().bold(),
            "Increment".into(),
            "<L,Right>".blue().bold(),
            "Quit".into(),
            "<Q>".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered()) // Set the instructions as the bottom header
            .border_set(border::THICK);
        let counter_text = Text::from(vec![Line::from(vec![
            "Value ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf); // New Paragraph that holds the counter value
    }
}
