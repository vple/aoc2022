use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

pub const DAY: Day = Day {
    day: 2,
    name: "Rock Paper Scissors",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve_part1(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse2(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve_part2(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

const WIN_SCORE: u32 = 6;
const DRAW_SCORE: u32 = 3;
const LOSE_SCORE: u32 = 0;

impl Shape {
    fn parse(value: &str) -> Result<Shape> {
        match value {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(eyre!("Could not parse shape: {}", value)),
        }
    }

    /**
     * Returns the `Shape` that gets [result] when played against this shape.
     */
    fn reverse_result(&self, result: &RoundResult) -> Shape {
        match self {
            Shape::Rock => match result {
                RoundResult::Win => Shape::Paper,
                RoundResult::Draw => Shape::Rock,
                RoundResult::Loss => Shape::Scissors,
            },
            Shape::Paper => match result {
                RoundResult::Win => Shape::Scissors,
                RoundResult::Draw => Shape::Paper,
                RoundResult::Loss => Shape::Rock,
            },
            Shape::Scissors => match result {
                RoundResult::Win => Shape::Rock,
                RoundResult::Draw => Shape::Scissors,
                RoundResult::Loss => Shape::Paper,
            },
        }
    }

    /**
     * Score for the shape itself.
     */
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn score_against(&self, other: &Shape) -> u32 {
        match self {
            Shape::Rock => match other {
                Shape::Rock => DRAW_SCORE,
                Shape::Paper => LOSE_SCORE,
                Shape::Scissors => WIN_SCORE,
            },
            Shape::Paper => match other {
                Shape::Rock => WIN_SCORE,
                Shape::Paper => DRAW_SCORE,
                Shape::Scissors => LOSE_SCORE,
            },
            Shape::Scissors => match other {
                Shape::Rock => LOSE_SCORE,
                Shape::Paper => WIN_SCORE,
                Shape::Scissors => DRAW_SCORE,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RoundResult {
    Win,
    Draw,
    Loss,
}

impl RoundResult {
    fn parse(value: &str) -> Result<RoundResult> {
        match value {
            "X" => Ok(RoundResult::Loss),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _ => Err(eyre!("Could not parse result: {}", value)),
        }
    }
}

struct Round {
    player1: Shape,
    player2: Shape,
}

impl Round {
    fn from(vec: &Vec<Shape>) -> Result<Round> {
        Ok(Round {
            player1: vec[0],
            player2: vec[1],
        })
    }

    fn with_second_result(result: RoundResult, p1_shape: Shape) -> Round {
        Round {
            player1: p1_shape,
            player2: p1_shape.reverse_result(&result),
        }
    }

    fn score(&self) -> u32 {
        self.player2.score() + self.player2.score_against(&self.player1)
    }
}

fn parse(input: &str) -> Result<Vec<Round>> {
    input
        .lines()
        .map(|l| {
            let Some((a, b)) = l.trim().split_once(" ") else { return Err(eyre!("Could not parse line: {}", l))};
            let p1 = Shape::parse(a)?;
            let p2 = Shape::parse(b)?;
            Round::from(&vec![p1, p2])
        })
        .collect()
}

fn parse2(input: &str) -> Result<Vec<Round>> {
    input
        .lines()
        .map(|l| {
            let Some((a, b)) = l.trim().split_once(" ") else { return Err(eyre!("Could not parse line: {}", l))};
            let p1 = Shape::parse(a)?;
            let result = RoundResult::parse(b)?;
            Ok(Round::with_second_result(result, p1))
        })
        .collect()
}

fn solve_part1(rounds: &Vec<Round>) -> u32 {
    rounds.iter().map(|r| r.score()).sum()
}

fn solve_part2(rounds: &Vec<Round>) -> u32 {
    rounds.iter().map(|r| r.score()).sum()
}
