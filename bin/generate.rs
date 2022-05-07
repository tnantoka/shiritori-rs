use shiritori::word::{Word, WordList};
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

fn main() {
    generate_unidic();
    generate_pokemon();
}

fn pretty() -> bool {
    let args = std::env::args().collect::<Vec<String>>();
    args.len() > 1 && args[1] == "-p"
}

fn generate_unidic() {
    let words = load_csv("unidic-cwj-3.1.0/lex_3_1", 0, 4, "名詞", 10);
    save_json(words, "unidic");
}

fn generate_pokemon() {
    let words = load_csv(
        "pokeapi-master/data/v2/csv/pokemon_species_names",
        2,
        1,
        "1",
        2,
    );
    save_json(words, "pokemon");
}

fn load_csv(
    path: &str,
    text_index: usize,
    filter_index: usize,
    filter_value: &str,
    reading_index: usize,
) -> Vec<Word> {
    let mut words = vec![];
    let file = File::open(format!("bin/{}.csv", path)).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let record = line.unwrap();
        let fields = record.split(',').collect::<Vec<&str>>();
        let text = fields[text_index];
        let filter = fields[filter_index];
        if filter == filter_value {
            let reading = fields[reading_index];
            if reading.chars().count() > 1 && !reading.starts_with('ン') {
                let word = Word::new(text.to_string(), reading.to_string());
                words.push(word);
            }
        }
    }
    words
}

fn save_json(words: Vec<Word>, name: &str) {
    let mut file = File::create(format!("src/words/{}.json", name)).unwrap();
    let word_list = WordList::new(words);
    let json = if pretty() {
        serde_json::to_string_pretty(&word_list).unwrap()
    } else {
        serde_json::to_string(&word_list).unwrap()
    };
    let bytes = json.as_bytes();
    file.write_all(bytes).unwrap();
}
