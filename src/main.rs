use std::{env, error::Error, io::{Lines, self, BufReader, BufRead}, path::Path, fs::File};

use types::{Input, UnitResult};

mod years;
mod types;

fn main() -> Result<(), Box<dyn Error>> {
    // aoc <year> <day> <part>
    let args: Vec<String> = env::args().collect();

    let year = &args[1];
    let day: u32 = args[2].parse()?;
    let part: u32 = args[3].parse()?;

    run_solution(year, day, part)?;

    Ok(())
}

fn day_str(day: u32) -> String {
    format!("day{}", day)
}

fn load_input(year: &str, day: u32) -> io::Result<Lines<BufReader<File>>> {
    let path = Path::new("inputs/years/").join(year).join(day_str(day));
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

fn run_solution(year: &str, day: u32, part: u32) -> Result<(), Box<dyn Error>> {
    let input = Box::new(load_input(&year, day)?);

    let solns2021: Vec<Vec<fn(Input) -> UnitResult>> = vec![
        vec![years::y2021::template::part1, years::y2021::template::part2],
        vec![years::y2021::day1::part1, years::y2021::day1::part2],
        vec![years::y2021::day2::part1, years::y2021::day2::part2],
        vec![years::y2021::day3::part1, years::y2021::day3::part2],
    ];

    let soln = solns2021[day as usize][(part - 1) as usize];

    soln(input)?;

    Ok(())
}
