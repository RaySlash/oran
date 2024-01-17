use std::io::{self, stdout, Error, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

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
        Self { quit: false }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        if self.quit {
            println!("Goodbye. \r");
        } else {
            self.draw_rows();
            print!("{}", termion::cursor::Goto(1, 1));
        }
        io::stdout().flush()
    }

    fn process_keypress(&mut self) -> Result<(), Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.quit = true,
            _ => (),
        }
        Ok(())
    }
    fn draw_rows(&self) {
        for _ in 0..24 {
            println!("~\r")
        }
    }
}

fn read_key() -> Result<Key, Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn panic_program(e: &Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
