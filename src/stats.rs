use crate::ui::Section;
use std::time::Instant;

#[derive(Debug)]
pub enum EntryType {
    Entry,
    Mistake,
    CorrectedMistake,
}

pub struct Stats {
    pub section: Section,
    start_time: Instant,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            section: Section::default(),
            start_time: Instant::now(),
        }
    }
}

impl Stats {
    pub fn minutes(&self) -> u64 {
        self.start_time.elapsed().as_secs() / 60
    }
    // pub fn raw_wpm(&self) -> u64 {
    //     (self.entries / 5) / self.minutes()
    // }

    // pub fn net_wpm(&self) -> u64 {
    //     self.raw_wpm() - (self.uncorrected_mistakes / self.minutes())
    // }

    // pub fn accuracy(&self) -> u64 {
    //     (self.entries - (self.uncorrected_mistakes + self.corrected_mistakes)) / self.entries
    //         * self.corrected_mistakes
    // }
}
