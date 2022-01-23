use std::fs;
mod day_07;
pub use crate::day_07::part_1;
pub use crate::day_07::part_2;

fn main() {
    // let filename = "examples/day_07";
    let filename = "inputs/day_07";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res = part_2(&contents);
    println!("result is {:?}", res);
}