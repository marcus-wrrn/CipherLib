use ciphers::elliptic::{EllipticCurve, Point, generate_group};

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
}

pub mod tests;


fn main() {
    let curve = EllipticCurve::new(1, 23);
    let generator_point = Point{x: 3, y: 10};

    let group = generate_group(curve, generator_point);

    for i in 0..group.len() {
        println!("{}P: ({}, {})", i + 1, group[i].x, group[i].y);
    }
}