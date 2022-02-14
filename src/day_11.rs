use std::collections::{HashMap, HashSet};
const directions: [(i64, i64); 8] = [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

fn get_one_iteration(matrix: &mut Vec<Vec<i64>>, m: i64, n: i64) -> i64 {
    let mut num_flashes = 0;
    let mut stack: Vec<(usize, usize)> = vec![];
    let mut flashed_pts: HashSet<(usize, usize)> = HashSet::new();
    for (i, row) in matrix.iter_mut().enumerate() {
        for (j, num) in row.iter_mut().enumerate() {
            *num += 1;
            if *num > 9 {
                stack.push((i, j))
            }
        }
    }

    while stack.len() > 0 {
        let flashing_pt = stack.pop().unwrap();
        if flashed_pts.contains(&flashing_pt) {
            continue;
        }
        else {
            flashed_pts.insert(flashing_pt);
            num_flashes += 1;
        }
        matrix[flashing_pt.0][flashing_pt.1] = 0;
        for direction in directions.into_iter() {
            let new_i = flashing_pt.0 as i64 + direction.0;
            let new_j = flashing_pt.1 as i64 + direction.1;
            if new_i >= 0 && new_i < m && new_j >= 0 && new_j < n && !flashed_pts.contains(&(new_i as usize, new_j as usize)) {
                matrix[new_i as usize][new_j as usize] += 1;
                if matrix[new_i as usize][new_j as usize] > 9 {
                    stack.push((new_i as usize, new_j as usize));
                }
            }
        }
    }
    num_flashes
}

pub fn part_1(contents: &str) -> i64 {
    let mut matrix: Vec<Vec<i64>> = vec!();
    for line in contents.lines() {
        let mut row: Vec<i64> = vec!();
        for char in line.chars() {
            row.push(char.to_digit(10).unwrap() as i64);
        }
        matrix.push(row);
    }
    let m = matrix.len() as i64;
    let n =matrix[0].len() as i64;
    let mut num_flashes = 0;
    for _ in 0..100 {
        num_flashes += get_one_iteration(& mut matrix, m, n);
    }
    num_flashes
}


pub fn part_2(contents: &str) -> i64 {
    let mut matrix: Vec<Vec<i64>> = vec!();
    for line in contents.lines() {
        let mut row: Vec<i64> = vec!();
        for char in line.chars() {
            row.push(char.to_digit(10).unwrap() as i64);
        }
        matrix.push(row);
    }
    let m = matrix.len() as i64;
    let n =matrix[0].len() as i64;
    let mut step = 0;
    loop {
        step += 1;
        if m * n == get_one_iteration(&mut matrix, m, n) {
            return step;
        }
    }
    0
}
