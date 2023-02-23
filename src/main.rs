use std::fs;
mod day_13;
use day_13::part_1;
use day_13::part_2;

fn main() {
    let filename = "examples/day_13";
    // let filename = "inputs/day_13";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res = day_13::part_1(&contents);
    println!("result is {:?}", res);
}