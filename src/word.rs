use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    text: String,
    reading: String,
}

impl Word {
    pub fn new(text: String, reading: String) -> Self {
        Word { text, reading }
    }

    fn first_letter(&self) -> char {
        self.text.chars().next().unwrap()
    }

    fn last_letter(&self) -> char {
        self.text.chars().last().unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WordList {
    items: Vec<Word>,
}

impl WordList {
    pub fn new(items: Vec<Word>) -> Self {
        WordList { items }
    }

    pub fn load(word_list_type: WordListType) -> Self {
        let json = fs::read_to_string(word_list_type.as_str()).unwrap();
        serde_json::from_str(&json).unwrap()
    }
}

pub enum WordListType {
    Unidic,
    Pokemon,
}

impl WordListType {
    pub fn as_str(self) -> String {
        match self {
            WordListType::Unidic => String::from("unidic"),
            WordListType::Pokemon => String::from("pokemon"),
        }
    }
}
