use std::fs::File;
use std::io::{BufReader, BufRead, Write};

fn main() {
    let dictionaries = vec!["words.txt", "words_alpha.txt"];

    let base_path = std::env::current_dir().unwrap();

    let mut newly_formed: Vec<String> = vec![];
    for file_name in dictionaries {
        let words = base_path.join(file_name);
        let file = File::open(words).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(word) = line {
                let word_up = word.to_uppercase();
                if !newly_formed.contains(&word_up) {
                    newly_formed.push(word_up.clone());
                }
            }
        }
    }

    newly_formed.sort();

    for word in newly_formed {
        let mut file = std::fs::OpenOptions::new().append(true).open(base_path.join("all_words.txt")).unwrap();
        // let ff = file.write(word.as_bytes());
        let ff = file.write(format!("{word}\n" ).as_bytes());
    }
}