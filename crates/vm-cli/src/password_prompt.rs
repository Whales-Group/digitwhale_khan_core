use std::io::{Write, stdout};

use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub fn prompt_password(label: &str) -> Result<String> {
    print!("{label}");
    stdout().flush()?;
    enable_raw_mode()?;

    let mut buf = String::new();
    loop {
        if let Event::Key(key) = read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Enter => break,
                KeyCode::Char(c) => {
                    buf.push(c);
                    print!("_");
                    stdout().flush()?;
                }
                KeyCode::Backspace => {
                    if !buf.is_empty() {
                        buf.pop();
                        print!("\u{8} \u{8}");
                        stdout().flush()?;
                    }
                }
                _ => {}
            }
        }
    }
    disable_raw_mode()?;
    println!();
    Ok(buf)
}
