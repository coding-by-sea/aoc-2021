use std::fs;
fn main() {
    let filename = "../aoc_input1";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    run1_1(&contents)
}

fn run1_1(contents: &String) {
    let mut curr:Option<u32> = None;
    let mut count = 0;
    for line in contents.lines() {
        let prev:Option<u32> = curr;
        curr = Some(line.parse().expect("failed to parse into an integer"));
        let curr_val = match curr{
            Some(j) => j,
            None => panic!("curr value shouldn't be none"),
        };
        count += match prev{
            Some(i) => (curr_val > i) as u32,
            None => 0,
        };
        };
    println!("count {:?}", count);
}

