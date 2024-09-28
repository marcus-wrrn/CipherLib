use rand::seq::SliceRandom;
use rand::thread_rng;

struct Rotor {
    initial_state: Vec<u8>,
    values: Vec<u8>,
}

impl Rotor {
    fn new(state: Vec<u8>) -> Rotor{
        let rotor = Rotor {
            initial_state: state.clone(),
            values: state,
        };

        rotor
    }

    fn rand_new(keyspace: u8) -> Rotor {
        let mut values: Vec<u8> = (0..keyspace).collect();
        let mut rng = thread_rng();
        values.shuffle(&mut rng);
        Rotor::new(values)
    }

    fn rotate(&mut self,) {
        let first_val = self.values[0];
        let value_length = self.values.len();

        for i in 0..(value_length - 1) {
            self.values[i] = self.values[i + 1];
        }

        self.values[value_length - 1] = first_val;
    }

    fn step(&mut self, character: u8, decrypt: bool) -> u8 {
        if !decrypt {
            return self.values.get(character as usize).copied().unwrap_or(character);
        } 
        self.values.iter().position(|&x| x == character).map(|pos| pos as u8).unwrap_or(character)
    }

    fn reset(&mut self) {
        self.values = self.initial_state.clone();
    }
}

pub struct EnigmaMachine {
    rotors: Vec<Rotor>,
    //reflector: Vec<u8>,
    //keyspace: u8,
}

impl EnigmaMachine {
    pub fn new(num_rotors: u8, keyspace: u8) -> EnigmaMachine {
        let mut rotors: Vec<Rotor> = Vec::new();
        for _ in 0..num_rotors {
            rotors.push(Rotor::rand_new(keyspace));
        }

        EnigmaMachine {
            rotors,
        }
    }

    pub fn reset(&mut self) {
        for rotor in self.rotors.iter_mut() {
            rotor.reset();
        }
    }

    fn rotate_rotors(&mut self) {
            for rotor in self.rotors.iter_mut() {
                rotor.rotate();
                if rotor.initial_state != rotor.values {
                    return;
            }
        }
    }

    fn run_through(&mut self, message: &str, decrypt: bool) -> String {
        let mut cipher_text = String::new();
        for ch in message.chars() {
            if !ch.is_alphabetic() {
                cipher_text.push(ch);
                continue;
            }

            let base_val = if ch.is_ascii_lowercase() { 'a' as u8 } else { 'A' as u8 };
            let mut char_val: u8 = ch as u8 - base_val;
            
            let rotor_collection: Box<dyn Iterator<Item = &mut Rotor>> = if !decrypt {
                Box::new(self.rotors.iter_mut())
            } else {
                Box::new(self.rotors.iter_mut().rev())
            };

            for rotor in rotor_collection {
                char_val = rotor.step(char_val, decrypt);
            }
            
            self.rotate_rotors();

            // char_val =  if !decrypt {
            //     self.reflector[char_val as usize]
            // } else {
            //     self.reflector.iter().position(|&x| x == char_val).map(|pos| pos as u8).unwrap_or(char_val)
            // };

            // for rotor in self.rotors.iter_mut().rev() {
            //     char_val = rotor.step(char_val, !decrypt);
            // }

            cipher_text.push(((char_val) + base_val as u8) as char);
        }
        cipher_text
    }

    pub fn encrypt(&mut self, message: &str) -> String {
        self.run_through(message, false)
    }

    pub fn decrypt(&mut self, message: &str) -> String {
        self.run_through(message, true)
    }
}