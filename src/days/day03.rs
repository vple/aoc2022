use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use array_tool::vec::*;
use color_eyre::{Report, Result};
use itertools::Itertools;
use std::collections::HashSet;

pub const DAY: Day = Day {
    day: 3,
    name: "Rucksack Reorganization",
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
struct Rucksack {
    compartment1: Vec<char>,
    compartment2: Vec<char>,
}

fn parse(input: &str) -> Result<Vec<Rucksack>, std::num::ParseIntError> {
    input
        .lines()
        .map(|l| l.trim().chars().collect())
        .map(|c: Vec<char>| {
            Ok(Rucksack {
                compartment1: c[0..c.len() / 2].to_vec(),
                compartment2: c[c.len() / 2..c.len()].to_vec(),
            })
        })
        .collect()
}

fn priority(item: &char) -> u32 {
    let ascii = *item as u32;
    if ascii >= 'a' as u32 {
        ascii - ('a' as u32) + 1
    } else {
        ascii - ('A' as u32) + 1 + 26
    }
}

fn solve_part1(rucksacks: &Vec<Rucksack>) -> u32 {
    let mut total = 0;
    for rucksack in rucksacks.iter() {
        let c1_set: HashSet<char> = HashSet::from_iter(rucksack.compartment1.iter().cloned());
        let c2_set: HashSet<char> = HashSet::from_iter(rucksack.compartment2.iter().cloned());

        c1_set
            .intersection(&c2_set)
            .for_each(|i| total += priority(i))
    }

    total
}

fn solve_part2(rucksacks: &Vec<Rucksack>) -> u32 {
    let mut total = 0;
    for group in &rucksacks.into_iter().chunks(3) {
        let rucksack_items: Vec<Vec<char>> = group
            .into_iter()
            .map(|rucksack| rucksack.compartment1.union(rucksack.compartment2.clone()))
            .collect();

        let badge = rucksack_items
            .into_iter()
            .reduce(|a, b| a.intersect(b.clone()))
            .unwrap();
        badge.iter().for_each(|i| total += priority(i));
    }
    total
}
