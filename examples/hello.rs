use shiritori::game::{Game, Player};
use shiritori::word::{WordList, WordListType};

use std::env;
use std::io::stdin;
use std::io::Write;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let list_type = if args.len() > 1 && args[1] == "unidic" {
        WordListType::Unidic
    } else {
        WordListType::Pokemon
    };
    let word_list = WordList::load(list_type);
    let mut game = Game::new(word_list);

    print_next(&game);
    loop {
        let text = input();
        if text.is_empty() {
            println!("exit");
            break;
        }

        let judgement = game.next_turn(&text);
        if judgement.game_over {
            if let Some(Player::You) = judgement.winner {
                println!("bot> {}", game.current_word().text);
            }
            println!("{} win", judgement.winner.unwrap().as_str());
            println!("({})", judgement.reason.unwrap().as_str());
            break;
        } else {
            print_next(&game);
        }
    }
}

fn input() -> String {
    let mut line = String::new();
    print!("you> ");
    std::io::stdout().flush().unwrap();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn print_next(game: &Game) {
    let current_word = game.current_word();
    println!("bot> {}({})", current_word.text, current_word.reading);
    println!("next: {}", current_word.last_letter());
}
