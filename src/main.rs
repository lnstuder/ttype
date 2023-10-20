mod prompt;

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute, queue, terminal, cursor, style::{self, Stylize}
};
use prompt::Prompt;
use std::{io::{stdout, Write}, collections::VecDeque};
use std::time::Duration;

struct App {
    dimensions: (u16, u16),
    prompt: Prompt,
    input_buffer: VecDeque<char>,
    hidden_input_buffer: VecDeque<char>,
    cursor_position: (u16, u16)
}

impl App {
    pub fn new(width: u16, height: u16) -> anyhow::Result<Self> {
        let prompt = Prompt::default();

        let input_buffer = VecDeque::new();

        let app = App {
            dimensions: (width, height),
            input_buffer,
            hidden_input_buffer: VecDeque::new(),
            prompt,
            cursor_position: (0, 0)
        };
        Ok(app)
    }

    pub fn update(&mut self, event: Event) {

        match event {
            Event::Resize(width, height) => {
                self.dimensions.0 = width;
                self.dimensions.1 = height;
            }

            Event::Key(KeyEvent { code, .. }) => {
                if let KeyCode::Char(c) = code {
                    self.input_buffer.push_back(c);

                    if cursor::position().unwrap().0 > self.cursor_position.0 + 15 {
                        self.prompt.shift_forward();
                        if let Some(invisible) = self.input_buffer.pop_front() {
                            self.hidden_input_buffer.push_back(invisible);
                        }
                    }
                }

                if let KeyCode::Backspace = code {
                    self.input_buffer.pop_back().unwrap();

                    if cursor::position().unwrap().0 > self.cursor_position.0 + 15 {
                        self.prompt.shift_back();
                        if let Some(visible) = self.hidden_input_buffer.pop_back() {
                            self.input_buffer.push_front(visible);
                        }
                    }
                }
            }
            _ => (),
        };
    }

    fn reset_cursor<T: Write>(&mut self, out: &mut T) -> anyhow::Result<()> {
        let cursor_x = self.dimensions.0 / 2u16 - self.prompt.len() / 2;
        let cursor_y = self.dimensions.1 / 2u16;

        self.cursor_position = (cursor_x, cursor_y);

        queue!(out, cursor::MoveTo(cursor_x, cursor_y))?;

        Ok(())
    }

    pub fn draw<T: Write>(&mut self, out: &mut T) -> anyhow::Result<()> {
        self.reset_cursor(out)?;

        let display_text = self.prompt.displayed_prompt();
        let displayed_chars = display_text.chars()
            .collect::<Vec<char>>();

        queue!(out, style::PrintStyledContent(display_text.grey()))?;
        self.reset_cursor(out)?;

        for (idx, chr) in self.input_buffer.iter().enumerate() {
            let correct_char = *displayed_chars.get(idx).unwrap();

            if correct_char == *chr {
                queue!(out, style::PrintStyledContent(correct_char.green()))?;
            } else {
                queue!(out, style::PrintStyledContent(correct_char.red()))?;
            }
        }

        Ok(())
    }
}

fn run() -> anyhow::Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;

    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All)
    )?;

    let (width, height) = terminal::size()?;

    let mut app = App::new(width, height)?;

    loop {
        if poll(Duration::from_millis(500))? {
            let event = read()?;
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Esc => break,
                    _ => {
                        app.update(event);
                    },
                }
            }
        }

        app.draw(&mut stdout)?;

        stdout.flush()?;
    }

    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        terminal::Clear(terminal::ClearType::All)
    )?;

    terminal::disable_raw_mode()?;

    Ok(())
}

fn main() {
    match run() {
        Ok(()) => println!("Exited successfully"),
        Err(what) => println!("{:?}", what),
    }
}
