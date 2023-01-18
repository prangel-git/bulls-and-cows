use crate::{CodeDigits, IndexToIndexMap, IndexSet, CODELENGTH};

pub fn find_matching(code_a: &CodeDigits, code_b: &CodeDigits) -> IndexToIndexMap {
    let mut matching = IndexToIndexMap::new();
    let mut used_indexes = IndexSet::new();
    
    find_matchings_with_same_index(code_a, code_b, &mut matching, &mut used_indexes);
    find_all_matchings(code_a, code_b, &mut matching, &mut used_indexes); 

    return matching;
}

fn find_all_matchings(
    code_a: &CodeDigits, 
    code_b: &CodeDigits, 
    matching: &mut IndexToIndexMap,
    used_indexes: &mut std::collections::HashSet<usize>
) {
    for i in 0..CODELENGTH {
        for j in 0..CODELENGTH {
            if code_a[i] == code_b[j] && !matching.contains_key(&i) && !used_indexes.contains(&j) {
                matching.insert(i, j);
                used_indexes.insert(j);
                break;
            }
        }
    }
}

fn find_matchings_with_same_index(
    code_a: &CodeDigits, 
    code_b: &CodeDigits, 
    matching: &mut IndexToIndexMap, 
    used_indexes: &mut IndexSet
) {
    for i in 0..CODELENGTH {
        if code_a[i] == code_b[i] {
            matching.insert(i, i);
            used_indexes.insert(i);
        } 
    }
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

        assert_eq!(expected_answer, find_matching(&code_a, &code_b));
    }

    #[test]
    fn find_matching_between_0123_and_0000() {
        let code_a = [0, 1, 2, 3];
        let code_b = [0, 0, 0, 0];

        let expected_answer = IndexToIndexMap::from([
            (0,0)
        ]);

        assert_eq!(expected_answer, find_matching(&code_a, &code_b));
    }

    #[test]
    fn find_matching_between_0000_and_0123() {
        let code_a = [0, 0, 0, 0];
        let code_b = [0, 1, 2, 3];

        let expected_answer = IndexToIndexMap::from([
            (0,0)
        ]);

        assert_eq!(expected_answer, find_matching(&code_a, &code_b));
    }

    #[test]
    fn find_matching_between_1111_and_0123() {
        let code_a = [1, 1, 1, 1];
        let code_b = [0, 1, 2, 3];

        let expected_answer = IndexToIndexMap::from([
            (1,1)
        ]);

        assert_eq!(expected_answer, find_matching(&code_a, &code_b));
    }
}