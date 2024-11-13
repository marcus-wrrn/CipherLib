
pub mod ciphers {
    pub mod block_ciphers {
        pub mod des;
    }
    pub mod lfsr;
    pub mod monoalphabetic;
    pub mod polyalphabetic;
    pub mod enigma;
    pub mod elgamal;
}


pub mod utils {
    pub mod math_operations;
    pub mod bit_operations;
    pub mod file_operations;
}

pub mod tests;


fn main() {
    let x = 8251;
    let y = 6105;
    let z = utils::math_operations::extended_gcd(x, y);

    println!("GCD of {} and {} is {}", z.0, z.1, z.2);
}