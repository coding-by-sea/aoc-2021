use std::fs;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
pub use crate::day_03::part_1;
pub use crate::day_03::part_2;

fn main() {
    // let filename = "examples/day_03";
    let filename = "inputs/day_03";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res = part_2(&contents);
    println!("result is {:?}", res);
}