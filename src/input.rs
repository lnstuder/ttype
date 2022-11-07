use crossterm::style::Color;

pub struct Input {
    input: char,
}

impl Input {
    pub fn new(input: char) -> Self {
        Self { input }
    }

    pub fn value(&self) -> char {
        self.input
    }

    pub fn output_color(&self, should_be: char) -> Color {
        if self.input == should_be {
            return Color::Green;
        }

        Color::Red
    }
}
