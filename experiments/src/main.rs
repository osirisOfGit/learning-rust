use std::{
    cmp::Ordering,
    io::{self},
};

fn main() {
    let mut input: String = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let mut start_of_last_word = 0;
            let mut alphabetical_words = String::with_capacity(input.len());
            let mut alphabetical_sentence = String::with_capacity(input.len());

            for (_, character) in input.char_indices() {
                if character == ' ' {
                    alphabetical_words.push(character);
                    start_of_last_word = alphabetical_words.len();
                } else {
                    sort_alpha_word(&mut alphabetical_words, character, start_of_last_word);
                    sort_alpha_sentence(&mut alphabetical_sentence, character);
                }
            }
            println!("Sorted words: {alphabetical_words}");
            println!("Sorted sentence: {alphabetical_sentence}");
        }

        Err(_) => {}
    }
}

fn sort_alpha_word(alphabetical_words: &mut String, character: char, start_of_last_word: usize) {
    if alphabetical_words.len() == 0 {
        alphabetical_words.push(character);
    } else {
        for (alpha_i, alpha_c) in alphabetical_words[start_of_last_word..].char_indices() {
            if alpha_c == ' '
                || character.to_lowercase().cmp(alpha_c.to_lowercase()) == Ordering::Less
            {
                alphabetical_words.insert(start_of_last_word + alpha_i, character);
                return;
            }
        }
        alphabetical_words.push(character);

    }
}

fn sort_alpha_sentence(alphabetical_sentence: &mut String, character: char) {
    for (alpha_i, alpha_c) in alphabetical_sentence.char_indices() {
        if character.to_lowercase().cmp(alpha_c.to_lowercase()) == Ordering::Less {
            alphabetical_sentence.insert(alpha_i, character);
            return;
        }
    }

    alphabetical_sentence.push(character);
}
