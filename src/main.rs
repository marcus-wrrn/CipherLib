//use utils::math_operations::mod_inverse;

pub mod ciphers {
    pub mod block_ciphers {
        pub mod des;
    }
    pub mod rsa;
    pub mod lfsr;
    pub mod monoalphabetic;
    pub mod polyalphabetic;
    pub mod enigma;
    pub mod elgamal;
    pub mod elliptic;
}


pub mod utils {
    pub mod math_operations;
    pub mod bit_operations;
    pub mod file_operations;
    pub mod file_encryption;
}

pub mod tests;




use ciphers::rsa::RSA;

fn main() {
    let p = 29;
    let q = 11;
    let x = 17;
    let m = 21;

    let rsa = RSA::new(p, q, x);

    
    println!("PK: {:?}, SK: {}", rsa.pk, rsa.sk);

    let ct = rsa.encrypt(m);
    let pt = rsa.decrypt(ct);

    println!("CT: {}, PT: {}", ct, pt);
}