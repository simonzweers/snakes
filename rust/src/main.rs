use core::time;
use std::{
    io::{self, stdin, stdout, Write},
    thread::{self, sleep},
};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand, QueueableCommand,
};

use crossterm::style::Print;

mod snake;

fn main() -> io::Result<()> {
    let _stdin = stdin();
    let mut stdout = stdout();

    enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;
    
    let _input_thread_handle = thread::spawn(|| {
        for _ in 1..10 {
            println!("Hlpalsdkjfh");
        }
    });

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
