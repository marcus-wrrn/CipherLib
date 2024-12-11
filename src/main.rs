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
}

pub mod tests;

use ciphers::rsa::RSA;
use ciphers::elgamal::ELGaml;
use ciphers::elliptic::{EllipticCurve, Point, calc_points};

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


    println!("ELGaml:");
    let p = 71;
    let g = 7;
    let x = 3;
    let m = 30;
    let k = 2;

    let cipher = ELGaml::new(p, g, x);
    let ct = cipher.encrypt(m, k);

    let pt = cipher.decrypt(ct);

    println!("ElGaml: {}, {}, {}", p, g, x);
    println!("Original: {}, CT: {:?}, PT: {}", m, ct, pt);

    println!("\nElliptic:");
    let gen_p = Point::new(3, 10);
    let curve = EllipticCurve::new(1, 23, &gen_p);
    curve.print_group();

    println!("Order calculation");
    let points = calc_points(23, 1, 1);
    for p in points {
        println!("{:?}", p);
    }

}