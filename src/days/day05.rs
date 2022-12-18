use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};
use regex::Regex;
use std::collections::HashMap;

pub const DAY: Day = Day {
    day: 5,
    name: "Supply Stacks",
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

#[derive(Debug, Clone)]
struct State {
    crates: HashMap<u32, Vec<char>>,
}

impl State {
    fn move_9000(&self, instruction: &Instruction) -> State {
        let Instruction { count, from, to } = instruction;
        let mut new_crates = self.crates.clone();

        for _ in 0..*count {
            let c = new_crates.get_mut(from).unwrap().pop().unwrap();
            new_crates.get_mut(to).unwrap().push(c);
        }

        State { crates: new_crates }
    }

    fn move_9001(&self, instruction: &Instruction) -> State {
        let Instruction { count, from, to } = instruction;
        let mut new_crates = self.crates.clone();

        let from_count = new_crates.get(from).unwrap().len();
        let moving: Vec<char> = new_crates
            .get_mut(from)
            .unwrap()
            .drain((from_count - *count as usize)..)
            .collect();
        new_crates.get_mut(to).unwrap().extend_from_slice(&moving);

        State { crates: new_crates }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    count: u32,
    from: u32,
    to: u32,
}

fn parse(input: &str) -> Result<(State, Vec<Instruction>), std::num::ParseIntError> {
    let input_items: Vec<&str> = input.trim().split("\n\n").collect();

    Ok((
        parse_initial(input_items[0]).unwrap(),
        parse_instructions(input_items[1]).unwrap(),
    ))
}

fn parse_initial(input: &str) -> Result<State> {
    let mut crate_state: HashMap<u32, Vec<char>> = HashMap::new();

    let mut lines: Vec<&str> = input.lines().collect();
    lines.reverse();

    // Column based parsing.
    for (i, c) in lines[0].chars().enumerate() {
        if c == ' ' {
            continue;
        }

        let index = c.to_digit(10).unwrap();
        let mut crates = Vec::new();

        for line_index in 1..lines.len() {
            let crate_val = lines[line_index].chars().nth(i).unwrap();
            if crate_val != ' ' {
                crates.push(crate_val);
            }
        }

        crate_state.insert(index, crates);
    }

    Ok(State {
        crates: crate_state,
    })
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>> {
    let instruction_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    input
        .lines()
        .map(|l| {
            let caps = instruction_regex.captures(l).unwrap();
            Ok(Instruction {
                count: caps[1].parse().unwrap(),
                from: caps[2].parse().unwrap(),
                to: caps[3].parse().unwrap(),
            })
        })
        .collect()
}

fn solve_part1(initial: &(State, Vec<Instruction>)) -> String {
    let mut state = initial.0.clone();
    for instruction in &initial.1 {
        state = state.move_9000(&instruction);
    }

    let result = String::from_iter(
        (1..=state.crates.len()).map(|i| state.crates.get(&(i as u32)).unwrap().last().unwrap()),
    );
    result
}

fn solve_part2(initial: &(State, Vec<Instruction>)) -> String {
    let mut state = initial.0.clone();
    for instruction in &initial.1 {
        state = state.move_9001(&instruction);
    }

    let result = String::from_iter(
        (1..=state.crates.len()).map(|i| state.crates.get(&(i as u32)).unwrap().last().unwrap()),
    );
    result
}
