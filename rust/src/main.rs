use core::time;
use std::{
    error::Error,
    io::{self, stdin, stdout, Cursor, Write},
    thread::sleep,
};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Print, PrintStyledContent},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand, QueueableCommand,
};

mod snake;

fn main() -> io::Result<()> {
    let stdin = stdin();
    let mut stdout = stdout();

    enable_raw_mode()?;
    let _ = stdout.execute(cursor::Hide);
    let mut i = 0;
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
        i += 1;
        stdout.queue(cursor::MoveTo(i, 3))?;
        stdout.queue(Print("i"))?;
        stdout.flush()?;
        sleep(time::Duration::from_millis(200));
    }

    let _ = stdout.execute(cursor::Show);
    disable_raw_mode().unwrap();
    Ok(())
}
