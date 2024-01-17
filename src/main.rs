use std::io::{self, stdout, Error, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    for input in io::stdin().bytes() {
        let byte = input.unwrap();
        let char = byte as char;

        if char.is_control() {
            println!("{:?} \r", char);
        } else {
            println!("{:?} ({})\r", char, byte);
        }

        if byte == 17 {
            break;
        }
    }
}

fn panic_program(e: Error) {
    panic!(e);
}
