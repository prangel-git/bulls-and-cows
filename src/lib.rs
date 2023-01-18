use std::collections::{HashMap, HashSet};

const CODELENGTH: usize = 4;

type Digit = i8;
type CodeDigits = [Digit; CODELENGTH];
type IndexToIndexMap = HashMap<usize, usize>;
type IndexSet = HashSet<usize>;

pub mod utils;