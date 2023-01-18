use std::collections::{HashSet};

use crate::{CodeDigits, CODEMAX};
use crate::utils::codeword_to_codedigits;

pub type MindState = HashSet<CodeDigits>;

pub fn new() -> MindState {
    HashSet::from_iter(
        (0..CODEMAX).map(|x| codeword_to_codedigits(x))
    )
}    


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn new_returns_a_valid_mindstate() {
        assert_eq!(new().len(), CODEMAX as usize)
    }
}