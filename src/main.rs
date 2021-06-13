use crossterm::cursor::MoveLeft;
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal,
};
use std::io::{stdout, Write};

fn main() -> crossterm::Result<()> {
    let mut buffer = String::new();
    let mut stdout = stdout();

    execute!(
        stdout,
        SetForegroundColor(Color::Blue),
        Print("> "),
        ResetColor
    )?;

    terminal::enable_raw_mode()?;
    loop {
        match read()? {
            Event::Key(KeyEvent { code, modifiers: _ }) => match code {
                KeyCode::Backspace => {
                    if !buffer.is_empty() {
                        buffer.pop();
                        queue!(stdout, MoveLeft(1), Print(" "), MoveLeft(1))?;
                        stdout.flush()?;
                    }
                }
                KeyCode::Enter => {
                    println!("{}", buffer);
                    break;
                }
                KeyCode::Left => {}
                KeyCode::Right => {}
                KeyCode::Up => {}
                KeyCode::Down => {}
                KeyCode::Home => {}
                KeyCode::End => {}
                KeyCode::PageUp => {}
                KeyCode::PageDown => {}
                KeyCode::Tab => {}
                KeyCode::BackTab => {}
                KeyCode::Delete => {}
                KeyCode::Insert => {}
                KeyCode::F(_) => {}
                KeyCode::Char(c) => {
                    queue!(stdout, Print(c))?;
                    stdout.flush()?;
                    buffer.push(c);
                }
                KeyCode::Null => {}
                KeyCode::Esc => {}
            },
            Event::Mouse(event) => execute!(stdout, Print(format!("{:?}", event)))?,
            Event::Resize(width, height) => {
                execute!(stdout, Print(format!("New size {}x{}", width, height)))?
            }
        }
    }
    terminal::disable_raw_mode()?;

    Ok(())
}
