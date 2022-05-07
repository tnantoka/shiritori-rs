use crate::word::{Word, WordList};
use serde::Serialize;

pub struct Game {
    pub word_list: WordList,
    pub histories: Vec<Word>,
}

impl Game {
    pub fn new(word_list: WordList) -> Self {
        let mut word = word_list.random_word();
        loop {
            if !word.last_letter_is_invalid() {
                break;
            }
            word = word_list.random_word();
        }
        Game {
            word_list,
            histories: vec![word],
        }
    }

    pub fn current_word(&self) -> Word {
        self.histories.last().unwrap().clone()
    }

    pub fn next_turn(&mut self, text: &str) -> Judgement {
        let word = self.word_list.word_by_text(text);
        let judgement = self.judge(text, word.clone(), Player::Bot);
        if let Some(ref word) = word {
            self.histories.push(word.clone());
        }
        if judgement.game_over {
            return judgement;
        }

        let word = self
            .word_list
            .random_word_by_prefix(word.unwrap().last_letter())
            .unwrap_or_else(|| Word::new(String::new(), String::new()));
        let judgement = self.judge(word.text.as_str(), Some(word.clone()), Player::You);
        self.histories.push(word);
        judgement
    }

    fn judge(&self, text: &str, word: Option<Word>, winner: Player) -> Judgement {
        match word {
            Some(word) => {
                if self.duplicated(text) {
                    return Judgement::new(true, Some(winner), Some(Reason::DuplicatedWord));
                } else if word.last_letter_is_invalid() {
                    return Judgement::new(true, Some(winner), Some(Reason::LastLetterIsInvalid));
                }
                Judgement::new(false, None, None)
            }
            None => Judgement::new(true, Some(winner), Some(Reason::NotFoundInDictionary)),
        }
    }

    fn duplicated(&self, text: &str) -> bool {
        for item in &self.histories {
            if item.text == text {
                return true;
            }
        }
        false
    }
}

#[derive(Serialize)]
pub struct Judgement {
    pub game_over: bool,
    pub winner: Option<Player>,
    pub reason: Option<Reason>,
}

impl Judgement {
    pub fn new(game_over: bool, winner: Option<Player>, reason: Option<Reason>) -> Self {
        Judgement {
            game_over,
            winner,
            reason,
        }
    }
}

#[derive(Serialize)]
pub enum Player {
    Bot,
    You,
}

impl Player {
    pub fn as_str(&self) -> &str {
        match self {
            Player::Bot => "Bot",
            Player::You => "You",
        }
    }
}

#[derive(Serialize)]
pub enum Reason {
    DuplicatedWord,
    LastLetterIsInvalid,
    NotFoundInDictionary,
}

impl Reason {
    pub fn as_str(&self) -> &str {
        match self {
            Reason::DuplicatedWord => "duplicated word",
            Reason::LastLetterIsInvalid => "last letter is invalid",
            Reason::NotFoundInDictionary => "not found in dictionary",
        }
    }
}
