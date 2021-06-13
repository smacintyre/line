use crossterm::cursor::{position, MoveRight, RestorePosition, SavePosition, MoveToColumn};
use crossterm::{
    cursor::{MoveLeft, MoveToNextLine},
    event::{read, Event, KeyCode, KeyEvent},
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal,
    terminal::{size, ScrollUp},
};
use std::io::{stdout, Stdout, Write};

fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();

    let mut buffer = String::new();

    terminal::enable_raw_mode()?;
    'repl: loop {
        let (screen_cols, screen_rows) = size()?;

        // Print the Prompt
        queue!(
            stdout,
            SetForegroundColor(Color::Blue),
            Print(format!("{}x{} > ", screen_cols, screen_rows)),
            ResetColor
        )?;

        stdout.flush()?;

        let (input_start_col, _input_start_row) = position()?;

        if !(buffer.is_empty()) {
            queue!(stdout, Print(&buffer),)?;
        }

        stdout.flush()?;

        // Read each key
        'input: loop {
            let (pos_x, _pos_y) = position()?;
            match read()? {
                Event::Key(KeyEvent { code, modifiers: _ }) => match code {
                    KeyCode::Backspace => {
                        if !buffer.is_empty() {
                            let i = (pos_x - input_start_col) as usize;
                            buffer.remove(i - 1);
                            queue!(
                                stdout,
                                SavePosition,
                                MoveToColumn(input_start_col + 1),
                                Print(&buffer),
                                Print(" "),
                                RestorePosition,
                                MoveLeft(1),
                            )?;
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
                        if pos_x > input_start_col {
                            queue!(stdout, MoveLeft(1))?;
                            stdout.flush()?;
                        }
                    }
                    KeyCode::Right => {
                        if (pos_x as usize) < (buffer.len() + (input_start_col as usize)) {
                            queue!(stdout, MoveRight(1))?;
                            stdout.flush()?;
                        }
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
                    KeyCode::Delete => {
                        let i = (pos_x - input_start_col) as usize;
                        if !buffer.is_empty() && i < buffer.len() {
                            buffer.remove(i);
                            queue!(
                                stdout,
                                SavePosition,
                                MoveToColumn(input_start_col + 1),
                                Print(&buffer),
                                Print(" "),
                                RestorePosition,
                            )?;
                            stdout.flush()?;
                        }
                    }
                    KeyCode::Insert => {}
                    KeyCode::F(_) => {}
                    KeyCode::Char(c) => {
                        let i = (pos_x - input_start_col) as usize;
                        buffer.insert(i, c);
                        queue!(
                            stdout,
                            SavePosition,
                            MoveToColumn(input_start_col + 1),
                            Print(&buffer),
                            RestorePosition,
                            MoveRight(1)
                        )?;
                        stdout.flush()?;
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
