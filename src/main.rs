use std::fs;
fn main() {
    // let filename = "examples/day_02";
    let filename = "inputs/day_02";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res = run2_2(&contents);
    println!("result is {:?}", res);
}

fn run1_1(contents: &String) -> u32{
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


fn run1_2(contents: &String) -> u32{
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

#[derive(Debug)]
struct SubmarineOperation{
    direction: String,
    steps: u32,
}

impl SubmarineOperation{
    fn new(line: &str) -> SubmarineOperation {
        let mut splited = line.split(' ');
        let direction = splited.next().unwrap().to_string();
        let steps = splited.next().unwrap().parse().expect("failed to parse into an integer");
        SubmarineOperation { direction, steps }
    }
}

fn run2_1(contents: &String) -> u32{
        let operations = contents.lines().map(|line|SubmarineOperation::new(line)).collect::<Vec<SubmarineOperation>>();
        operations
            .iter()
            .fold([0, 0], |[pos, dep], operation| {match operation.direction.as_str(){
                "forward" => [pos + operation.steps, dep],
                "backward" => [pos - operation.steps, dep],
                "up" => [pos,  dep - operation.steps],
                "down" => [pos,  dep + operation.steps],
                _=>[pos, dep]
            }})
            .iter()
            .fold(1, |res, x| {res * x})
    }

fn run2_2(contents: &String) -> u32 {
    let operations = contents.lines().map(|line| SubmarineOperation::new(line)).collect::<Vec<SubmarineOperation>>();
    let res = operations
        .iter()
        .fold([0, 0, 0], |[pos, dep, aim], operation| {
            match operation.direction.as_str() {
                "forward" => [pos + operation.steps, dep + operation.steps * aim, aim],
                "up" => [pos, dep, aim - operation.steps],
                "down" => [pos, dep, aim + operation.steps],
                _ => [pos, dep, aim]
            }
        });
    res.split_last()
        .unwrap()
        .1
        .iter()
        .fold(1, |res, x| { res * x })
}
