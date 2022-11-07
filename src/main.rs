mod input;
mod prompt;
mod stats;
mod ui;

use crate::stats::Stats;
use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute, queue, terminal,
};
use input::Input;
use prompt::Prompt;
use stats::EntryType;
use std::io::{stdout, Write};
use std::time::Duration;

struct App {
    dimenions: (u16, u16),
    stats: Stats,
    prompt: Prompt,
    input_buffer: Vec<Input>,
    last_event: Option<Event>,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let (width, height) = terminal::size()?;

        let stats = Stats::default();
        let mut prompt = Prompt::default();
        prompt.next_lines();

        let input_buffer = Vec::new();

        let app = App {
            dimenions: (width, height),
            stats,
            prompt,
            input_buffer,
            last_event: None,
        };
        Ok(app)
    }

    pub fn update(&mut self, event: Event) {
        match event {
            Event::Resize(width, height) => {
                self.dimenions.0 = width;
                self.dimenions.1 = height;
            }
            Event::Key(KeyEvent { code, .. }) => {
                if let KeyCode::Char(c) = code {
                    let input = Input::new(c);
                    self.input_buffer.push(input);

                    if let Some(Event::Key(KeyEvent { code, .. })) = self.last_event {
                        match code {
                            KeyCode::Char(_) => {
                                self.stats.entries.push(EntryType::Entry);
                                if !self.prompt.input_correct(&self.input_buffer) {
                                    self.stats.entries.push(EntryType::Mistake)
                                }
                            }
                            KeyCode::Backspace => {
                                self.stats.entries.push(EntryType::CorrectedMistake)
                            }
                            _ => (),
                        }
                    }
                } else if let KeyCode::Backspace = code {
                    self.input_buffer.pop();
                }
            }
            _ => (),
        };

        self.last_event = Some(event.clone());
    }

    pub fn view<T: Write>(&mut self, out: &mut T) -> anyhow::Result<()> {
        let (width, height) = terminal::size()?;
        self.prompt.section.x = width / 2 - self.prompt.len() as u16 / 2;
        self.prompt.section.y = height / 2;

        self.stats.draw(out)?;
        self.prompt.draw(out, &self.input_buffer)?;

        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {}
}

fn run() -> anyhow::Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;

    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All)
    )?;

    let mut app = App::new()?;
    app.view(&mut stdout)?;
    stdout.flush()?;

    loop {
        queue!(stdout, terminal::Clear(terminal::ClearType::All))?;

        if poll(Duration::from_millis(250))? {
            let event = read()?;
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Esc => break,
                    _ => app.update(event),
                }
            }
        }

        if app.input_buffer.len() > app.prompt.len() {
            app.input_buffer.clear();
            app.prompt.next_lines()
        }

        app.view(&mut stdout)?;

        let cursor_pos_x = app.prompt.section.x + (app.input_buffer.len()) as u16;
        let cursor_pos_y = app.prompt.section.y;
        queue!(stdout, cursor::MoveTo(cursor_pos_x, cursor_pos_y))?;

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
