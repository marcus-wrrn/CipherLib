use crate::utils::math_operations::{euler_phi, extended_gcd};
use num_bigint::ToBigUint;


pub struct RSA {
    pub pk: (u32, u32),
    pub sk: u32,
}

impl RSA {
    pub fn new(p: u32, q: u32, e: u32) -> Self {
        let n = p * q;
        let e_n = (p - 1) * (q - 1);
        
        // validate e
        let (gcd, mut a, _) = extended_gcd(e, e_n);
        assert_eq!(gcd, 1);

        if a <= 0 {
            a += e_n as i32;
        }
        
        Self {
            pk: (n, e),
            sk: a as u32
        }
    }

    pub fn encrypt(&self, m: u32) -> u32 {
        let m_big = m.to_biguint().unwrap();
        let e_big = self.pk.0.to_biguint().unwrap();
        let n_big = self.pk.1.to_biguint().unwrap();

        // Modular exponentiation to calculate (m^n) % e
        let x = m_big.modpow(&n_big, &e_big);
    
        // Convert result to u32, if it fits
        let dat = x.to_u32_digits();

        dat[0]
    }

    pub fn decrypt(&self, c: u32) -> u32 {
        let c_big = c.to_biguint().unwrap();
        let sk_big = self.sk.to_biguint().unwrap();
        let n_big = self.pk.0.to_biguint().unwrap();

        let x = c_big.modpow(&sk_big, &n_big);
        x.to_u32_digits()[0]
    }
}

pub fn num_possible_keys(p: u32, q: u32) -> u32 {
    let e_n = (p - 1) * (q - 1);
    euler_phi(e_n)
}