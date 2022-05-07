use rand::Rng;
use serde::{Deserialize, Serialize};

use super::Word;

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
            WordListType::Unidic => include_str!("../words/unidic.json").to_string(),
            WordListType::Pokemon => include_str!("../words/pokemon.json").to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_word_by_text() {
        let word_list = WordList {
            items: vec![
                Word::new("パイ".to_string(), "パイ".to_string()),
                Word::new("パン".to_string(), "パン".to_string()),
            ],
        };
        let word = word_list.word_by_text("パン").unwrap();
        assert_eq!(word.text, "パン");
    }

    #[test]
    fn it_returns_random_word() {
        let word_list = WordList {
            items: vec![
                Word::new("パイ".to_string(), "パイ".to_string()),
                Word::new("パン".to_string(), "パン".to_string()),
            ],
        };
        let word = word_list.random_word();
        assert_eq!(["パイ", "パン"].contains(&word.text.as_str()), true);
    }

    #[test]
    fn it_returns_word_by_prefix() {
        let word_list = WordList {
            items: vec![
                Word::new("パイ".to_string(), "パイ".to_string()),
                Word::new("アイ".to_string(), "アイ".to_string()),
            ],
        };
        let word = word_list.random_word_by_prefix('ア').unwrap();
        assert_eq!("アイ", &word.text);
    }
}
