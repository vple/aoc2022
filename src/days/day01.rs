use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 1,
    name: "Calorie Counting",
    part_1: run_part1,
    part_2: Some(run_part2),
    // other: &[("Parse", run_parse), ("No Alloc", run_no_alloc)],
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve_part1(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve_part2(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn parse(input: &str) -> Result<Vec<Vec<u32>>, std::num::ParseIntError> {
    input
        .trim()
        .split("\n\n")
        .map(|block| block.trim().lines().map(str::parse).collect())
        .collect()
}

fn solve_part1(elves: &[Vec<u32>]) -> u32 {
    elves.iter().map(|e| e.iter().sum()).max().unwrap()
}

fn solve_part2(elves: &[Vec<u32>]) -> u32 {
    let mut calories: Vec<u32> = elves.iter().map(|e| e.iter().sum()).collect();
    calories.sort();
    calories.iter().rev().take(3).sum()
}
