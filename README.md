# Shiritori-rs

Shiritori is a famous word chain game in Japan.

![](/docs/shirotori.png)

## Live demo (Wasm)

https://tnantoka.github.io/shiritori-wasm/

## Usage

```
shiritori = { git = "https://github.com/tnantoka/shiritori-rs.git" }

# for Wasm
getrandom = { version = "0.2.6", features = ["js"] }
```

```rust
use shiritori::game::{Game};
use shiritori::word::{WordList, WordListType};

let word_list = WordList::load(WordListType::Pokemon);
let mut game = Game::new(word_list);

game.current_turn(); # Word { text: "ピカチュウ", reading: "ピカチユウ" }
let judgement = game.next_turn("ウーラオス");
game.current_turn(); # Word { text: "スイクン", reading: "スクイン" }

judgement.game_over # true
judgement.winner # Player::You
judgement.reason # Reason::LastLetterIsInvalid
```

## Example

```
$ cargo run --example hello pokemon

bot> ピカチュウ(ピカチユウ)
next: ウ
you> ウーラオス
bot> スイクン(スイクン)
You win
(last letter is invalid)
```

## Development

### Update word list

```
$ cargo run --bin generate

# pretty
$ cargo run --bin generate -- -p
```

## Acknowledgments 

- https://clrd.ninjal.ac.jp/unidic/
  - unidic-cwj-3.1.0/lex_3_1.csv
- https://github.com/PokeAPI/pokeapi
  - data/v2/csv/pokemon_species_names.csv