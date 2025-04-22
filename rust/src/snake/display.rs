use crossterm::{cursor, style::Print, terminal, ExecutableCommand, QueueableCommand};
use std::io::{self, Error};

use super::{GameState, FIELD_HEIGHT, FIELD_WIDTH};

pub fn draw_field(mut stdout: &io::Stdout) -> Result<(), Error> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    for y in 0..=FIELD_HEIGHT {
        for x in 0..=FIELD_WIDTH {
            if (y == FIELD_HEIGHT) || (x == FIELD_WIDTH) {
                // in this loop we are more efficient by not flushing the buffer.
                stdout.queue(cursor::MoveTo(x * 2, y))?;
                stdout.queue(Print("##"))?;
            }
        }
    }
    Ok(())
}

pub fn draw_snake(mut stdout: &io::Stdout, gs: &GameState) -> Result<(), Error> {
    for node in &gs.snake_nodes {
        stdout.queue(cursor::MoveTo(
            (node.x * 2).try_into().unwrap_or(0),
            node.y.try_into().unwrap_or(0),
        ))?;
        stdout.queue(Print("[]"))?;
    }
    Ok(())
}

pub fn draw_food(mut stdout: &io::Stdout, gs: &GameState) -> Result<(), Error> {
    stdout.queue(cursor::MoveTo(
        (gs.food.x * 2).try_into().unwrap_or(0),
        (gs.food.y).try_into().unwrap_or(0),
    ))?;
    stdout.queue(Print("()"))?;
    Ok(())
}
