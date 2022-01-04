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
pub fn part_1(contents: &String) -> u32{
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

pub fn part_2(contents: &String) -> u32{
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
