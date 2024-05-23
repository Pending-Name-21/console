//! Demonstrates how to set color for text and background example.
//!
//! cargo run --bin colors
//! 

use std::io::{self, Write};
use crossterm::{
    ExecutableCommand,
    QueueableCommand,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    // Clean terminal
    stdout.execute(Clear(ClearType::All))?;

    // Establecer el color de primer plano (texto)
    // Set color of first plane (cyan text)
    stdout.queue(SetForegroundColor(Color::Cyan))?;
    stdout.queue(Print("Text on color Cyan\n"))?;

    // Set background color
    stdout.queue(SetBackgroundColor(Color::Yellow))?;
    stdout.queue(Print("Text with yellow background color\n"))?;

    // Change first plane text (red) and background color (white)
    stdout.queue(SetForegroundColor(Color::Red))?;
    stdout.queue(SetBackgroundColor(Color::White))?;
    stdout.queue(Print("Red text with white background color\n"))?;

    // Reset colors to default status
    stdout.queue(ResetColor)?;
    stdout.queue(Print("Colors restore\n"))?;

    // Apply changes to the queue
    stdout.flush()?;

    Ok(())
}
