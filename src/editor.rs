use crate::Terminal;
use std::io::Error;
use termion::event::Key;

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
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        if self.quit {
            println!("Goodbye. \r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
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
        for _ in 0..self.terminal.size().height - 1 {
            println!("~\r")
        }
    }
}

fn panic_program(e: &Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
