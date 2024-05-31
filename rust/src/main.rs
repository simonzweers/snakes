use core::time;
use std::{
    io::{self, stdin, stdout, Cursor},
    thread::sleep,
};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};

mod snake;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout();

    enable_raw_mode().unwrap();
    let _ = stdout.execute(cursor::Hide);

    loop {
        match snake::display::draw_field(&stdout) {
            Ok(_) => println!("Drawing field succeeded"),
            Err(err) => println!("Whoops, cannot print correctly: {err}"),
        }
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => break,
            _ => (),
        }
        sleep(time::Duration::from_millis(200));
    }

    let _ = stdout.execute(cursor::Show);
    disable_raw_mode().unwrap();
}
