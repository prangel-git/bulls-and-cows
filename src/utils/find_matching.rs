use crate::{CodeDigits, IndexToIndexMap, IndexSet};

pub fn find_matching(code_a: CodeDigits, code_b: CodeDigits) -> IndexToIndexMap {
    let mut matching = IndexToIndexMap::new();
    let mut used_indexes = IndexSet::new();
    
    for i in 0..4 {
        for j in 0..4 {
            if code_a[i] == code_b[j] && !used_indexes.contains(&j) {
                matching.insert(i, j);
                used_indexes.insert(i);
                break;
            }
        }        
    }

    return matching;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_matching_between_0123_and_0123() {
        let code_a = [0, 1, 2, 3];
        let code_b = [0, 1, 2, 3];

        let expected_answer = IndexToIndexMap::from([
            (0,0),
            (1,1),
            (2,2),
            (3,3)
        ]);

        assert_eq!(expected_answer, find_matching(code_a, code_b));
    }

    #[test]
    fn find_matching_between_0123_and_0000() {
        let code_a = [0, 1, 2, 3];
        let code_b = [0, 0, 0, 0];

        let expected_answer = IndexToIndexMap::from([
            (0,0)
        ]);

        assert_eq!(expected_answer, find_matching(code_a, code_b));
    }

    #[test]
    fn find_matching_between_0000_and_0123() {
        let code_a = [0, 0, 0, 0];
        let code_b = [0, 1, 2, 3];

        let expected_answer = IndexToIndexMap::from([
            (0,0)
        ]);

        assert_eq!(expected_answer, find_matching(code_a, code_b));
    }
}