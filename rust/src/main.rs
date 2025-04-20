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
use snake::{Direction, Position};

mod snake;

fn main() -> io::Result<()> {
    let _stdin = stdin();
    let mut stdout = stdout();

    enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;

    let game_state = Arc::new(Mutex::new(snake::GameState {
        active: true,
        head_direction: Position { x: 1, y: 0 },
        head_pos: Position { x: 5, y: 5 },
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
                    }) => gs.set_direction(Direction::UP),
                    Event::Key(KeyEvent {
                        code: KeyCode::Down,
                        ..
                    }) => gs.set_direction(Direction::DOWN),
                    Event::Key(KeyEvent {
                        code: KeyCode::Left,
                        ..
                    }) => gs.set_direction(Direction::LEFT),
                    Event::Key(KeyEvent {
                        code: KeyCode::Right,
                        ..
                    }) => gs.set_direction(Direction::RIGHT),
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

        // UPDATE GAME STATE
        {
            let mut gs = game_state.lock().unwrap();
            if !gs.active {
                break;
            };

            gs.move_snakehead();
            stdout.queue(cursor::MoveTo(
                (gs.head_pos.x * 2).try_into().unwrap_or(0),
                gs.head_pos.y.try_into().unwrap_or(0),
            ))?;
        }

        stdout.queue(Print("[]"))?;
        stdout.flush()?;
        sleep(time::Duration::from_millis(100));
    }

    let _ = _input_thread_handle.join();
    let _ = stdout.execute(cursor::Show);
    disable_raw_mode().unwrap();
    Ok(())
}
