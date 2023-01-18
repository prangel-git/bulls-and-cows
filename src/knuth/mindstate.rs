use crate::{CodeDigits, Clue, CODEMAX, CODELENGTH};
use crate::utils::{codeword_to_codedigits, find_bulls_and_cows};

pub struct MindState {
    possible_secrets: Vec<CodeDigits>
}

impl MindState {
    pub fn new() -> Self {
        MindState { 
            possible_secrets: Vec::from_iter(
            (0..CODEMAX).map(codeword_to_codedigits)
        )
        } 
    }  

    pub fn update_mindstate(&mut self, guess: &CodeDigits, clue: &Clue) { 
        self.possible_secrets.retain(|secret| find_bulls_and_cows(secret, guess) == *clue);  
    }

    pub fn get_guess(&self) -> CodeDigits {
        if self.possible_secrets.is_empty() {
            codeword_to_codedigits(0)
        } else if self.possible_secrets.len() == (CODEMAX as usize) {
            initial_guess()
        } else {
            self.possible_secrets[0]
        }
    }
}    

fn initial_guess() -> CodeDigits {
    let mut initial = [0; CODELENGTH];
    let mut current = 0;
    
    for i in 0..CODELENGTH {
        initial[i] = current;
        current = (current + 1) % 2;
    }

    initial
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn new_returns_a_valid_mindstate() {
        let mindstate = MindState::new();
        assert_eq!(mindstate.possible_secrets.len(), CODEMAX as usize)
    }

    #[test]
    fn guess_0123_clue_4b0c_returns_mindstate_containing_only_0123() {
        let mut mindstate = MindState::new();
        let guess = [0,1,2,3];
        let clue = Clue{bulls:4, cows:0};
        mindstate.update_mindstate(&guess, &clue);

        assert!(mindstate.possible_secrets.contains(&[0,1,2,3]));
        assert_eq!(mindstate.possible_secrets.len(), 1)   
    }

    #[test]
    fn guess_0123_clue_0b4c_returns_mindstate_containing_9elements() {
        let mut mindstate = MindState::new();
        let guess = [0,1,2,3];
        let clue = Clue{bulls:0, cows:4};
        mindstate.update_mindstate(&guess, &clue);

        assert_eq!(mindstate.possible_secrets.len(), 9)  
    }

    #[test]
    fn get_guess_returns_first_valid_guess() {
        let mut mindstate = MindState::new();
        let guess = [0,1,2,3];
        let clue = Clue{bulls:0, cows:4};
        mindstate.update_mindstate(&guess, &clue);
        
        assert_eq!(mindstate.get_guess(), [1,0,3,2])
    }

    #[test]
    fn get_guess_returns_0000_when_mindstate_is_empty() {
        let mut mindstate = MindState::new();
        mindstate.possible_secrets = Vec::new();

        assert_eq!(mindstate.get_guess(), [0,0,0,0])
    }

    
}