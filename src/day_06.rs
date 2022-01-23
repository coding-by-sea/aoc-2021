use std::collections::HashMap;

#[derive(Debug)]
struct Fish {
    age: i64
}

impl Fish {
    const NEWBORN_CYCLE: i64 = 9;
    const BIRTH_CYCLE: i64 = 7;

    fn get_num_fish(&self, mut days: i64, mem: &mut HashMap<i64, i64>) -> i64 {
        let mut count: i64 = 1;
        days = days - self.age - 1;
        let days_to_cache = days;
        if let Some(val) = mem.get(&days) {
            return *val;
        }
        while days >= 0 {
            let new_fish = Fish {
                age: Self::NEWBORN_CYCLE - 1
            };
            count += new_fish.get_num_fish(days, mem);
            days = days - Self::BIRTH_CYCLE;
        }
        mem.insert(days_to_cache, count);
        count
    }

}

impl From<&str> for Fish{
    fn from(value: &str) -> Self {
    Fish{
            age: value.parse().unwrap()
        }
    }
}


fn simulate(contents: &str, days: i64) -> i64 {
    let mut school: Vec<Fish> = vec!();
    for age in contents.split(',') {
       school.push(Fish::from(age))
    }
    let mut count: i64 = 0;
    let mut mem: HashMap<i64, i64> = HashMap::new();
    for fish in school.iter() {
        count += fish.get_num_fish(days, &mut mem);
        println!("{:?}", count);
    }
    count
}


pub fn part_1(contents: &str) -> i64 {
    simulate(contents, 80)
}

pub fn part_2(contents: &str) -> i64 {
    simulate(contents, 256)
}
