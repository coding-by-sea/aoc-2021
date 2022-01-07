use std::collections::HashMap;

fn calculate_all_scores(contents: &str) -> Vec<u32>  {
    let mut rows:Vec<Vec<u32>> = vec!();
    let mut numbers: Vec<u32> = vec!();
    let mut scores: Vec<u32> = vec!();
    for (i, line) in contents.lines().enumerate(){

        if i == 0 {
            numbers = line.split(',').map(|x| x.parse().expect("cannot parse the input numbers")).collect();
        }
        else if (i - 1) % 6 != 0 {
            rows.push(line.split_whitespace().map(|x| x.parse().expect("cannot parse the input numbers")).collect());
        }
    }
    let mut row_sums = vec![0; rows.len()];
    let mut col_sums= vec![0; rows.len()];
    let mut board_won: Vec<u32> = vec![0; rows.len() / 5];
    let mut num_pos: HashMap<u32, Vec<(usize, usize)>> = HashMap::new();
    for (i, row) in rows.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            row_sums[i] += num;
            let index_col_sums= 5 * (i / 5) + j;
            col_sums[index_col_sums] += num;
            match num_pos.get_mut(&num) {
                Some(list) => list.push((i, index_col_sums)),
                None => {num_pos.insert(num, vec![(i, index_col_sums)]);}
            }
        }
    }
    let mut winning_num : u32 = 0;
    for num in numbers.into_iter() {
        winning_num = num;
        match num_pos.get(&num) {
            Some(list) => {
                for &(index_row, index_col) in list.iter() {
                    row_sums[index_row] -= num;
                    col_sums[index_col] -= num;
                    if (row_sums[index_row] == 0 || col_sums[index_col] == 0) && (board_won[index_row / 5] == 0) {
                        scores.push(num * (row_sums[index_row / 5 * 5..index_row / 5 * 5 + 5].iter().sum::<u32>()));
                        board_won[index_row / 5] = 1;
                    }
                }
            }
            None => {}
        }
    }
    scores
}

pub fn part_1(contents: &str) -> u32 {
    let scores = calculate_all_scores(contents);
    *scores.iter().next().unwrap()
}

pub fn part_2(contents: &str) -> u32 {
    let scores = calculate_all_scores(contents);
    *scores.last().unwrap()
}