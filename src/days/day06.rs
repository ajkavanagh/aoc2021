// --- Day 6: Lanternfish ---

// The sea floor is getting steeper. Maybe the sleigh keys got carried this way?

// A massive school of glowing lanternfish swims past. They must spawn quickly to reach such large numbers - maybe exponentially quickly? You should model their growth rate to be sure.

use std::num::ParseIntError;

use crate::utils;


type Fish = [u64; 9];


fn parse_fish<S: AsRef<str>>(line: S) -> Result<Fish, String> {
    let fishes = line.as_ref()
        .split(",")
        .map(|l| l.parse::<usize>()).collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Parser error: {}", e))?;
    println!("Fishes = {:?}", fishes);

    let mut fish: Fish = [0; 9];
    for f in fishes {
        if f > 8 {
            return Err(format!("A fish's age was not between 0 and 8: {}", f));
        }
        fish[f] += 1;
    }
    Ok(fish)
}


// age a generation of fish
fn age_fish(gen: &Fish) -> Fish {
    let mut next_gen: Fish = [0; 9];
    for i in 1..=8 {
        next_gen[i-1] = gen[i];
    }
    next_gen[8] = gen[0];
    next_gen[6] += gen[0];
    next_gen
}

pub fn day6_1() {
    println!("Day 5: Hydrothermal Venture");
    let read_lines = utils::read_file::<String>("./input/day06.txt");
    if read_lines.len() != 1 {
        println!("Input file has either no lines or more than 1: {}", read_lines.len());
        return;
    }
    let lines = read_lines.iter().cloned().collect::<Result<Vec<_>, _>>().expect("Failed to read file");
    let mut fish = parse_fish(&lines[0]).expect("Failed to parse fish");
    println!("Fish are: {:?}", fish);
    for i in 1..=80 {
        fish = age_fish(&fish);
        if i == 18  || i == 80 {
            println!("Day {}, fish = {:?}, total = {}", i, fish, fish.iter().sum::<u64>());
        }
    }
}


pub fn day6_2() {
    println!("Day 5: Hydrothermal Venture");
    let read_lines = utils::read_file::<String>("./input/day06.txt");
    if read_lines.len() != 1 {
        println!("Input file has either no lines or more than 1: {}", read_lines.len());
        return;
    }
    let lines = read_lines.iter().cloned().collect::<Result<Vec<_>, _>>().expect("Failed to read file");
    let mut fish = parse_fish(&lines[0]).expect("Failed to parse fish");
    println!("Fish are: {:?}", fish);
    for i in 1..=256 {
        fish = age_fish(&fish);
        if i == 18  || i == 80 || i == 256{
            println!("Day {}, fish = {:?}, total = {}", i, fish, fish.iter().sum::<u64>());
        }
    }
}
