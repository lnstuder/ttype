use std::fs;
use std::io::{self, Write};

const WORDLIST_FILE_PATH: &str = "./wordlist.txt";

use crossterm::style::Color;
pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command, Result,
};

fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

fn get_wordlist() -> Result<Vec<String>> {
    let chars = fs::read_to_string(WORDLIST_FILE_PATH)?
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    Ok(chars)
}

fn run<W: Write>(out: &mut W) -> Result<()> {
    execute!(out, terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();

    execute!(out, style::SetBackgroundColor(Color::Black))?;

    let wordlist = get_wordlist()?;
    let mut chunks = wordlist.chunks(10);

    loop {
        let (cols, rows) = terminal::size()?;
        queue!(
            out,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(cols / 4, rows / 2 - 1)
        )?;

        let current_chunk = chunks.next();
        if current_chunk.is_none() {
            break;
        }
        let current_chunk = current_chunk.unwrap();

        execute!(out, style::Print(current_chunk.join(" ")))?;
        execute!(out, cursor::Hide, cursor::MoveTo(cols / 4, rows / 2 - 1))?;

        for chr in current_chunk.join(" ").chars() {
            if let Ok(input_char) = read_char() {
                let mut col = Color::Grey;
                if input_char == chr {
                    col = Color::Green;
                } else if input_char != chr {
                    col = Color::Red;
                }

                queue!(out, style::SetForegroundColor(col), style::Print(chr))?;
                out.flush()?;
            }
        }
    }

    execute!(out, terminal::LeaveAlternateScreen, cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();

    Ok(())
}

fn main() -> Result<()> {
    let mut stdout = io::stdout();

    if let Err(what) = run(&mut stdout) {
        println!("Error: {:?}", what);
    }

    Ok(())
}
