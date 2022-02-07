use std::fs;
mod day_09;
use day_09::part_1;
use day_09::part_2;

fn main() {
    // let filename = "examples/day_09";
    let filename = "inputs/day_09";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res = day_09::part_2(&contents);
    println!("result is {:?}", res);
}