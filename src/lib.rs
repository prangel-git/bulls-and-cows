use std::collections::{HashMap, HashSet};

const CODELENGTH: usize = 4;

type Digit = u8;
type CodeDigits = [Digit; CODELENGTH];
type CodeWord = u16;
type IndexToIndexMap = HashMap<usize, usize>;
type IndexSet = HashSet<usize>;


#[derive(PartialEq, Eq, Debug)]
pub struct Clue {
    bulls: u8,
    cows: u8
}

pub mod utils;
pub mod knuth;