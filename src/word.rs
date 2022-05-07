use regex::Regex;
use serde::{Deserialize, Serialize};

pub mod word_list;
pub use word_list::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs() {
        let word = Word::new("リュー".to_string(), "リュー".to_string());
        assert_eq!(word.text, "リュー");
        assert_eq!(word.reading, "リユ");
    }

    #[test]
    fn returns_first_letter() {
        let word = Word::new("アイ".to_string(), "アイ".to_string());
        assert_eq!(word.first_letter(), 'ア');
    }

    #[test]
    fn returns_last_letter() {
        let word = Word::new("アイ".to_string(), "アイ".to_string());
        assert_eq!(word.last_letter(), 'イ');
    }

    #[test]
    fn validates() {
        let word = Word::new("パイ".to_string(), "パイ".to_string());
        assert_eq!(word.last_letter_is_invalid(), false);

        let word = Word::new("パン".to_string(), "パン".to_string());
        assert_eq!(word.last_letter_is_invalid(), true);
    }
}
