use std::collections::HashMap;

fn dfs_1(neighbors: & HashMap<&str, Vec<&str>>, cave: &str, path: Vec<String>, paths: & mut Vec<Vec<String>>) {
    for neighbor in neighbors.get(cave).unwrap().iter() {
        let mut path_clone = path.clone();
        if neighbor.to_string() == neighbor.to_uppercase() {
            path_clone.push(neighbor.to_string());
        }
        else if !path.contains(&(neighbor.to_string())) {
                path_clone.push(neighbor.to_string())
            }
        else {
            continue
        }
        if *(path_clone.last().unwrap()) == "end" {
            paths.push(path_clone);
        }
        else {
            dfs_1(neighbors, neighbor, path_clone, paths);
        }
        }
}

pub fn part_1(contents: &str) -> i64 {
    let mut neighbors:HashMap<&str, Vec<&str>> = HashMap::new();
    for line in contents.lines() {
        let (start, end) = line.trim().split_once('-').unwrap();
        neighbors.entry(start).or_insert(Vec::new()).push(end);
        neighbors.entry(end).or_insert(Vec::new()).push(start);
    }
    let mut paths: Vec<Vec<String>> = Vec::new();
    dfs_1(&neighbors, "start", vec![("start").to_string()], & mut paths);
    // println!("{:?}", paths);
    paths.len() as i64
}


fn dfs_2<'a>(neighbors: & HashMap<&str, Vec<& 'a str>>, cave: &str, path: Vec<& 'a str>, paths: & mut Vec<Vec<& 'a str>>, small_twice_visited: bool) {
    for neighbor in neighbors.get(cave).unwrap().iter() {
        let mut updated_small_twice_visited = small_twice_visited;
        let mut path_clone = path.clone();
        if neighbor.to_string() == neighbor.to_uppercase() {
            path_clone.push(*neighbor);
        }
        else if !path.contains(neighbor) {
            path_clone.push(*neighbor)
        }
        else if neighbor.to_string() == "start" || small_twice_visited {
            continue
        }
        else {
            path_clone.push(*neighbor);
            updated_small_twice_visited = true;
        }

        if *(path_clone.last().unwrap()) == "end" {
            paths.push(path_clone);
        }
        else {
            dfs_2(neighbors, neighbor, path_clone, paths, updated_small_twice_visited);
        }
    }
}


pub fn part_2(contents: &str) -> i64 {
    let mut neighbors:HashMap<&str, Vec<&str>> = HashMap::new();
    for line in contents.lines() {
        let (start, end) = line.trim().split_once('-').unwrap();
        neighbors.entry(start).or_insert(Vec::new()).push(end);
        neighbors.entry(end).or_insert(Vec::new()).push(start);
    }
    let mut paths: Vec<Vec<&str>> = Vec::new();
    dfs_2(&neighbors, "start", vec!["start"], & mut paths, false);
    // println!("{:?}", paths);
    paths.len() as i64
    }
