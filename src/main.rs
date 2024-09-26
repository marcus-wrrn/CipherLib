use crate::ciphers::lfsr::LFSR;
use crate::utils::math_operations::euler_phi;
use crate::ciphers::symmetric_ciphers::{shift_cipher, substitution_cipher};
use crate::ciphers::weektwo_ciphers::{affine_cipher, vigenere_cipher, permutation_cipher, reverse_permutation_cipher};

pub mod ciphers;
pub mod utils;

fn get_bit(x: u32, i: u32) -> u32 {
    ((x >> i) & 1) as u32
}

// fn reverse_bits(x: u32, n: u32) -> u32 {
//     let mut rev = 0;
//     for i in 0..n {
//         rev = rev << 1;
//         rev += get_bit(x, i);
//     }
//     rev
// }

fn calc_affine_keys(m: u32) -> u32 {
    euler_phi(m) * m
}

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
    println!("FSR1:\nPeriod: {}\nOut Seq: {:b}\n", fsr.period, fsr.out_seq);
    
    let fsr2 = LFSR::new(0b1011, 4, custom_out_fn2);
    println!("FSR2:\nPeriod: {}\nOut Seq: {:b}\n", fsr2.period, fsr2.out_seq);

    let fsr3 = LFSR::new(0b010011, 6, custom_out_fn3);
    println!("FSR3:\nPeriod: {}\nOut Seq: {:b}\n", fsr3.period, fsr3.out_seq);


    // Calculate the number of keys
    println!("Number of keys for m = {}: {}", 17, calc_affine_keys(17));
    println!("Number of keys for m = {}: {}", 20, calc_affine_keys(20));

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
    let cipher_text = affine_cipher(plain_text, (3, 0));
    println!("\nAffine:\nPlain: {}\nCipher: {}", plain_text, cipher_text); 

    let plain_text = "SECURITY";
    let key = "QUEEN";
    let cipher_text = vigenere_cipher(plain_text, key);
    println!("\nVigenere:\nPlain: {}\nCipher: {}", plain_text, cipher_text); 

    let plain_text = "shesellsseashellsbytheseashore";
    let key: &[usize; 7] = &[3, 5, 1, 6, 7, 4, 2];
    let cipher_text = permutation_cipher(plain_text, key);

    println!("\nPermutation:\nPlain: {}\nCipher: {}", plain_text, cipher_text); 

    let plain_text = reverse_permutation_cipher(&cipher_text, key);
    println!("\nDecrypted: {}", plain_text);

}