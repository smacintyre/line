use crossterm::cursor::position;
use crossterm::{
    cursor::{MoveLeft, MoveToNextLine},
    event::{read, Event, KeyCode, KeyEvent},
    style::{Color, Print, ResetColor, SetForegroundColor},
    queue,
    terminal,
    terminal::{size, ScrollUp},
};
use std::io::{stdout, Stdout, Write};

fn main() -> crossterm::Result<()> {
    let mut buffer = String::new();
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    'repl: loop {
        let (cols, rows) = size()?;
        let (pos_x, pos_y) = position()?;

        // Print the Prompt
        queue!(
            stdout,
            SetForegroundColor(Color::Blue),
            Print(format!("{}x{} @{}x{} > ", cols, rows, pos_x, pos_y)),
            ResetColor
        )?;

        if !(buffer.is_empty()) {
            queue!(
                stdout,
                Print(&buffer),
            )?;
        }

        stdout.flush()?;

        // Read each key
        'input: loop {
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
                            print_message(&mut stdout, &format!("Result: {}", buffer))?;
                            buffer.clear();
                            break 'input;
                        }
                    }
                    KeyCode::Left => {
                        print_message(&mut stdout, "Left!")?;
                        break 'input;
                    }
                    KeyCode::Right => {
                        print_message(&mut stdout, "Right!")?;
                        break 'input;
                    }
                    KeyCode::Up => {
                        print_message(&mut stdout, "Up!")?;
                        break 'input;
                    }
                    KeyCode::Down => {
                        print_message(&mut stdout, "Down!")?;
                        break 'input;
                    }
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
                Event::Mouse(event) => {
                    print_message(&mut stdout, &format!("MouseEvent: {:?}", event))?;
                    break 'input;
                }
                Event::Resize(width, height) => {
                    print_message(&mut stdout, &format!("New size {}x{}", width, height))?;
                    break 'input;
                }
            }
        }
    }
    terminal::disable_raw_mode()?;

    println!();
    Ok(())
}

fn print_message(stdout: &mut Stdout, msg: &str) -> crossterm::Result<()> {
    let (_cols, rows) = size()?;
    let (_pos_x, pos_y) = position()?;
    let scroll_distance = if pos_y == (rows - 1) { 1 } else { 0 };
    queue!(
        stdout,
        ScrollUp(scroll_distance),
        MoveToNextLine(1),
        Print(msg),
        ScrollUp(scroll_distance),
        MoveToNextLine(1)
    )?;
    stdout.flush()?;
    Ok(())
}
