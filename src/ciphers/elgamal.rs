use num_bigint::ToBigUint;
pub struct ELGaml {
    p: usize,
    g: usize,
    priv_k: usize,
    pub_k: usize
}

fn calc_pub_key(g: usize, p: usize, sk: usize) -> u32 {
    let g_big = g.to_biguint().unwrap();
    let p_big = p.to_biguint().unwrap();
    let sk_big = sk.to_biguint().unwrap();

    let x = g_big.modpow(&sk_big, &p_big);
    x.to_u32_digits()[0]
}

pub fn encrypt(g: usize, p: usize, pk: u32, m: u32, k: u32) -> (u32, u32) {
    let g_big = g.to_biguint().unwrap();
        let k_big = k.to_biguint().unwrap();
        let p_big = p.to_biguint().unwrap();
        let pk_big = pk.to_biguint().unwrap();

        let ct0 = g_big.modpow(&k_big, &p_big);
        let ct1 = (m * pk_big.pow(k)) % p_big;

        
        (ct0.to_u32_digits()[0], ct1.to_u32_digits()[0])
}

impl ELGaml {
    pub fn new(p: usize, g: usize, private_key: usize) -> Self {
        Self {
            p,
            g,
            priv_k: private_key,
            pub_k: calc_pub_key(g, p, private_key) as usize
        }
    }

    pub fn encrypt(&self, m: u32, k: u32) -> (u32, u32) {
        encrypt(self.g, self.p, self.pub_k as u32, m, k)
    }

    pub fn decrypt(&self, cipher_text: (u32, u32)) -> u32 {
        let ct0_big = cipher_text.0.to_biguint().unwrap();
        let ct1_big = cipher_text.1.to_biguint().unwrap();
        let p_big = self.p.to_biguint().unwrap();

        let mod_inv_ct = ct0_big.pow(self.priv_k as u32).modinv(&p_big).unwrap();

        let x = (ct1_big * mod_inv_ct) % self.p;
        
        x.to_u32_digits()[0]
    }
}