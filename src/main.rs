use std::io;

fn is_vowel(letter: char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    vowels.contains(&letter)
}

fn main() {
    println!("Welcome to the Pig Latin converter!");

    let exit_phrase = "quit";

    loop {
        println!("Please enter a phrase to convert.");

        let mut phrase = String::new();

        io::stdin()
            .read_line(&mut phrase)
            .expect("Failed to read line");

        if phrase.trim() == exit_phrase {
            break;
        }

        let words = phrase.split_whitespace();
        for word in words {
            let mut chars = word.chars();

            if let Some(first_char) = chars.next() {
                let suffix = if is_vowel(first_char) {
                    String::from("-hay")
                } else {
                    format!("-{}ay", first_char)
                };

                let mut pig_latin_word = chars.as_str().to_string();
                pig_latin_word.push_str(&suffix);
                print!("{pig_latin_word} ");
            }
        }
        println!("");
    }
    println!("Thanks for using the Pig Latin converter!");
}
