use crate::utils::bit_operations::reverse_bits;

#[derive(Debug)]
pub struct LFSR<F>
where F: Fn(u32) -> u32 {
    initial_state: u32,
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
            initial_state: state, 
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

    fn calc_period(&mut self) -> (u32, u32) {
        let mut out_seq = reverse_bits(self.initial_state, self.size as u32);

        for i in 0..1000 {
            //print!("State: {:0width$b}", self.state,  width = self.size as usize);
            let out_bit = self.step();
            //println!(" Out: {}", out_bit);
            out_seq = (out_seq << 1) + out_bit;
            if self.state == self.initial_state {
                return (i + 1, out_seq);
            }
        }
        (0, out_seq)
    }
}