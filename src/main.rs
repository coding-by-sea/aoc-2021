use std::fs;
mod day_10;
use day_10::part_1;
use day_10::part_2;

fn main() {
    // let filename = "examples/day_10";
    let filename = "inputs/day_10";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res = day_10::part_2(&contents);
    println!("result is {:?}", res);
}