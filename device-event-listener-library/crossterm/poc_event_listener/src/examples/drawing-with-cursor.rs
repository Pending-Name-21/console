use std::io::{self, Write};
use crossterm::{
    ExecutableCommand,
    cursor::MoveTo,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, MouseEvent, MouseEventKind, KeyCode},
    terminal::{self, Clear, ClearType},
    style::Print,
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    // Setting de terminal
    terminal::enable_raw_mode()?;
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(EnableMouseCapture)?;

    // Main loop to capture the events
    loop {
        // Wait for an event
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Mouse(MouseEvent {
                    kind: MouseEventKind::Moved,
                    column,
                    row,
                    ..
                }) => {
                    // Move the cursor to the position mouse and draw
                    stdout.execute(MoveTo(column, row))?;
                    stdout.execute(Print("*"))?;
                    stdout.flush()?;
                }
                Event::Key(key_event) => {
                    // End the loop if the 'Esc' key is pressed
                    if key_event.code == KeyCode::Esc {
                        break;
                    }
                }
                _ => {}
            }
        }
    }
    
    // Restore the initial configuration
    stdout.execute(DisableMouseCapture)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
