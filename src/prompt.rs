use std::collections::VecDeque;

pub struct Prompt {
    prompt_buffer: VecDeque<char>,
    pub offset: usize,
    pub displayed_length: usize,
}

impl Default for Prompt {
    fn default() -> Self {
        let wordlist = include_str!("wordlist.txt");

        let words = wordlist
            .split("\n")
            .collect::<Vec<&str>>()
            .join(" ")
            .chars()
            .collect();

        Prompt {
            prompt_buffer: words,
            offset: 0,
            displayed_length: 40,
        }
    }
}

impl Prompt {
    pub fn shift_forward(&mut self) {
        if self.offset < self.prompt_buffer.len() {
            self.offset += 1;
        }
    }

    pub fn shift_back(&mut self) {
        if self.offset > 0 {
            self.offset -= 1;
        }
    }

    pub fn displayed_prompt(&self) -> String {
        self.prompt_buffer
            .clone()
            .range(self.offset..(self.offset + self.displayed_length))
            .copied()
            .collect()
    }

    pub fn len(&self) -> u16 {
        self.displayed_prompt().len() as u16
    }
}
