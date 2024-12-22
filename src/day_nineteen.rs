use std::collections::{BinaryHeap, HashSet};
use crate::read_input;

pub fn first_task() {
    let patterns_input = read_input("./input/19_patterns").into_iter()
        .reduce(|mut a, b| {
            a.push_str(&b);
            a
        })
        .unwrap();

    let patterns: Vec<&str> = patterns_input.split(",").collect::<Vec<&str>>();

    let desired_patterns = read_input("./input/19_desired_patterns");

    let sum= desired_patterns.into_iter().filter(|dp|{
        is_buildable_from_patterns(dp, &patterns)
    }).count();

    println!("Answer 1/2: {}", sum);
}

fn is_buildable_from_patterns(desired_pattern: &str, patterns: &Vec<&str>) -> bool {
    let mut pattern: String = desired_pattern.to_owned().clone();
    let mut patterns_store: BinaryHeap<&str> = BinaryHeap::from_iter(patterns.clone());
    while pattern.len() > 0 && patterns_store.len() > 0 {
        let p = patterns_store.pop().unwrap();
        if pattern.contains(p) {
            pattern = pattern.replace(p, "");
        }
    }
    if (patterns_store.len() == 0 && pattern.len() > 0) {
        // all patterns got tested, no match
        return false;
    }
    if pattern.len() == 0 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::day_nineteen::is_buildable_from_patterns;
    use crate::read_input;

    #[test]
    fn simulate_task_one_run() {
        let input = read_input("./input/19_test");

        let patterns = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        let desired_patterns = vec!["brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb"];

        let sum= desired_patterns.into_iter().filter(|dp|{
            is_buildable_from_patterns(dp, &patterns)
        }).count();

        assert_eq!(sum, 6);
    }

    #[test]
    fn test_is_buildable_from_patterns() {
        let patterns = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

        assert_eq!(is_buildable_from_patterns("brwrr", &patterns), true);
        assert_eq!(is_buildable_from_patterns("bggr", &patterns), true);
        assert_eq!(is_buildable_from_patterns("gbbr", &patterns), true);
        assert_eq!(is_buildable_from_patterns("rrbgbr", &patterns), true);
        assert_eq!(is_buildable_from_patterns("bwurrg", &patterns), true);
        assert_eq!(is_buildable_from_patterns("brgr", &patterns), true);
        assert_eq!(is_buildable_from_patterns("ubwu", &patterns), false);
        assert_eq!(is_buildable_from_patterns("bbrgwb", &patterns), false);
    }
}