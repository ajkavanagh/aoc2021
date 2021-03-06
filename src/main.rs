use std::env;
//use std::cmp;
use std::process;
//use std::io::prelude::*;
//use std::fs::OpenOptions;
//use std::fs;

mod days;
mod utils;

struct Config {
    day: u32,
    part: u32,
}


const MAX_DAY: u32 = 25;  // update when we add a day

impl Config {

    fn new(args: &[String]) -> Result<Config, String> {
        let num_args = args.len();
        if num_args == 1 {
            return Ok(Config {day: 1, part: 1})
        }
        let command = args[1].to_lowercase();
        let parts = command.split("-").into_iter().collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(format!("command '{}' isn't a valid day-part", command));
        }
        let day: u32 = parts[0].parse().unwrap_or(0);
        let part: u32 = parts[1].parse().unwrap_or(0);
        if day < 1 || day > MAX_DAY || part < 1 || part > 2 {
            return Err(format!("day or part is not parsable as an int or not in range: input was '{}'", command));
        }
        return Ok(Config{day, part});
    }
}



fn usage() -> Result<(), String> {
    eprintln!("Usage: aoc2021 <day>-<part>");
    Ok(())
}


fn run_day_part(day: u32, part: u32) {
    match (day, part) {
        (1,1) => days::day01::day1_1(),
        (1,2) => days::day01::day1_2(),
        (2,1) => days::day02::day2_1(),
        (2,2) => days::day02::day2_2(),
        (3,1) => days::day03::day3_1(),
        (3,2) => days::day03::day3_2(),
        (4,1) => days::day04::day4_1(),
        (4,2) => days::day04::day4_2(),
        (5,1) => days::day05::day5_1(),
        (5,2) => days::day05::day5_2(),
        (6,1) => days::day06::day6_1(),
        (6,2) => days::day06::day6_2(),
        (7,1) => days::day07::day7_1(),
        (7,2) => days::day07::day7_2(),
        (8,1) => days::day08::day8_1(),
        (8,2) => days::day08::day8_2(),
        (9,1) => days::day09::day9_1(),
        (9,2) => days::day09::day9_2(),
        (10,1) => days::day10::day10_1(),
        (10,2) => days::day10::day10_2(),
        (11,1) => days::day11::day11_1(),
        (11,2) => days::day11::day11_2(),
        (12,1) => days::day12::day12_1(),
        (12,2) => days::day12::day12_2(),
        (13,1) => days::day13::day13_1(),
        (13,2) => days::day13::day13_2(),
        (14,1) => days::day14::day14_1(),
        (14,2) => days::day14::day14_2(),
        (15,1) => days::day15::day15_1(),
        (15,2) => days::day15::day15_2(),
        (16,1) => days::day16::day16_1(),
        (16,2) => days::day16::day16_2(),
        (17,1) => days::day17::day17_1(),
        (17,2) => days::day17::day17_2(),
        (18,1) => days::day18::day18_1(),
        (18,2) => days::day18::day18_2(),
        _ => println!("Day {0}-{1} not defined (yet?)", day, part),
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Couldn't parse arguments: {}", err);
        usage().unwrap();
        process::exit(1);
    });
    println!("the day is {}-{}", config.day, config.part);
    run_day_part(config.day, config.part);
}
