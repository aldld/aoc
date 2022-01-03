use std::str::FromStr;

use crate::types::{Input, UnitResult};

pub fn part1(_input: Input) -> UnitResult {
    Ok(())
}

pub fn part2(input: Input) -> UnitResult {
    let mut depth: u32 = 0;
    let mut pos: u32 = 0;
    let mut aim: u32 = 0;

    for line in input {
        let step: Step = line?.parse()?;
        match step {
            Step::Forward(amount) => {
                pos += amount;
                depth += aim * amount;
            },
            Step::Down(amount) => {
                aim += amount;
            },
            Step::Up(amount) => {
                aim -= amount;
            },
        }
    }

    println!("depth={}, pos={}, depth*pos={}", depth, pos, depth * pos);

    Ok(())
}

#[derive(Debug)]
enum Step {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Step {
    type Err = &'static str; // Whatever...

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(' ').and_then(|(direction, amount)| {
            let amount: u32 = amount.parse().ok()?;
            match direction {
                "forward" => Some(Step::Forward(amount)),
                "down" => Some(Step::Down(amount)),
                "up" => Some(Step::Up(amount)),
                _ => None,
            }
        }).ok_or_else(|| "Failed to parse")
    }
}

