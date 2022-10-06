#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use crossterm::queue;
use crossterm::style::Color;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, style,
    terminal::{self, ClearType},
    Result,
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;
use std::io::{self, Write};
use std::string::ToString;
use std::time::{Duration, Instant};

const WORDLIST_FILE_PATH: &str = "./wordlist.txt";

const TICK_RATE: Duration = Duration::from_millis(500);

fn get_wordlist() -> Result<Vec<String>> {
    let chars = fs::read_to_string(WORDLIST_FILE_PATH)?
        .split('\n')
        .map(ToString::to_string)
        .collect();

    Ok(chars)
}

// fn on_tick() -> Result<()> {
//     Ok(())
// }

fn draw_ui<W: Write>(out: &mut W, current_chunk: &String, input_buffer: &str) -> Result<()> {
    let (cols, rows) = terminal::size()?;
    let (initial_col, initial_row) = (cols / 4, rows / 2 - 1);

    execute!(
        out,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(initial_col, initial_row),
        style::ResetColor,
        style::Print(current_chunk),
        cursor::MoveTo(initial_col, initial_row),
    )?;

    let chunks_chars = current_chunk.chars().collect::<Vec<char>>();

    for (char_index, character) in input_buffer.chars().enumerate() {
        let mut char_color = Color::Green;

        let char_index_u16: u16 = {
            let idx = u16::try_from(char_index).ok();
            idx.unwrap_or_default()
        };

        if character != chunks_chars[char_index as usize] {
            char_color = Color::Red;
        }

        queue!(
            out,
            cursor::MoveTo(initial_col + char_index_u16, initial_row),
            style::SetForegroundColor(char_color),
            style::Print(chunks_chars[char_index as usize]),
        )?;
    }

    out.flush()?;

    Ok(())
}

fn setup<W: Write>(out: &mut W) -> Result<()> {
    execute!(out, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    execute!(out, style::SetBackgroundColor(Color::Black))?;

    Ok(())
}

fn clean_up<W: Write>(out: &mut W) -> Result<()> {
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
    setup(&mut stdout)?;

    let mut input_buffer = String::new();

    let wordlist = {
        let mut wordlist = get_wordlist()?;
        wordlist.shuffle(&mut thread_rng());
        wordlist
    };

    let mut word_chunks = wordlist
        .chunks(15)
        .map(|c| c.join(" "))
        .collect::<Vec<String>>();

    let mut current_chunk = word_chunks.pop().expect("No chunk of words available.");

    let mut last_tick = Instant::now();
    loop {
        draw_ui(&mut stdout, &current_chunk, &input_buffer)?;

        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Backspace => {
                        input_buffer.pop();
                    }
                    KeyCode::Char(c) => {
                        input_buffer.push(c);
                    }
                    _ => {
                        break;
                    }
                }
            }
        }

        if input_buffer.len() == current_chunk.len() {
            let next_chunk = word_chunks.pop();
            if next_chunk.is_none() {
                break;
            }
            current_chunk = next_chunk.unwrap();
            input_buffer.clear();
        }

        if last_tick.elapsed() >= TICK_RATE {
            // on_tick()?;
            last_tick = Instant::now();
        }
    }

    clean_up(&mut stdout)?;

    Ok(())
}
