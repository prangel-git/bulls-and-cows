use crate::{CodeWord, CodeDigits, CODELENGTH};


pub fn codedigits_to_codeword(digits: CodeDigits) -> CodeWord {
    let mut codeword = 0;
    for i in 0..CODELENGTH {
        codeword *= 10;
        codeword += digits[i] as u16;
    }

    return codeword;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codedigits_0123_goes_to_codeword_123 () {
        let digits = [0,1,2,3];
        let expected_word = 123;

        assert_eq!(codedigits_to_codeword(digits), expected_word);
    }
}