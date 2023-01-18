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
}