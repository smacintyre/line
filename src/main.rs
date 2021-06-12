use std::io::{stdout, Write};
use crossterm::{
    style::{
        Color,
        SetForegroundColor,
        SetBackgroundColor,
        Print,
        ResetColor
    },
    event::{
        KeyEvent,
        Event,
        read,
        KeyCode
    },
    execute,
    terminal,
    ExecutableCommand,
};

fn main() -> crossterm::Result<()> {
    let mut buffer = String::new();

    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        Print("> "),
        ResetColor
    )?;

    terminal::enable_raw_mode()?;
    loop {
        match read()? {
            Event::Key(KeyEvent { code, modifiers }) => {
                match code {
                    KeyCode::Backspace => {}
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
                        let mut char_buffer = [0; 4];
                        let bytes = c.encode_utf8(&mut char_buffer).as_bytes();
                        stdout().write_all(bytes)?;
                        stdout().flush()?;
                        buffer.push(c);
                    }
                    KeyCode::Null => {}
                    KeyCode::Esc => {}
                }
            }
            Event::Mouse(event) => println!("{:?}", event),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }
    }
    terminal::disable_raw_mode()?;

    Ok(())
}
