use crate::{CodeDigits, Clue, CODELENGTH};

use super::find_matching::find_matching;

pub fn _find_bulls_and_cows_1(secret: &CodeDigits, guess: &CodeDigits) -> Clue {
    let mut bulls = 0;
    let mut cows = 0;
    
    let matching = find_matching(secret, guess);

    for (i, j) in matching {
        if i == j {
            bulls += 1;
        } else {
            cows += 1;
        }
    }
    
    Clue {bulls, cows}
}

pub fn find_bulls_and_cows(secret: &CodeDigits, guess: &CodeDigits) -> Clue {
    let mut bulls = 0;
    let mut cows = 0;

    let mut index_secret = [0u8; 10];
    let mut index_guess = [0u8; 10];

    for i in 0..CODELENGTH {
        let secret_i = secret[i];
        let guess_i = guess[i];

        if secret_i == guess_i {
            bulls += 1;
        } else {
            index_secret[secret_i as usize] += 1;
            index_guess[guess_i as usize] += 1;
        }
    }

    for i in 0..CODELENGTH {
        cows += std::cmp::min(index_secret[i], index_guess[i]); 
    }

    Clue {bulls, cows}
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn secret_0123_guess_0123_returns_4bulls0cows() {
        let secret = [0,1,2,3];
        let guess = [0,1,2,3];

        let expected_answer = Clue {bulls:4, cows:0};
        assert_eq!(find_bulls_and_cows(&secret, &guess), expected_answer);
    }

    #[test]
    fn secret_0123_guess_3210_returns_0bulls4cows() {
        let secret = [0,1,2,3];
        let guess = [3,2,1,0];

        let expected_answer = Clue {bulls:0, cows:4};
        assert_eq!(find_bulls_and_cows(&secret, &guess), expected_answer);
    }
}