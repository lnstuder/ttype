use crate::ui::Section;
use crossterm::{
    event::{Event, KeyCode},
    style::Color,
};
use std::{collections::VecDeque, fs::read_to_string};

const WORDLIST_PATH: &str = "./wordlist.txt";

pub struct Prompt {
    pub section: Section,
    wordlist: VecDeque<Vec<String>>,
    prompt: String,
}

impl Default for Prompt {
    fn default() -> Self {
        let wordlist = read_to_string(WORDLIST_PATH).unwrap();

        let words: VecDeque<Vec<String>> = wordlist
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .chunks(10)
            .map(|v| v.to_vec())
            .collect();

        Prompt {
            section: Section::default(),
            wordlist: words,
            prompt: String::new(),
        }
    }
}

impl Prompt {
    pub fn next_lines(&mut self) {
        self.prompt = self.wordlist.pop_front().unwrap().join(" ");
    }

    pub fn prompt(&mut self, input_buffer: &Vec<KeyCode>) -> Vec<(char, Color)> {
        let mut prompt: Vec<(char, Color)> = Vec::new();

        for (idx, chr) in self.prompt.chars().enumerate() {
            let input_key = input_buffer.get(idx);

            if input_key.is_none() {
                prompt.push((chr, Color::Grey));
                continue;
            }

            let input_key = input_key.unwrap();
            match *input_key {
                KeyCode::Char(c) => {
                    prompt.push((c, if c == chr { Color::Green } else { Color::Red }));
                }
                KeyCode::Backspace => {
                    prompt.pop();
                }
                _ => (),
            };
        }

        prompt
    }

    pub fn peek(&self) -> String {
        let mut next_line = String::new();

        if let Some(peek) = self.wordlist.iter().peekable().peek() {
            next_line = peek.join(" ");
        }

        next_line
    }

    pub fn len(&self) -> usize {
        self.prompt.len()
    }
}
