use std::collections::HashMap;

fn get_score_corrupted(char_score_mapping: &HashMap<char, i64>, char_char_mapping: &HashMap<char, char>, line: &str) -> i64 {
    let mut stack: Vec<char> = vec![];
    for char in line.chars() {
        if !char_score_mapping.contains_key(&char) {
            stack.push(char);
        }
        else if stack.len() == 0 {
            return 0;
        }
        else if *char_char_mapping.get(&char).unwrap() == *stack.last().unwrap() {
            stack.pop();
        }
        else {
            return *char_score_mapping.get(&char).unwrap();
        }

    }
    0
}


fn get_score_incomplete(char_score_mapping: &HashMap<char, i64>, char_char_mapping: &HashMap<char, char>, line: &str) -> i64 {
    let mut stack: Vec<char> = vec![];
    for char in line.chars() {
        if !char_score_mapping.contains_key(&char) {
            stack.push(char);
        }
        else if stack.len() == 0 {
            return 0;
        }
        else if char == *char_char_mapping.get(stack.last().unwrap()).unwrap() {
            stack.pop();
        }
        else {
            return 0;
        }

    }
    stack.reverse();
    stack.iter().fold(0, |accu, char| 5 * accu + char_score_mapping.get(char_char_mapping.get(char).unwrap()).unwrap())
}

pub fn part_1(contents: &str) -> i64 {
    let char_score_mapping: HashMap<char, i64> = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let char_char_mapping: HashMap<char, char> = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    contents.lines().fold(0, |accu, line| accu + get_score_corrupted(&char_score_mapping, &char_char_mapping, line))
    }

pub fn part_2(contents: &str) -> i64 {
    let char_score_mapping: HashMap<char, i64> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let char_char_mapping: HashMap<char, char> = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut scores = contents.lines().map(|line| get_score_incomplete(&char_score_mapping, &char_char_mapping, line)).filter(|x| *x != 0).collect::<Vec<i64>>();
    scores.sort();
    scores[scores.len() / 2]
}
