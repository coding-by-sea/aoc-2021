use std::cmp::{max, min};
use std::collections::HashMap;
use std::num;
use std::path::Iter;
use std::str::Split;
use counter::Counter;

#[derive(Debug)]
struct Line {
    pt1: (u32, u32),
    pt2: (u32, u32),
}
impl Line {
    fn get_points_btw(&self) -> Vec<(u32, u32)>{
        let mut points = Vec::new();
        if self.is_horizontal() {
            let max = max(self.pt1.0, self.pt2.0);
            let min = min(self.pt1.0, self.pt2.0);
            for x in min..(max + 1) {
               points.push((x, self.pt1.1))
            }
        }
        else if self.is_vertical() {
            let max = max(self.pt1.1, self.pt2.1);
            let min = min(self.pt1.1, self.pt2.1);
            for y in min..(max + 1) {
                points.push((self.pt1.0, y))
            }
        }
        else {
            let max = max(self.pt1.1, self.pt2.1) as i32;
            let min = min(self.pt1.1, self.pt2.1) as i32;
            for i in 0..(max - min + 1) {
                points.push(((self.pt1.0 as i32 - i * (self.pt1.0 as i32 - self.pt2.0 as i32).signum()) as u32, (self.pt1.1 as i32 - i * (self.pt1.1 as i32 - self.pt2.1 as i32).signum()) as u32));
            }
        }
        points
    }

    fn is_horizontal_or_vertical(&self) -> bool {
       self.is_horizontal() || self.is_vertical()
    }

    fn is_horizontal(&self) -> bool {
        self.pt1.1 == self.pt2.1
    }

    fn is_vertical(&self) -> bool {
        self.pt1.0 == self.pt2.0
    }

    fn is_diagonal(&self) -> bool {(self.pt1.0 as i32 - self.pt2.0 as i32).abs() == (self.pt1.1 as i32 - self.pt2.1 as i32).abs()}

}

impl TryFrom<&str> for Line{
    type Error = num::ParseIntError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let line = value.trim();
        let splited: Vec<&str> = line.split(" -> ").collect();
        let mut left_splited = splited[0].split(",");
        let mut right_splited = splited[1].split(",");
        Ok(Line{
            pt1: (left_splited.next().unwrap().parse()?, left_splited.next().unwrap().parse()?),
            pt2: (right_splited.next().unwrap().parse()?, right_splited.next().unwrap().parse()?),
        })
    }
}
pub fn part_1(contents: &str) -> u32 {
    let mut num_pt2_overlapped = 0;
    let mut counter: Counter<(u32, u32), u32> = Counter::new();
    for line_in_contents in contents.lines() {
        let line = Line::try_from(line_in_contents).expect("not be able to parse contents");
        if line.is_horizontal_or_vertical() {
            for pt in line.get_points_btw() {
                counter[&pt] += 1;
                if counter[&pt] == 2 {
                    num_pt2_overlapped += 1;
                }
            }
        }
    }
    num_pt2_overlapped
}

pub fn part_2(contents: &str) -> u32 {
    let mut num_pts_overlapped = 0;
    let mut counter: Counter<(u32, u32), u32> = Counter::new();
    for line_in_contents in contents.lines() {
        let line = Line::try_from(line_in_contents).expect("not be able to parse contents");
        if line.is_horizontal_or_vertical() || line.is_diagonal() {
            for pt in line.get_points_btw() {
                counter[&pt] += 1;
                if counter[&pt] == 2 {
                    num_pts_overlapped += 1;
                }
            }
        }
    }
    num_pts_overlapped
}
