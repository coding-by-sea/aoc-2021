use std::fs;
mod day_11;
use day_11::part_1;
use day_11::part_2;

fn main() {
    // let filename = "examples/day_11";
    let filename = "inputs/day_11";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res = day_11::part_2(&contents);
    println!("result is {:?}", res);
}