use core::time;
use std::{
    io::{self, stdin, stdout, Write},
    sync::{Arc, Mutex},
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

    let game_state = Arc::new(Mutex::new(snake::GameState {
        active: true,
        head_pos_y: 0,
        head_pos_x: 0,
    }));

    let game_state_clone = Arc::clone(&game_state);
    let _input_thread_handle = thread::spawn(move || {
        loop {
            let input = crossterm::event::read().unwrap();
            {
                let mut gs = game_state_clone.lock().unwrap();
                match input {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    }) => {
                        gs.active = false;
                        break;
                    }
                    Event::Key(KeyEvent {
                        code: KeyCode::Up, ..
                    }) => gs.head_pos_y -= 1,
                    Event::Key(KeyEvent {
                        code: KeyCode::Down,
                        ..
                    }) => gs.head_pos_y += 1,
                    Event::Key(KeyEvent {
                        code: KeyCode::Left,
                        ..
                    }) => gs.head_pos_x -= 1,
                    Event::Key(KeyEvent {
                        code: KeyCode::Right,
                        ..
                    }) => gs.head_pos_x += 1,
                    _ => (),
                }
            }
        }
        snake::gamelogic::handle_input();
    });

    let mut i = 0;
    loop {
        match snake::display::draw_field(&stdout) {
            Ok(_) => println!("Drawing field succeeded"),
            Err(err) => println!("Whoops, cannot print correctly: {err}"),
        }
        i += 1;
        stdout.queue(cursor::MoveTo(i, 3))?;
        stdout.queue(Print("i"))?;
        {
            let gs = game_state.lock().unwrap();
            if !gs.active {
                break;
            };
            stdout.queue(cursor::MoveTo(gs.head_pos_x * 2, gs.head_pos_y))?;
        }
        stdout.queue(Print("[]"))?;
        stdout.flush()?;
        sleep(time::Duration::from_millis(200));
    }

    let _ = _input_thread_handle.join();
    let _ = stdout.execute(cursor::Show);
    disable_raw_mode().unwrap();
    Ok(())
}
