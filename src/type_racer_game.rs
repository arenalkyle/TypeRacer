use std::io;
use std::time::Instant;

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
    "Improving your words per minute, typing speed, and accuracy can greatly enhance your productivity and communication skills."
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
            sentence: CHOICES[0].to_string(),
            input: String::new(),
            started_at: None,
            finished_at: None,
        }
    }

    pub fn start(&mut self) {
        if self.started_at.is_none() {
            self.started_at = Some(Instant::now());
            self.finished_at = None;
            self.input.clear();
        }
    }

    pub fn stop(&mut self) {
        self.started_at = None;
        self.finished_at = Some(Instant::now());
        self.input.clear();
    }

    pub fn is_started(&self) -> bool {
        self.started_at.is_some()
    }

    pub fn sentence(&self) -> &str {
        &self.sentence
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn set_input(&mut self, input: String) {
        self.input = input;

        if self.input == self.sentence && self.finished_at.is_none() && self.started_at.is_some() {
            self.finished_at = Some(Instant::now());
        }
    }

    // wpm

}

pub fn play_game(sentence: String) {
    let mut user_input = String::new();
    let mut current_char = sentence.chars().nth(0).unwrap();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");

}

fn calculate_correct_string(string: String) -> usize {
    string.len()
}

fn calculate_wpm(input: String, sentence: String) -> String {
    sentence
}