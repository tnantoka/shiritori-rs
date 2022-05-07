use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Word {
    pub text: String,
    pub reading: String,
}

impl Word {
    pub fn new(text: String, reading: String) -> Self {
        let regex = Regex::new(r"ー\z").unwrap();
        let downcases = [
            'ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ヵ', 'ヶ', 'ッ', 'ャ', 'ュ', 'ョ', 'ヮ',
        ];
        let upppercases = [
            'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'ケ', 'ツ', 'ヤ', 'ユ', 'ヨ', 'ワ',
        ];
        let reading = regex
            .replace_all(reading.as_str(), "")
            .chars()
            .map(|char| {
                let index = downcases.iter().position(|&c| c == char);
                if let Some(index) = index {
                    upppercases[index].to_string()
                } else {
                    char.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("");
        Word { text, reading }
    }

    pub fn first_letter(&self) -> char {
        self.reading.chars().next().unwrap()
    }

    pub fn last_letter(&self) -> char {
        self.reading.chars().last().unwrap()
    }

    pub fn last_letter_is_invalid(&self) -> bool {
        self.reading.ends_with('ン')
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WordList {
    pub items: Vec<Word>,
}

impl WordList {
    pub fn new(items: Vec<Word>) -> Self {
        WordList { items }
    }

    pub fn load(word_list_type: WordListType) -> Self {
        let json = word_list_type.as_json();
        serde_json::from_str(&json).unwrap()
    }

    pub fn word_by_text(&self, text: &str) -> Option<Word> {
        for item in &self.items {
            if item.text == text || item.reading == text {
                return Some(item.clone());
            }
        }
        None
    }

    pub fn random_word(&self) -> Word {
        let mut rng = rand::thread_rng();
        self.items[rng.gen_range(0..self.items.len())].clone()
    }

    pub fn random_word_by_prefix(&self, prefix: char) -> Option<Word> {
        let mut rng = rand::thread_rng();

        let mut words = vec![];
        for item in &self.items {
            if item.reading.starts_with(prefix) {
                words.push(item);
            }
        }

        if words.is_empty() {
            None
        } else {
            Some(words[rng.gen_range(0..words.len())].clone())
        }
    }
}

pub enum WordListType {
    Unidic,
    Pokemon,
}

impl WordListType {
    pub fn as_json(self) -> String {
        match self {
            WordListType::Unidic => include_str!("words/unidic.json").to_string(),
            WordListType::Pokemon => include_str!("words/pokemon.json").to_string(),
        }
    }
}
