use crate::Terminal;
use std::io::Error;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    quit: bool,
    terminal: Terminal,
    cursor_position: Position,
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
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        if self.quit {
            Terminal::clear_screen();
            println!("Goodbye. \r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Ok(Terminal::flush())
    }

    fn process_keypress(&mut self) -> Result<(), Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.quit = true,
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                };
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                };
            }
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        self.cursor_position = Position { x, y }
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
        let mut greeter = format!("Oran | v{}\r", VERSION);
        let width = self.terminal.size().width as usize;
        let len = greeter.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        greeter = format!("~{}{}", spaces, greeter);
        greeter.truncate(width);
        println!("{}", greeter);
    }
}

fn panic_program(e: &Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
