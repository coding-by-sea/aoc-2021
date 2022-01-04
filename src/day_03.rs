fn update_count(count: &mut Vec<u32>, line: &str) -> (){
    for (i, char) in line.chars().enumerate(){
        count[i] += match char{
            '1' => 1,
            _=>0
        }
    }
}
pub fn part_1(contents: &str) -> u32 {
    let mut num = 0;
    let mut count: Vec<u32> = vec![0; contents.lines().next().unwrap().len() as usize];
    for line in contents.lines(){
        num += 1;
        update_count(&mut count, line)
    }
    let gamma: String = count.iter()
        .map(|x| {
            if x > & (num / 2) {
                '1'
            }
            else {
                '0'
            }
        })
        .collect();
    let gamma_parsed: u32 = u32::from_str_radix(&gamma, 2).expect("not a valid integer");
    gamma_parsed * (gamma_parsed ^ (u32::pow(2,count.len() as u32) - 1))
}


fn get_max_bits(number: u32) -> u32{
    let mut count = 0;
    let mut x = number;
    while x > 0 {
        count += 1;
        x >>= 1;
    }
    count
}

fn get_most_common_bit(numbers: &Vec<u32>, pos: i32) -> u32{
    let count = numbers.iter().filter(|x|{(*x & (1 << pos)) / (1 << pos) == 1}).count();
    if count >= (numbers.len() + 1) / 2 {
        1
    }
    else {
        0
    }
}
pub fn part_2(contents: &str) -> u32{
    let numbers:Vec<u32> = contents.lines().map(|x| u32::from_str_radix(x, 2).expect("not a valid integer")).collect();
    let max_number = numbers.iter().max().unwrap();
    let max_bits = get_max_bits(*max_number);
    let mut oxygen_generator_numbers = numbers.clone();
    let mut co2_scrubber_numbers = numbers.clone();

    let mut pos: i32 = (max_bits - 1) as i32;
    while oxygen_generator_numbers.len() > 1 {
        let most_common_bit = get_most_common_bit(&oxygen_generator_numbers, pos);
        oxygen_generator_numbers = oxygen_generator_numbers.into_iter().filter(|x| {most_common_bit == (x & (1 << pos)) / (1 << pos)}).collect();
        pos -= 1;
    }
    let oxygen_generator_rating = oxygen_generator_numbers.iter().next().unwrap();

    pos = (max_bits - 1) as i32;
    while co2_scrubber_numbers.len() > 1 {
        let least_common_bit = 1 ^ get_most_common_bit(&co2_scrubber_numbers, pos);
        co2_scrubber_numbers = co2_scrubber_numbers.into_iter().filter(|x| {least_common_bit == (x & (1 << pos)) / (1 << pos)}).collect();
        pos -= 1;
    }
    let co2_scrubber_rating = co2_scrubber_numbers.iter().next().unwrap();
    oxygen_generator_rating * co2_scrubber_rating
}
