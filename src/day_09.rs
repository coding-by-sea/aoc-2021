use std::collections::HashSet;

const directions: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn find_all_low_points(matrix: &Vec<Vec<i64>>, m: i64, n: i64) -> Vec<(i64, i64)> {
    let mut low_points: Vec<(i64, i64)> = vec!();
    for (i, row) in matrix.iter().enumerate() {
        for (j, num) in row.iter().enumerate() {
            if directions.iter().all(
                |x| {
                    let row_index = i as i64 + x.0;
                    let col_index = j as i64 + x.1;
                    (row_index < 0 || row_index >= m || col_index < 0 || col_index >= n || matrix[row_index as usize][col_index as usize] > *num)
                }
            ) {
                low_points.push((i as i64, j as i64));
            }
        }
    }
    return low_points
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
    let low_points: Vec<(i64, i64)> = find_all_low_points(&matrix, m, n);
    low_points.iter().fold(0, |accu, x| accu + matrix[x.0 as usize][x.1 as usize] + 1)
}

fn get_basin_size(matrix: &Vec<Vec<i64>>, m: i64, n: i64, low_point: &(i64, i64)) -> i64{
    let mut pts: Vec<(i64, i64)> = vec!();
    let mut size = 1;
    let mut visited: HashSet<(i64, i64)> = HashSet::from([*low_point]);

    pts.push(*low_point);
    while pts.len() > 0 {
        let pt = pts.pop().unwrap();
        for direction in directions {
            let row_index = pt.0 + direction.0;
            let col_index = pt.1 + direction.1;
            if row_index >= 0 && row_index < m && col_index >= 0 && col_index < n && matrix[row_index as usize][col_index as usize] != 9 && ! visited.contains(&(row_index, col_index)) {
                size += 1;
                pts.push((row_index, col_index));
                visited.insert((row_index, col_index));
            }

        }

    }
    size
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
    let low_points: Vec<(i64, i64)> = find_all_low_points(&matrix, m, n);
    let mut basin_sizes = low_points.iter().map(|pt| get_basin_size(&matrix, m, n, pt)).collect::<Vec<i64>>();
    let mut res: i64 = 1;
    basin_sizes.sort();
    basin_sizes.reverse();
    for (i, size) in basin_sizes.iter().enumerate() {
        res *= *size;
        if i  == 2 {
            break;
        }
    }
    res
}

