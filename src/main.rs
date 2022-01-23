use std::fs;
mod day_06;
pub use crate::day_06::part_1;
pub use crate::day_06::part_2;

fn main() {
    // let filename = "examples/day_06";
    let filename = "inputs/day_06";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res = part_2(&contents);
    println!("result is {:?}", res);
}