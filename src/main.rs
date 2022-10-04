use crossterm::style::Color;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Result,
};
use std::fs;
use std::io::{self, Write};

const WORDLIST_FILE_PATH: &str = "./wordlist.txt";

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
    let mut chunks = wordlist.chunks(30);

    loop {
        let (cols, rows) = terminal::size()?;
        let (initial_col, initial_row) = (cols / 4, rows / 2 - 1);
        queue!(
            out,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(initial_col, initial_row)
        )?;

        let current_chunk = chunks.next();
        if current_chunk.is_none() {
            break;
        }
        // We're checking if current_chunk is null above, so unwrap is allowed here.
        let current_chunk = current_chunk.unwrap().join(" ");

        let lines = textwrap::wrap(current_chunk.as_str(), 50)
            .iter_mut()
            .map(|line| format!("{}{}", line, ' '))
            .collect::<Vec<String>>();

        for (line_idx, line) in lines.iter().enumerate() {
            execute!(
                out,
                cursor::MoveTo(initial_col, initial_row + (line_idx as u16))
            )?;
            execute!(out, style::Print(&line))?;
        }

        for (line_idx, line) in lines.iter().enumerate() {
            execute!(
                out,
                cursor::MoveTo(initial_col, initial_row + (line_idx as u16))
            )?;

            for chr in line.chars() {
                if let Ok(input_char) = read_char() {
                    let mut col = Color::Green;
                    if input_char != chr {
                        col = Color::Red;
                    }

                    execute!(out, style::SetForegroundColor(col), style::Print(chr))?;
                }
            }
        }
    }

    execute!(
        out,
        style::ResetColor,
        terminal::Clear(ClearType::All),
        terminal::LeaveAlternateScreen,
    )?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn main() -> Result<()> {
    let mut stdout = io::stdout();

    if let Err(what) = run(&mut stdout) {
        println!("Error: {:?}", what);
    }

    Ok(())
}
