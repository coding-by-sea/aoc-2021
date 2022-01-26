use std::fs;
mod day_08;
use day_08::part_1;
use day_08::part_2;

fn main() {
    let filename = "examples/day_08";
    // let filename = "inputs/day_08";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res = day_08::part_1(&contents);
    println!("result is {:?}", res);
}