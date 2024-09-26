pub fn ceaser_cipher(plain_text: &str) -> String {
    const SHIFT_VAL: i8 = 3;
    let mut cipher_text = String::new();
    let characters = plain_text.as_bytes();
    
    for character in characters {
        if *character == b' ' {
            cipher_text.push(' ');
            continue;
        }

        let mut value: i8 = (character - 'a' as u8) as i8;
        // Shift value
        value = value - SHIFT_VAL;

        if value < 0 {
            value += 26;
        }

        let new_character = (value as u8 + 'a' as u8) as char;
        cipher_text.push(new_character);
    }
    cipher_text
}

pub fn shift_cipher(plain_text: &str, shift_val: i8) -> String {
    let mut cipher_text = String::new();

    for character in plain_text.chars() {
        if character.is_ascii_alphabetic() {
            // Determine the ASCII base ('a' or 'A')
            let base = if character.is_ascii_lowercase() { b'a' } else { b'A' };

            // Shift the character and wrap it around the alphabet
            let shifted_char = (((character as u8 - base) as i8 + shift_val) % 26 + 26) % 26;
            cipher_text.push((shifted_char as u8 + base) as char);
        } else {
            // Non-alphabetic characters are appended as is (e.g., spaces, punctuation)
            cipher_text.push(character);
        }
    }

    cipher_text
}

pub fn substitution_cipher(plain_text: &str, key: &[u8]) -> String {
    let mut cipher_text = String::new();
    if key.len() != 26 {
        return cipher_text;
    }

    for character in plain_text.chars() {
        if character.is_alphabetic() {
            let base = if character.is_ascii_lowercase() { b'a' } else { b'A' };
            let index = (character as u8 - base) as usize;
            
            cipher_text.push(key[index] as char);
        } else {
            cipher_text.push(character);
        }
    }

    cipher_text
}