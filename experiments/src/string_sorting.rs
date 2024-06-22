use std::cmp::Ordering;

pub fn sort(input: &String) -> (String, String) {
    // Originally did input, now just passing as param for testing purposes
    // match io::stdin().read_line(&mut input) {
    //     Ok(_) => {
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

    // Not necessary in this scenario since we're clearing the memory immediately  after asserting
    // but good to get in the habit of 
    alphabetical_sentence.shrink_to_fit();
    //     Err(_) => {}
    // }
    (alphabetical_words, alphabetical_sentence)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_works() {
        let (alpha_words, alpha_sentence) = self::sort(&mut String::from("The Quick Brown Fox jumped over the laZy Dog"));

        assert_eq!(alpha_words, String::from("ehT cikQu Bnorw Fox dejmpu eorv eht alyZ Dgo"));
        assert_eq!(alpha_sentence, String::from("aBcdDeeeeFghhijklmnoooopQrrTtuuvwxyZ"));
    }
}
