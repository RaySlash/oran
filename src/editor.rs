use crate::Terminal;
use std::io::Error;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                panic_program(&error);
            }
            if self.quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                panic_program(&error);
            }
        }
    }

    pub fn default() -> Self {
        Self {
            quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(0, 0);
        if self.quit {
            Terminal::clear_screen();
            println!("Goodbye. \r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
        Terminal::cursor_show();
        Ok(Terminal::flush())
    }

    fn process_keypress(&mut self) -> Result<(), Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.quit = true,
            _ => (),
        }
        Ok(())
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_dashboard();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_dashboard(&self) {
        let greeter = format!("Oran | v{}\r", VERSION);
        let width = self.terminal.size().width as usize;
        let len = greeter.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        greeter = format("~{}{}", spaces, greeter);
        greeter.truncate(width);
        println!("{}", greeter);
    }
}

fn panic_program(e: &Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
