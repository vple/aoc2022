use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 4,
    name: "Camp Cleanup",
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

#[derive(Debug, Clone, Copy)]
struct SectionAssignment(u32, u32);

fn parse(
    input: &str,
) -> Result<Vec<(SectionAssignment, SectionAssignment)>, std::num::ParseIntError> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .split(",")
                .map(|assignment| {
                    let section_ids: Vec<u32> = assignment
                        .trim()
                        .split("-")
                        .map(str::parse)
                        .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
                        .unwrap();
                    SectionAssignment(section_ids[0], section_ids[1])
                })
                .collect()
        })
        .map(|assignments: Vec<SectionAssignment>| Ok((assignments[0], assignments[1])))
        .collect()
}

fn solve_part1(assignments: &Vec<(SectionAssignment, SectionAssignment)>) -> u32 {
    assignments
        .iter()
        .filter(|(first, second)| {
            (first.0 <= second.0 && first.1 >= second.1)
                || (first.0 >= second.0 && first.1 <= second.1)
        })
        .count() as u32
}

fn solve_part2(assignments: &Vec<(SectionAssignment, SectionAssignment)>) -> u32 {
    assignments
        .iter()
        .filter(|(first, second)| {
            (first.0 <= second.0 && second.0 <= first.1)
                || (second.0 <= first.0 && first.0 <= second.1)
        })
        .count() as u32
}
