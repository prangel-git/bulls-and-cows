use crate::{CodeDigits, Clue, CODEMAX};
use crate::utils::{codeword_to_codedigits, find_bulls_and_cows};

pub type MindState = Vec<CodeDigits>;

pub fn new() -> MindState {
    Vec::from_iter(
        (0..CODEMAX).map(|x| codeword_to_codedigits(x))
    )
}    

pub fn update_mindstate(guess: &CodeDigits, clue: &Clue, mindstate: &mut MindState) { 
    mindstate.retain(|secret| find_bulls_and_cows(secret, guess) == *clue);  
}

pub fn get_guess(mindstate: &MindState) -> CodeDigits {
    if mindstate.is_empty() {
        codeword_to_codedigits(0)
    } else {
        mindstate[0].clone()
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn new_returns_a_valid_mindstate() {
        assert_eq!(new().len(), CODEMAX as usize)
    }

    #[test]
    fn guess_0123_clue_4b0c_returns_mindstate_containing_only_0123() {
        let mut mindstate = new();
        let guess = [0,1,2,3];
        let clue = Clue{bulls:4, cows:0};
        update_mindstate(&guess, &clue, &mut mindstate);

        assert!(mindstate.contains(&[0,1,2,3]));
        assert_eq!(mindstate.len(), 1)   
    }

    #[test]
    fn guess_0123_clue_0b4c_returns_mindstate_containing_9elements() {
        let mut mindstate = new();
        let guess = [0,1,2,3];
        let clue = Clue{bulls:0, cows:4};
        update_mindstate(&guess, &clue, &mut mindstate);

        assert_eq!(mindstate.len(), 9)
    }

    #[test]
    fn get_guess_returns_first_valid_guess() {
        let mut mindstate = new();
        let guess = [0,1,2,3];
        let clue = Clue{bulls:0, cows:4};
        update_mindstate(&guess, &clue, &mut mindstate);

        assert_eq!(get_guess(&mindstate), [1,0,3,2])
    }

    #[test]
    fn get_guess_returns_0000_when_mindstate_is_empty() {
        let mindstate = Vec::new();

        assert_eq!(get_guess(&mindstate), [0,0,0,0])
    }

    
}