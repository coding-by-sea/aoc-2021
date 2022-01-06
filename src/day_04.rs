use std::collections::HashMap;

pub fn part_1(contents: &str) -> Result<u32, String>  {
    let mut rows:Vec<Vec<u32>> = vec!();
    let mut numbers: Vec<u32> = vec!();
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
        println!("{}", winning_num);
        println!("{:?}", row_sums);
        println!("{:?}", col_sums);
        winning_num = num;
        match num_pos.get(&num) {
            Some(list) => {
                for &(index_row, index_col) in list.iter() {
                    row_sums[index_row] -= num;
                    col_sums[index_col] -= num;
                    if row_sums[index_row] == 0 || col_sums[index_col] == 0 {
                        return Ok(num * (row_sums[index_row / 5 * 5..index_row / 5 * 5 + 5].iter().sum::<u32>()))
                    }
                }
            }
            None => {}
        }
    }
    Err("no winning board".to_string())


}

pub fn part_2(contents: &str) -> u32 {
    1
}
