use crate::utils::bit_operations::reverse_bits;

#[derive(Debug)]
pub struct LFSR<F>
where F: Fn(u32) -> u32 {
    pub state: u32,
    pub size: usize,
    pub calc_output: F,
    pub period: u32,
    pub out_seq: u32
}

impl<F> LFSR<F> 
where F: Fn(u32) -> u32 {
    pub fn new(state: u32, size: usize, calc_output: F) -> Self {
        let mut fsr = LFSR { 
            state, 
            size, 
            calc_output,
            period: 0,
            out_seq: 0
        };

        let data = fsr.calc_period();
        fsr.period = data.0;
        fsr.out_seq = data.1;
        fsr
    }

    fn step(&mut self) -> u32{
        let out_bit = (self.calc_output)(self.state);

        // Shift the state to the right by 1 bit
        self.state >>= 1;
        self.state += out_bit << (self.size - 1);
        out_bit
    }

    fn check_period(&self, prev_states: &Vec<u32>) -> i32 {
        for (i, state) in prev_states.iter().enumerate() {
            if *state == self.state {
                return i as i32;
            }
        }
        -1
    }

    fn calc_period(&mut self) -> (u32, u32) {
        let mut out_seq = reverse_bits(self.state, self.size as u32);
        let mut prev_seqs = vec![self.state];
        

        for i in 0..32 {
            let out_bit = self.step();
            
            // Shift the output sequence to the left by 1 bit and add the output bit
            out_seq = (out_seq << 1) + out_bit;

            

            let period_check = self.check_period(&prev_seqs);

            if period_check != -1 {
                return ((i + 1 - period_check) as u32, out_seq);
            }

            // Add next state to the list of previous states
            prev_seqs.push(self.state);
            
        }
        (0, out_seq)
    }

    pub fn print_period(&self, name: &str) {
        println!("{}:\nPeriod: {}\nOut Seq: {:b}\n", name, self.period, self.out_seq);
    }
}
