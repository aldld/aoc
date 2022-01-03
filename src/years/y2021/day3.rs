use std::collections::HashSet;

use crate::types::{Input, UnitResult};

pub fn part1(input: Input) -> UnitResult {
    let mut lines = input.peekable();
    let bit_width = lines.peek().unwrap().as_ref().unwrap().len();

    let mut counts: Vec<i32> = vec![0; bit_width];

    for line in lines {
        let line = line?;

        for (idx, bit) in line.chars().enumerate() {
            if bit == '1' {
                counts[idx] += 1;
            } else {
                counts[idx] -= 1;
            }
        }
    }

    let (gamma_rate, epsilon_rate) =
        counts.iter().rev().enumerate().map(|(pow, value)| {
            if *value >= 0 {
                (1 << pow, 0)
            } else {
                (0, 1 << pow)
            }
        }).fold((0, 0), |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2));

    println!("{}", gamma_rate * epsilon_rate);

    Ok(())
}

pub fn part2(input: Input) -> UnitResult {
    let values: HashSet<String> = input.map(Result::unwrap).collect();
    let o2_rating = oxygen_generator_rating(values.clone()) as u64;
    let co2_rating = co2_scrubber_rating(values) as u64;

    println!("O2 {}, CO2 {}", o2_rating, co2_rating);
    println!("{}", o2_rating * co2_rating);

    Ok(())
}

fn oxygen_generator_rating(mut values: HashSet<String>) -> u32 {
    let mut bit_idx = 0;
    while values.len() > 1 {
        let most_common = most_common_bit(&values, bit_idx);
        values.retain(|val| val.as_bytes()[bit_idx] == most_common as u8);

        bit_idx += 1;
    }

    let bit_str = values.iter().next().unwrap();
    println!("o2 {}", bit_str);
    bits_to_u32(bit_str)
}

fn co2_scrubber_rating(mut values: HashSet<String>) -> u32 {
    let mut bit_idx = 0;
    while values.len() > 1 {
        let most_common = most_common_bit(&values, bit_idx);
        values.retain(|val| val.as_bytes()[bit_idx] != most_common as u8);

        bit_idx += 1;
    }

    let bit_str = values.iter().next().unwrap();
    println!("co2 {}", bit_str);
    bits_to_u32(bit_str)
}

fn bits_to_u32(bit_str: &str) -> u32 {
    bit_str
        .bytes()
        .rev()
        .enumerate()
        .map(|(pow, bit)| if bit == '1' as u8 { 1 << pow } else { 0 })
        .sum()
}

fn most_common_bit(values: &HashSet<String>, bit_idx: usize) -> char {
    let count: i32 = values
        .iter()
        .map(|val| val.as_bytes()[bit_idx])
        .map(|bit| if bit == '1' as u8 { 1 } else { -1 })
        .sum();

    if count >= 0 {
        '1'
    } else {
        '0'
    }
}
