use ciphers::lfsr::LFSR;
use utils::bit_operations::get_bit;
// use utils::math_operations::euler_phi;
use ciphers::monoalphabetic::{shift_cipher, substitution_cipher, affine_cipher};
use ciphers::polyalphabetic::{vigenere_cipher, permutation_cipher};
use ciphers::enigma::EnigmaMachine;
use ciphers::block_ciphers::des::{ExpansionTable, PermutationTable};

pub mod ciphers;
pub mod utils;
pub mod tests;

fn main() {
    // TODO: Move this code to the tests module
    // For cipher usage this should be run as a cmd line tool

    let custom_out_fn4 = |x: u32| -> u32 {
        (get_bit(x,5) + get_bit(x, 3) * get_bit(x, 1)) & 1
    };

    let fsr4 = LFSR::new(0b110110, 6, custom_out_fn4);
    fsr4.print_period("LFSR");

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

    let plain_text = "GRAD";
    let affine_key = (11, 2);
    let cipher_text = affine_cipher(plain_text, affine_key, false);
    println!("\nAffine:\nPlain: {}\nCipher: {}", plain_text, cipher_text); 
    let plain_text = affine_cipher(&cipher_text, affine_key, true);
    println!("Decrypted: {}", plain_text);

    let plain_text = "GRAD";
    let key = "MARC";
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

    let expansion_table = ExpansionTable::default();
    let block: u32 = 0b11110000101010101111000010101010;

    let expanded_block = expansion_table.expand(block);
    println!("Expanded block: {:048b}", expanded_block);

    let perm_table = PermutationTable::new();
    let sub_block = 0b0000_1100_0010_0001_0110_1101_0101_1100;
    let permuted = perm_table.permute(sub_block);
    print_permuted(permuted);

    let plaintext: [[u8; 4]; 4] = [
        [0x00, 0x04, 0x08, 0x0C],
        [0x01, 0x05, 0x09, 0x0D],
        [0x02, 0x06, 0x0A, 0x0E],
        [0x03, 0x07, 0x0B, 0x0F],
    ];

    let key: [[u8; 4]; 4] = [
        [0x01, 0x01, 0x01, 0x01],
        [0x01, 0x01, 0x01, 0x01],
        [0x01, 0x01, 0x01, 0x01],
        [0x01, 0x01, 0x01, 0x10],
    ];

    let mut state: [[u8; 4]; 4] = [[0; 4]; 4];
    println!("\nQ4:");
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] = plaintext[i][j] ^ key[i][j];
            print!("{:02x} ", state[i][j]);
        }
        println!();
    }

    // Shift rows
    

}


fn print_permuted(permuted: u32) {
    print!("Permuted: ");
    for i in 0..32 {
        if i != 0 && i % 4 == 0 {
            print!(" ");
        }
        print!("{}", get_bit(permuted, i));
    }
    println!();
}