#[cfg(test)]
mod tests {
    use crate::ciphers::block_ciphers::des;
    use crate::utils::scripts::initialize_substitution_blocks;

    mod lfsr {
        use crate::ciphers::lfsr::LFSR;
        use crate::utils::bit_operations::get_bit;

        #[test]
        fn lfsr1() {
            let custom_out_fn = |state: u32| -> u32 {
                (get_bit(state, 1) + get_bit(state, 0)) + get_bit(state, 3) & 1
            };
        
            let fsr = LFSR::new(0b11001, 5, custom_out_fn);

            assert_eq!(fsr.period, 15);
            assert_eq!(fsr.out_seq, 0b10011011100001010011);
        }

        #[test]
        fn lfsr2() {
            let custom_out_fn2 = |x: u32| -> u32 {
                let val = !(get_bit(x, 3) * get_bit(x, 2) * get_bit(x, 1) + get_bit(x, 1) + get_bit(x, 0));
                val & 1
            };
        
            let fsr2 = LFSR::new(0b1011, 4, custom_out_fn2);

            assert_eq!(fsr2.period, 16);
            assert_eq!(fsr2.out_seq, 0b11011001010000111101);
        }

        #[test]
        fn lfsr3() {
            let custom_out_fn3 = |x: u32| -> u32 {
                (get_bit(x, 0) + get_bit(x, 3) + get_bit(x, 5)) & 1
            };

            let fsr3 = LFSR::new(0b010011, 6, custom_out_fn3);
            assert_eq!(fsr3.period, 7);
            assert_eq!(fsr3.out_seq, 0b1100101110010);
        }

        #[test]
        fn non_linearfsr() {
            let custom_out_fn4 = |x: u32| -> u32 {
                (get_bit(x,5) + get_bit(x, 3) * get_bit(x, 1)) & 1
            };
        
            let fsr4 = LFSR::new(0b110110, 6, custom_out_fn4);
            assert_eq!(fsr4.period, 9);
            assert_eq!(fsr4.out_seq, 0b11011100111110100111);
        }
    }
    
    mod enigma {
        use crate::ciphers::enigma::EnigmaMachine;
        #[test]
        fn enigma_1rotor () {
            let mut enigma = EnigmaMachine::new(1, 26);
            let plain_text = "hello world my name is charles, do you like bread, I like bread, fuck yeah fuck yeah fuck yeah";
            let cipher_text = enigma.encrypt(plain_text);
            enigma.reset();
            let decrypted_text = enigma.decrypt(&cipher_text);
            assert_eq!(plain_text, decrypted_text);
        }
    
        #[test]
        fn enigma_4rotor () {
            let mut enigma = EnigmaMachine::new(4, 26);
            let plain_text = "hello";
            let cipher_text = enigma.encrypt(plain_text);
            enigma.reset();
            let decrypted_text = enigma.decrypt(&cipher_text);
            assert_eq!(plain_text, decrypted_text);
        }
        
        #[test]
        fn enigma_reset() {
            let mut enigma = EnigmaMachine::new(4, 26);
            let plain_text = "hello world my name is charles, this is a random long stream of text used to test the enigma machine using multiple rotors.";
            let cipher_text1 = enigma.encrypt(plain_text);
            enigma.reset();
            let cipher_text2 = enigma.encrypt(plain_text);
            assert_eq!(cipher_text1, cipher_text2);
        }
    }

    #[test]
    fn sdes_save() {
        let filename = String::from("subbox.bin");
        assert!(initialize_substitution_blocks(&filename));
    }

    #[test]
    fn sdes_new() {
        let subbox1 = des::SBox::new();
        let subbox2 = des::SBox::new();

        assert_ne!(subbox1.value(), subbox2.value());
    }
}

