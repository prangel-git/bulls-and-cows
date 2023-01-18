use crate::{CodeWord, CodeDigits, CODELENGTH};


pub fn codeword_to_codedigits(word: CodeWord) -> CodeDigits {
    let mut current_word = word;
    let mut digits = [0, 0, 0, 0];

    for i in (0..CODELENGTH).rev() {
        let digit = (current_word % 10) as u8;
        digits[i] = digit;
        current_word /= 10;
    }

    return digits;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codeword_123_goes_to_codedigits_0123 () {
        let word = 123;
        let expected_digits = [0,1,2,3];

        assert_eq!(codeword_to_codedigits(word), expected_digits);
    }
    
}