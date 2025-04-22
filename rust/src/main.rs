use core::time;
use std::{
    io::{self, stdin, stdout, Write},
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{poll, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};

use snake::Direction;

mod snake;

fn main() -> io::Result<()> {
    let _stdin = stdin();
    let mut stdout = stdout();

    enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;

    let game_state = Arc::new(Mutex::new(snake::GameState::new()));

    let game_state_clone = Arc::clone(&game_state);
    let _input_thread_handle = thread::spawn(move || loop {
        if let Ok(input_present) = poll(Duration::from_millis(30)) {
            if input_present {
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
        }
        {
            let gs = game_state_clone.lock().unwrap();
            if !gs.active {
                break;
            }
        }
    });

    loop {
        snake::display::draw_field(&stdout)?;

        // UPDATE GAME STATE
        {
            let mut gs = game_state.lock().unwrap();
            if !gs.active {
                break;
            };

            gs.propagate();
            gs.check_gamestate();

            snake::display::draw_snake(&stdout, &gs)?;
            snake::display::draw_food(&stdout, &gs)?;
        }

        stdout.flush()?;
        sleep(time::Duration::from_millis(100));
    }

    stdout.flush()?;
    let _ = _input_thread_handle.join();
    let _ = stdout.execute(cursor::Show);
    disable_raw_mode().unwrap();
    Ok(())
}
