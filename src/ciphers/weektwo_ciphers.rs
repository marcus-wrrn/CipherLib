fn create_byte_list(keyword: &str) -> Vec<u8> {
    let mut byte_list = Vec::new(); // Create a new vector to hold the byte values

    for ch in keyword.chars() {
        if !ch.is_alphabetic() {
            continue;
        }
        let base = if ch.is_ascii_lowercase() { b'a' } else { b'A' };
        let value = ch as u8 - base;
        byte_list.push(value); // Push the computed value into the vector
    }

    byte_list // Return the vector
}

pub fn vigenere_cipher(plain_text: &str, keyword: &str) -> String {
    let keylist = create_byte_list(keyword);
    let mut key_ptr: usize = 0;
    let mut cipher_text = String::new();
    
    for ch in plain_text.chars() {
        if key_ptr >= keylist.len() {
            key_ptr = 0;
        }

        if ch.is_alphabetic() {
            let key_index = keylist[key_ptr];

            let base = if ch.is_ascii_lowercase() { b'a' } else { b'A' };
            let value = ((ch as u8 - base) + key_index) % 26;
            cipher_text.push((value + base) as char );
            key_ptr += 1;
        } else {
            cipher_text.push(ch);
        }
    }

    cipher_text
}


fn perm_text_creation(text: &str, key: &[usize]) -> Vec<char> {
    let txt_len = text.len();
    let k_len = key.len();
    let cipher_text_size = txt_len + (k_len - (txt_len % k_len));
    let cipher_text = vec![' '; cipher_text_size]; // Initialize with spaces
    cipher_text
}

pub fn permutation_cipher(plain_text: &str, key: &[usize]) -> String {
    let mut cipher_text = perm_text_creation(plain_text, key);

    println!("Length of cipher text: {}", cipher_text.len());
    // Encrypt the plain text using the permutation key
    let mut key_ptr = 0;
    let mut delimeter = 0;
    for ch in plain_text.chars() {
        if key_ptr >= key.len() {
            key_ptr = 0;
            delimeter += key.len();
        }
        // Ensure the index is valid for the key
        cipher_text[key[key_ptr] - 1 + delimeter] = ch; // Permute the character according to the key
        key_ptr += 1;
    }

    cipher_text.iter().collect() // Convert vector of chars back to String
}

pub fn reverse_permutation_cipher(cipher_text: &str, key: &[usize]) -> String {
    let mut plain_text = perm_text_creation(cipher_text, key);

    let mut key_ptr = 0;
    let mut delimeter = 0;
    for (i, ch) in cipher_text.chars().enumerate() {
        if key_ptr >= key.len() {
            key_ptr = 0;
            delimeter += key.len();
        }

        if let Some(index) = key.iter().position(|&x| x == i - delimeter + 1) {
            plain_text[index + delimeter] = ch;
        } else {
            println!("Fuck that isn't supposed to happen!");
        }

        key_ptr += 1;
    }

    plain_text.iter().collect()
}