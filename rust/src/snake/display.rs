use crossterm::{cursor, style::Print, terminal, ExecutableCommand, QueueableCommand};
use std::io::{self, Error, Write};

use super::{FIELD_HEIGHT, FIELD_WIDTH};

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
    stdout.flush()?;
    Ok(())
}
