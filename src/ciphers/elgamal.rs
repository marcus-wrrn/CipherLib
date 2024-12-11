use crate::utils::math_operations::{modpow, mod_inverse};

pub struct ELGaml {
    p: u64,
    g: u64,
    priv_k: u64,
    pub_k: u64
}

fn calc_pub_key(g: u64, p: u64, sk: u64) -> u64 {
    modpow(g, sk, p)
}

pub fn encrypt(g: u64, p: u64, pk: u64, m: u64, k: u64) -> (u64, u64) {
    let ct0 = modpow(g, k, p);
    let ct1 = modpow(pk, k, p);
    let ct1 = (ct1 * m) % p; 
    
    (ct0, ct1)
}

impl ELGaml {
    pub fn new(p: u64, g: u64, private_key: u64) -> Self {
        Self {
            p,
            g,
            priv_k: private_key,
            pub_k: calc_pub_key(g, p, private_key)
        }
    }

    pub fn encrypt(&self, m: u64, k: u64) -> (u64, u64) {
        encrypt(self.g, self.p, self.pub_k, m, k)
    }

    pub fn decrypt(&self, cipher_text: (u64, u64)) -> u64 {
        let p1 = modpow(cipher_text.0, self.priv_k, self.p);
        let p1 = mod_inverse(p1 as usize, self.p as usize).unwrap() as u64;
        (cipher_text.1 * p1) % self.p
    }
}