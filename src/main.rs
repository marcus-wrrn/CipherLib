use ciphers::lfsr::LFSR;
use utils::bit_operations::get_bit;
// use utils::math_operations::euler_phi;
use ciphers::monoalphabetic::{shift_cipher, substitution_cipher, affine_cipher};
use ciphers::weektwo_ciphers::{vigenere_cipher, permutation_cipher};
use ciphers::enigma::EnigmaMachine;

pub mod ciphers;
pub mod utils;
pub mod tests;

// fn calc_affine_keys(m: u32) -> u32 {
//     euler_phi(m) * m
// }

fn main() {
    let custom_out_fn = |state: u32| -> u32 {
        (get_bit(state, 1) + get_bit(state, 0)) + get_bit(state, 3) & 1
    };

    let custom_out_fn2 = |x: u32| -> u32 {
        let val = !(get_bit(x, 3) * get_bit(x, 2) * get_bit(x, 1) + get_bit(x, 1) + get_bit(x, 0));
        val & 1
    };

    let custom_out_fn3 = |x: u32| -> u32 {
        (get_bit(x, 0) + get_bit(x, 3) + get_bit(x, 5)) & 1
    };


    // Create the LFSR ciphers
    let fsr = LFSR::new(0b11001, 5, custom_out_fn);
    fsr.print_period("FSR1");
    
    let fsr2 = LFSR::new(0b1011, 4, custom_out_fn2);
    fsr2.print_period("FSR2");

    let fsr3 = LFSR::new(0b010011, 6, custom_out_fn3);
    fsr3.print_period("LFSR3");


    // Calculate the number of keys
    // println!("Number of keys for m = {}: {}", 17, calc_affine_keys(17));
    // println!("Number of keys for m = {}: {}", 20, calc_affine_keys(20));

    let plain_text = "Wrgdb lv Wkxuvgdb";
    let k = 3;
    let cipher_text = shift_cipher(plain_text, -k);
    println!("Original: {}\nCeaser Cipher: {}", plain_text, cipher_text);

    let final_text = shift_cipher(&cipher_text, k);
    println!("Final: {}", final_text);

    let substitution_key = "dlryvohezxwptbgfjqnmuskaci";
    let plain_text = "MGZVYZLGHCMHJMYXSSFMNHAHYCDLMHA";
    let cipher_text = substitution_cipher(&plain_text, substitution_key.as_bytes());
    println!("\nSubstitution:\nPlain: {}\nCipher: {}", plain_text, cipher_text); 

    let plain_text = "wewillmeetatmidnight";
    let cipher_text = affine_cipher(plain_text, (3, 0), false);
    println!("\nAffine:\nPlain: {}\nCipher: {}", plain_text, cipher_text); 
    let plain_text = affine_cipher(&cipher_text, (3, 0), true);
    println!("Decrypted: {}", plain_text);

    let plain_text = "SECURITY";
    let key = "QUEEN";
    let cipher_text = vigenere_cipher(plain_text, key);
    println!("\nVigenere:\nPlain: {}\nCipher: {}", plain_text, cipher_text); 

    let plain_text = "shesellsseashellsbytheseashore";
    let key: &[usize; 6] = &[3, 6, 1, 5, 2, 4];
    let cipher_text = permutation_cipher(plain_text, key, false);

    println!("\nPermutation:\nPlain: {}\nCipher: {}", plain_text, cipher_text); 

    let plain_text = permutation_cipher(&cipher_text, key, true);
    println!("Decrypted: {}", plain_text);

    let plain_text = "hello";//"helloworldmynameisjoe";
    let mut enigma = EnigmaMachine::new(4, 26);
    let cipher_text = enigma.encrypt(plain_text);
    println!("\nEnigma:\nPlain: {}\nCipher: {}", plain_text, cipher_text);
    enigma.reset();
    let plain_text = enigma.decrypt(&cipher_text);
    println!("Decrypted: {}", plain_text);

}