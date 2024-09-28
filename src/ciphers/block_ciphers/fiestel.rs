use rand::seq::SliceRandom;

pub struct Fiestel {
    rounds: u32,
    block_size: u32,
    key: u64,
    permutaion_table: Vec<u32>,
}

impl Fiestel {
    pub fn new(rounds: u32, block_size: u32, key: u64) -> Self {
        let mut permutaion_table = Vec::new();
        for i in 0..block_size {
            permutaion_table.push(i);
        }
        let mut rng = rand::thread_rng();
        permutaion_table.shuffle(&mut rng);

        Fiestel {
            rounds,
            block_size,
            key,
            permutaion_table,
        }
    }

    fn f(&self, block: u64, key: u64) -> u64 {
        let mut block = block;
        let mut key = key;
        for i in 0..self.block_size {
            let bit = block & 1;
            block >>= 1;
            key ^= bit << self.permutaion_table[i as usize];
        }
        key
    }

    pub fn encrypt(&self, block: u64) -> u64 {
        let mut left = block >> self.block_size / 2;
        let mut right = block & ((1 << self.block_size / 2) - 1);

        for _ in 0..self.rounds {
            let new_right = left ^ self.f(right, self.key);
            left = right;
            right = new_right;
        }

        (left << self.block_size / 2) | right
    }
}
