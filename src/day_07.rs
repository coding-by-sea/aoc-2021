use std::cmp;

#[derive(Debug)]
pub struct Crab {
    pos: i64
}

impl Crab {
    fn get_constant_fuel_cost(&self, pos_x: i64) -> i64{
        (self.pos - pos_x).abs()
    }
    fn get_increasing_fuel_cost(&self, pos_x: i64) -> i64{
        (self.pos - pos_x).abs() * (1+ (self.pos - pos_x).abs()) / 2
    }
}
impl From<&str> for Crab {
    fn from(value: &str) -> Self {
        Crab {
            pos: value.parse().unwrap()
        }
    }
}

pub fn part_1(school: &Vec<Crab>) -> i64 {
    let min_pos: i64 = school.iter().fold(0, |min, crab| cmp::min(min, crab.pos));
    let max_pos: i64 = school.iter().fold(0, |max, crab| cmp::max(max, crab.pos));
    let mut left = min_pos;
    let mut right = max_pos;
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        let fuel_cost_mid_minus_one = school.iter().fold(0, |acc, crab| acc + crab.get_constant_fuel_cost(mid - 1));
        let fuel_cost_mid = school.iter().fold(0, |acc, crab| acc + crab.get_constant_fuel_cost(mid));
        if fuel_cost_mid_minus_one >= fuel_cost_mid {
            left = mid;
        }
        else {
            right = mid;
        }
    }
    let fuel_cost_left = school.iter().fold(0, |acc, crab| acc + crab.get_constant_fuel_cost(left));
    let fuel_cost_right = school.iter().fold(0, |acc, crab| acc + crab.get_constant_fuel_cost(right));
    cmp::min(fuel_cost_left, fuel_cost_right)
}

pub fn part_2(school: &Vec<Crab>) -> i64 {
    let min_pos: i64 = school.iter().fold(0, |min, crab| cmp::min(min, crab.pos));
    let max_pos: i64 = school.iter().fold(0, |max, crab| cmp::max(max, crab.pos));
    let mut left = min_pos;
    let mut right = max_pos;
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        let fuel_cost_mid_minus_one = school.iter().fold(0, |acc, crab| acc + crab.get_increasing_fuel_cost(mid - 1));
        let fuel_cost_mid = school.iter().fold(0, |acc, crab| acc + crab.get_increasing_fuel_cost(mid));
        if fuel_cost_mid_minus_one >= fuel_cost_mid {
            left = mid;
        }
        else {
            right = mid;
        }
    }
    let fuel_cost_left = school.iter().fold(0, |acc, crab| acc + crab.get_increasing_fuel_cost(left));
    let fuel_cost_right = school.iter().fold(0, |acc, crab| acc + crab.get_increasing_fuel_cost(right));
    cmp::min(fuel_cost_left, fuel_cost_right)
}