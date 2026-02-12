use std::io;
use std::time::{Duration, Instant};

use rand::prelude::IndexedRandom;

const CHOICES: [&'static str; 10] = [
    "Traveling allows you to witness a vast diversity of cultures that exist across our small blue planet.",
    "Homework allows for the student to reinforce their understanding of the material and develop critical thinking skills.",
    "Software development is a complex and rewarding field that requires creativity, problem solving, and collaboration.",
    "Tarleton is a public university known for its strong commitment to student success and academic excellence.",
    "Texas history is a rich and diverse subject that encompasses unique culture, politics, and geography.",
    "Life is precious and fleeting and we should cherish every moment we have.",
    "Consistency is often the silent ingredient that turns a simple spark of interest into a lifetime of expertise.",
    "A mixture of adrenaline and anxiety flooded his system; however, he remained focused on the task at hand.",
    "Completing tasks with dedication and perseverance is key to achieving success.",
    "Improving your words per minute, typing speed, and accuracy can greatly enhance your productivity and communication skills.",
];

pub struct TypeRacerGame {
    sentence: String,
    input: String,
    started_at: Option<Instant>,
    finished_at: Option<Instant>,
}

impl TypeRacerGame {
    pub fn new() -> Self {
        Self {
            sentence: "Press the start button to begin!".to_string(),
            input: String::new(),
            started_at: None,
            finished_at: None,
        }
    }

    pub fn start(&mut self) {
        if !self.is_running() {
            let mut rng = rand::rng();
            if let Some(choice) = CHOICES.choose(&mut rng) {
                self.sentence = (*choice).to_string();
            }

            self.started_at = Some(Instant::now());
            self.finished_at = None;
            self.input.clear();
        }
    }

    pub fn stop(&mut self) {
        // Keep started_at so elapsed/WPM still works after stopping.
        if self.started_at.is_some() && self.finished_at.is_none() {
            self.finished_at = Some(Instant::now());
        }
    }

    pub fn is_running(&self) -> bool {
        self.started_at.is_some() && self.finished_at.is_none()
    }

    // Backwards compatibility with your current App logic
    pub fn is_started(&self) -> bool {
        self.is_running()
    }

    pub fn sentence(&self) -> &str {
        &self.sentence
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn push_char(&mut self, c: char) {
        if !self.is_running() {
            return;
        }

        self.input.push(c);

        // Stop condition inside game state (App will also react and stop the round)
        if self.input == self.sentence {
            self.finished_at = Some(Instant::now());
        }
    }

    pub fn backspace(&mut self) {
        if !self.is_running() {
            return;
        }
        self.input.pop();
    }

    pub fn elapsed(&self) -> Option<Duration> {
        let started = self.started_at?;
        let finished = self.finished_at.unwrap_or_else(Instant::now);
        Some(finished.saturating_duration_since(started))
    }

    pub fn calculate_wpm(&self) -> Option<u32> {
        let elapsed = self.elapsed()?;
        let secs = elapsed.as_secs_f64();
        if secs <= 0.0 {
            return Some(0);
        }

        let chars = self.input.chars().count() as f64;
        let words = chars / 5.0; // typing-test convention
        let minutes = secs / 60.0;
        let wpm = (words / minutes).floor();

        Some(wpm.max(0.0) as u32)
    }
}

// (Unused helpers can stay for now)
pub fn play_game(sentence: String) {
    let mut user_input = String::new();
    let _current_char = sentence.chars().nth(0).unwrap();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
}

fn calculate_correct_string(string: String) -> usize {
    string.len()
}

fn calculate_wpm(input: String, sentence: String) -> String {
    let _ = input;
    sentence
}