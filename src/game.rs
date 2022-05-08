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
        let judgement = self.judge(text, self.current_word(), word.clone(), Player::Bot);
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
        let judgement = self.judge(
            word.text.as_str(),
            self.current_word(),
            Some(word.clone()),
            Player::You,
        );
        self.histories.push(word);
        judgement
    }

    fn judge(
        &self,
        text: &str,
        current_word: Word,
        next_word: Option<Word>,
        winner: Player,
    ) -> Judgement {
        match next_word {
            Some(word) => {
                if self.duplicated(text) {
                    return Judgement::new(true, Some(winner), Some(Reason::DuplicatedWord));
                } else if current_word.last_letter() != word.first_letter() {
                    return Judgement::new(true, Some(winner), Some(Reason::FirstLetterIsInvalid));
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

#[derive(Serialize, Debug, PartialEq)]
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

#[derive(Serialize, Debug, PartialEq)]
pub enum Reason {
    DuplicatedWord,
    FirstLetterIsInvalid,
    LastLetterIsInvalid,
    NotFoundInDictionary,
}

impl Reason {
    pub fn as_str(&self) -> &str {
        match self {
            Reason::DuplicatedWord => "duplicated word",
            Reason::FirstLetterIsInvalid => "first letter is invalid",
            Reason::LastLetterIsInvalid => "last letter is invalid",
            Reason::NotFoundInDictionary => "not found in dictionary",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs() {
        let word_list = WordList {
            items: vec![
                Word::new("パイ".to_string(), "パイ".to_string()),
                Word::new("パン".to_string(), "パン".to_string()),
            ],
        };
        let game = Game::new(word_list);
        assert_eq!(
            ["パイ", "パン"].contains(&game.current_word().text.as_str()),
            true
        );
    }

    #[test]
    fn it_goes_next_turn() {
        let word_list = WordList {
            items: vec![Word::new("カイ".to_string(), "カイ".to_string())],
        };
        let mut game = Game::new(word_list);
        game.word_list = WordList {
            items: vec![
                Word::new("カイ".to_string(), "カイ".to_string()),
                Word::new("イカ".to_string(), "イカ".to_string()),
            ],
        };
        game.histories = vec![Word::new("パイ".to_string(), "パイ".to_string())];
        let judgement = game.next_turn("イカ");
        assert_eq!("カイ", game.current_word().text.as_str());
        assert_eq!(judgement.game_over, false);

        let judgement = game.next_turn("イカ");
        assert_eq!(judgement.game_over, true);
        assert_eq!(judgement.winner, Some(Player::Bot));
        assert_eq!(judgement.reason, Some(Reason::DuplicatedWord));
    }

    #[test]
    fn it_judges_first_letter() {
        let word_list = WordList {
            items: vec![Word::new("カイ".to_string(), "カイ".to_string())],
        };
        let mut game = Game::new(word_list);
        game.word_list = WordList {
            items: vec![
                Word::new("カイ".to_string(), "カイ".to_string()),
                Word::new("タコ".to_string(), "タコ".to_string()),
            ],
        };
        let judgement = game.next_turn("タコ");
        assert_eq!(judgement.game_over, true);
        assert_eq!(judgement.winner, Some(Player::Bot));
        assert_eq!(judgement.reason, Some(Reason::FirstLetterIsInvalid));
    }
}
