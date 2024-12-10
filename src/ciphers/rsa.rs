use crate::utils::math_operations::{euler_phi, extended_gcd, binary_decomposition, find_modulus};


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
        let pow_of_two = binary_decomposition(self.pk.1 as u64);
        let result = find_modulus(pow_of_two, m as u64, self.pk.0 as u64);
        result as u32
    }

    pub fn decrypt(&self, c: u32) -> u32 {
        let pow_of_two = binary_decomposition(self.sk as u64);
        let result = find_modulus(pow_of_two, c as u64, self.pk.0 as u64);
        result as u32
    }
}

pub fn num_possible_keys(p: u32, q: u32) -> u32 {
    let e_n = (p - 1) * (q - 1);
    euler_phi(e_n)
}