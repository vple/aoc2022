use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};
use std::collections::HashSet;

pub const DAY: Day = Day {
    day: 6,
    name: "Tuning Trouble",
    part_1: run_part1,
    part_2: Some(run_part2),
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

fn parse(input: &str) -> Result<&str, std::num::ParseIntError> {
    Ok(input.trim())
}

fn find_marker(signal: &str, num_chars: usize) -> u32 {
    let mut i = 0;
    loop {
        let marker = &signal[i..i+num_chars];
        let chars: HashSet<char> = HashSet::from_iter(marker.chars());
        
        if chars.len() == num_chars {
            break;
        }
        i+=1;
    }
    i as u32
}

fn solve_part1(signal: &str) -> u32 {
    find_marker(signal, 4) + 4
}

fn solve_part2(signal: &str) -> u32 {
    find_marker(signal, 14) + 14
}
