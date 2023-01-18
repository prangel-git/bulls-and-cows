use std::collections::{HashMap, HashSet};

use crate::{Clue, CodeWord, CODEMAX};

#[derive(PartialEq, Eq, Debug)]
pub struct MindState {
    pub guess_clues: HashMap<CodeWord, Clue>,
    pub consistent_secrets: HashSet<CodeWord>
}

impl MindState {
    pub fn new() -> Self {
        MindState { 
            guess_clues: HashMap::new(), 
            consistent_secrets: HashSet::from_iter(0..CODEMAX) 
         }
    }    
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn new_returns_a_valid_mindstate() {
        let expected_mindstate = MindState { 
            guess_clues: HashMap::new(), 
            consistent_secrets: HashSet::from_iter(0..CODEMAX) 
         };
        
         assert_eq!(MindState::new(), expected_mindstate)
         
    }
}