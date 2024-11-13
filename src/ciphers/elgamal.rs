use crate::utils::math_operations::mod_inverse;

pub struct ELGaml {
    p: usize,
    g: usize,
    priv_k: u32,
    pub_k: u32
}

impl ELGaml {
    pub fn new(p: usize, g: usize, private_key: u32) -> Self {
        Self {
            p,
            g,
            priv_k: private_key,
            pub_k: (g.pow(private_key) % p) as u32
        }
    }

    pub fn encrypt(&self, m: usize, k: u32) -> (usize, usize) {
        let ct0 = self.g.pow(k) % self.p;
        let ct1 = (m * (self.pub_k.pow(k)) as usize) % self.p;
        (ct0, ct1)
    }

    pub fn decrypt(&self, cipher_text: (usize, usize)) -> Option<usize> {
        if let Some(mod_inv_ct) = mod_inverse(cipher_text.0.pow(self.priv_k), self.p) {
            return Some((cipher_text.1 * mod_inv_ct as usize) % self.p)
        }

        None
    }
}