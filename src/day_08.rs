use std::collections::HashSet;

pub fn part_1(contents: &str) -> i64 {
    let length_simple_digits: HashSet<usize> = HashSet::from([2,3,4,7]);
    let mut count = 0;
    for line in contents.lines() {
        count += line.split('|')
            .last()
            .unwrap()
            .split_whitespace()
            .fold(
                0,
                |count, word| {
                    match length_simple_digits.contains(&(word.len())) {
                        true => count + 1,
                        false => count
                    }
                });
    }
    count
}

pub fn part_2(contents: &str) -> i64 {
    1
}
