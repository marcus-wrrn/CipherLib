use num_bigint::ToBigUint;
pub struct ELGaml {
    p: usize,
    g: usize,
    priv_k: u32,
    pub_k: u32
}

fn calc_pub_key(g: usize, p: usize, sk: u32) -> u32 {
    let g_big = g.to_biguint().unwrap();
    let p_big = p.to_biguint().unwrap();
    let sk_big = sk.to_biguint().unwrap();

    let x = g_big.modpow(&sk_big, &p_big);
    x.to_u32_digits()[0]
}

impl ELGaml {
    pub fn new(p: usize, g: usize, private_key: u32) -> Self {
        Self {
            p,
            g,
            priv_k: private_key,
            pub_k: calc_pub_key(g, p, private_key)
        }
    }

    

    pub fn encrypt(&self, m: usize, k: u32) -> (u32, u32) {
        let g_big = self.g.to_biguint().unwrap();
        let k_big = k.to_biguint().unwrap();
        let p_big = self.p.to_biguint().unwrap();
        let pk_big = self.pub_k.to_biguint().unwrap();

        let ct0 = g_big.modpow(&k_big, &p_big);
        let ct1 = (m * pk_big.pow(k)) % p_big;

        
        (ct0.to_u32_digits()[0], ct1.to_u32_digits()[0])
    }

    pub fn decrypt(&self, cipher_text: (u32, u32)) -> u32 {
        let ct0_big = cipher_text.0.to_biguint().unwrap();
        let ct1_big = cipher_text.1.to_biguint().unwrap();
        let p_big = self.p.to_biguint().unwrap();

        let mod_inv_ct = ct0_big.pow(self.priv_k).modinv(&p_big).unwrap();

        let x = (ct1_big * mod_inv_ct) % self.p;
        
        x.to_u32_digits()[0]
    }
}