pub fn part_1(contents: &String) -> u32{
    let mut curr:Option<u32> = None;
    let mut count = 0;
    for line in contents.lines() {
        let prev:Option<u32> = curr;
        curr = Some(line.parse().expect("failed to parse into an integer"));
        let curr_val = match curr{
            Some(j) => j,
            None => panic!("curr value shouldn't be none"),
        };
        count += match prev{
            Some(i) => (curr_val > i) as u32,
            None => 0,
        };
    };
    count
}


pub fn part_2(contents: &String) -> u32{
    let depths: Vec<u32> = contents
        .lines()
        .map(|x| {
            x.parse()
                .expect("could not parse invalid integer string")
        })
        .collect();
    depths
        .iter()
        .fold(
            (0, None, None, None),
            |(count, a, b, c), x|{
                match (a, b, c){
                    (Some(aa), Some(_bb), Some(_cc)) => {
                        if  x > aa {
                            (count + 1, b, c, Some(x))
                        }
                        else{
                            (count, b, c, Some(x))
                        }

                    },
                    (None, None, None) => (count, None, None, Some(x)),
                    (None, None, Some(_cc)) => (count, None, c, Some(x)),
                    (None, Some(_bb), Some(_cc)) => (count, b, c, Some(x)),
                    _ => panic!("logic error"),

                }

            }
        ).0

}

