#[cfg(test)]
mod tests {
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
        let plain_text = "hello world my name is charles, do you like bread, I like bread, fuck yeah fuck yeah fuck yeah";
        let cipher_text1 = enigma.encrypt(plain_text);
        enigma.reset();
        let cipher_text2 = enigma.encrypt(plain_text);
        assert_eq!(cipher_text1, cipher_text2);
    }
}

