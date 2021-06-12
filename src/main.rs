use std::io::stdout;
use crossterm::{
    event::read,
    execute,
    terminal,
    style::{SetForegroundColor, Color, SetBackgroundColor, Print, ResetColor}
};
use crossterm::event::Event;

fn main() -> crossterm::Result<()> {
    terminal::enable_raw_mode()?;

    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print("Styled text here."),
        ResetColor
    )?;

    match read()? {
        Event::Key(event) => println!("{:?}", event),
        Event::Mouse(event) => println!("{:?}", event),
        Event::Resize(width, height) => println!("New size {}x{}", width, height),
    }

    terminal::disable_raw_mode()?;

    Ok(())
}
