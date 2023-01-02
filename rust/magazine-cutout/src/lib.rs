// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut hash_m = HashMap::new();

    let mut hash_n = HashMap::new();

    magazine
        .iter()
        .for_each(|v| *hash_m.entry(*v).or_insert(1) += 1);

    note.iter()
        .for_each(|v| *hash_n.entry(*v).or_insert(1) += 1);

    for key in hash_n.keys() {
        let m = hash_m.get(key);
        let n = hash_n.get(key);

        match (m, n) {
            (Some(m_val), Some(n_val)) => {
                if m_val < n_val {
                    return false;
                }
            }
            _ => return false,
        };
    }

    true
}
