use aoc_2021::day_07;
use std::fs;
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 07");
    let filename = "inputs/day_07";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let school:Vec<day_07::Crab> = contents.split(',').map(|x| day_07::Crab::from(x)).collect();

    group.bench_function("part 1", |b| {
        b.iter(|| black_box(day_07::part_1(&school)))
    });
    group.bench_function("part 2", |b| {
        b.iter(|| black_box(day_07::part_2(&school)))
    });
    group.finish();
}

criterion_group!(benches, benchmark);