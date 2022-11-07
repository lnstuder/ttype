use crate::input::Input;
use crate::prompt::Prompt;
use crate::stats::Stats;
use crossterm::queue;
use crossterm::{
    cursor,
    style::{self, Stylize},
};
use std::io::Write;

#[derive(Default)]
pub struct Section {
    pub x: u16,
    pub y: u16,
}

impl Stats {
    pub fn draw<T: Write>(&self, out: &mut T) -> anyhow::Result<()> {
        let (pos_x, pos_y) = (self.section.x, self.section.y);
        let stats = format!("{:?}", self.entries);

        queue!(
            out,
            cursor::MoveTo(pos_x, pos_y),
            style::Print("Statistics:"),
            cursor::MoveToNextLine(1),
            style::Print(stats)
        )?;

        Ok(())
    }
}

impl Prompt {
    pub fn draw<T: Write>(&mut self, out: &mut T, input_buffer: &Vec<Input>) -> anyhow::Result<()> {
        let (pos_x, pos_y) = (self.section.x, self.section.y);
        let prompt = self.prompt(input_buffer);

        queue!(out, cursor::MoveTo(pos_x, pos_y))?;
        for (chr, color) in prompt {
            let chr = chr.with(color);
            queue!(out, style::PrintStyledContent(chr))?;
        }

        let next_line = self.peek().with(style::Color::Grey);
        queue!(
            out,
            cursor::MoveTo(pos_x, pos_y + 1),
            style::PrintStyledContent(next_line)
        )?;
        queue!(out, cursor::MoveTo(pos_x, pos_y))?;

        Ok(())
    }
}
