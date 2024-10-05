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

    mod block_ciphers {
        use crate::ciphers::block_ciphers::des::ExpansionTable;
        use crate::ciphers::block_ciphers::des;

        #[test]
        fn expansion_table() {
            let expansion_table = ExpansionTable::default();
            let block: u32 = 0b11110000101010101111000010101010;

            let expanded_block = expansion_table.expand(block);

            assert_eq!(expanded_block, 0b011110100001010101010101011110100001010101010101)
        }

        #[test]
        fn sbox1() {
            let s_box_temp: [[u8; 16]; 4] =  [
                [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
                [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
                [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
                [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13]
            ];
            let s_box = des::SBox::from(s_box_temp);

            let block = 0b011100;
            let result = s_box.substitution(block);
            let expected_result = 0b0000;
            assert_eq!(result, expected_result);
        }

        #[test]
        fn sbox2() {
            let s_box_temp: [[u8; 16]; 4] =  [
                [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10],
                [3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5],
                [0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15],
                [13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9],
            ];
            let s_box = des::SBox::from(s_box_temp);

            let block = 0b010001;
            let result = s_box.substitution(block);
            let expected_result = 0b1100;
            assert_eq!(result, expected_result);
        }

        #[test]
        fn sbox8() {
            let s_box_temp: [[u8; 16]; 4] =  [
                [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7],  // 0yyyy0
                [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],  // 0yyyy1
                [7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8],  // 1yyyy0
                [2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11],  // 1yyyy1
            ];
            let s_box = des::SBox::from(s_box_temp);

            let block = 0b110011;
            let result = s_box.substitution(block);
            let expected_result = 12;
            assert_eq!(result, expected_result);
        }

        #[test]
        fn s_boxes() {
            let sboxes = des::SBoxes::new();
            let block: u64 = 0b0111_0001_0001_0111_0011_0010_1110_0001_0101_1100_1111_0011;
            let result = sboxes.substitution(block);
            dbg!("Result: {:032b}", result);
            let expected_result = 0b0000_1100_0010_0001_0110_1101_0101_1100;
            assert_eq!(result, expected_result);
        }

        #[test]
        fn perm_box() {
            let perm_table = des::PermutationTable::new();
            let block = 0b0000_1100_0010_0001_0110_1101_0101_1100;
            let permuted = perm_table.permute(block);
            let expected_result = 0b1001_1010_0001_1100_0010_0000_1011_1100;
            dbg!("Permuted: {:032b}", permuted);
            assert_eq!(permuted, expected_result);
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

