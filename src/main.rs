use crossterm::cursor::position;
use crossterm::{
    cursor::{MoveLeft, MoveToNextLine},
    event::{read, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal,
    terminal::{size, ScrollUp},
};
use std::io::{stdout, Write};

fn main() -> crossterm::Result<()> {
    let mut buffer = String::new();
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    'repl: loop {
        let (cols, rows) = size()?;
        let (pos_x, pos_y) = position()?;

        // Print the Prompt
        execute!(
            stdout,
            SetForegroundColor(Color::Blue),
            Print(format!("{}x{} @{}x{} > ", cols, rows, pos_x, pos_y)),
            ResetColor
        )?;

        // Read each key
        'input: loop {
            let (_cols, rows) = size()?;
            let (_pos_x, pos_y) = position()?;
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
                        if buffer == "exit" {
                            break 'repl;
                        } else {
                            let scroll_distance = if pos_y == (rows - 1) { 1 } else { 0 };
                            queue!(
                                stdout,
                                ScrollUp(scroll_distance),
                                MoveToNextLine(1),
                                Print(format!("Result: {}", buffer)),
                                ScrollUp(scroll_distance),
                                MoveToNextLine(1)
                            )?;
                            stdout.flush()?;
                            buffer.clear();
                            break 'input;
                        }
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
    }
    terminal::disable_raw_mode()?;

    println!();
    Ok(())
}
