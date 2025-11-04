use std::collections::HashMap;
use std::hash::Hash;

pub struct VecHelpers;

impl VecHelpers {
    #[must_use]
    pub fn count_values<T: Eq + Hash + Clone>(values: Vec<Option<T>>) -> HashMap<Option<T>, usize> {
        let mut counts: HashMap<_, usize> = HashMap::new();
        for value in values {
            *counts.entry(value).or_insert(0) += 1;
        }
        counts
    }

    #[allow(clippy::as_conversions, clippy::cast_precision_loss)]
    #[must_use]
    pub fn get_supermajority<T: Eq + Hash + Clone>(values: Vec<Option<T>>) -> Option<T> {
        let total = values.len();
        let counts = VecHelpers::count_values(values);
        counts.into_iter().find_map(|(ext, count)| {
            if (count as f64) / (total as f64) > 0.7 {
                ext.clone()
            } else {
                None
            }
        })
    }

    #[must_use]
    pub fn join(parenthetical: Vec<String>, delimiter: char) -> String {
        parenthetical.iter().fold(String::new(), |mut acc, s| {
            if !acc.is_empty() {
                acc.push(delimiter);
            }
            acc.push_str(s);
            acc
        })
    }
}
