use std::fs;
mod day_12;
use day_12::part_1;
use day_12::part_2;

fn main() {
    // let filename = "examples/day_12";
    let filename = "inputs/day_12";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res = day_12::part_2(&contents);
    println!("result is {:?}", res);
}