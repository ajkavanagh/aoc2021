//--- Day 7: The Treachery of Whales ---

//A giant whale has decided your submarine is its next meal, and it's much faster than you are. There's nowhere to run!

//Suddenly, a swarm of crabs (each in its own tiny submarine - it's too deep for them otherwise) zooms in to rescue you! They seem to be preparing to blast a hole in the ocean floor; sensors indicate a massive underground cave system just beyond where they're aiming!

//The crab submarines all need to be aligned before they'll have enough power to blast a large enough hole for your submarine to get through. However, it doesn't look like they'll be aligned before the whale catches you! Maybe you can help?

//There's one major catch - crab submarines can only move horizontally.

//You quickly make a list of the horizontal position of each crab (your puzzle input). Crab submarines have limited fuel, so you need to find a way to make all of their horizontal positions match while requiring them to spend as little fuel as possible.

//For example, consider the following horizontal positions:

//16,1,2,0,4,2,7,1,2,14

//This means there's a crab with horizontal position 16, a crab with horizontal position 1, and so on.

//Each change of 1 step in horizontal position of a single crab costs 1 fuel. You could choose any horizontal position to align them all on, but the one that costs the least fuel is horizontal position 2:

    //Move from 16 to 2: 14 fuel
    //Move from 1 to 2: 1 fuel
    //Move from 2 to 2: 0 fuel
    //Move from 0 to 2: 2 fuel
    //Move from 4 to 2: 2 fuel
    //Move from 2 to 2: 0 fuel
    //Move from 7 to 2: 5 fuel
    //Move from 1 to 2: 1 fuel
    //Move from 2 to 2: 0 fuel
    //Move from 14 to 2: 12 fuel

//This costs a total of 37 fuel. This is the cheapest possible outcome; more expensive outcomes include aligning at position 1 (41 fuel), position 3 (39 fuel), or position 10 (71 fuel).

//Determine the horizontal position that the crabs can align to using the least fuel possible. How much fuel must they spend to align to that position?


use std::num::ParseIntError;

use crate::utils;


fn parse_line<S: AsRef<str>>(line: S) -> Result<Vec<u32>, ParseIntError>  {
    line.as_ref().split(",")
        .map(|l| l.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
}

fn find_min_fuel(positions: &Vec<u32>, part2: bool) -> u32 {
    let mut max_pos: u32 = 0;
    let mut min_pos: u32 = u32::MAX;
    for &p in positions.iter() {
        if p > max_pos {
            max_pos = p;
        }
        if p < min_pos {
            min_pos = p;
        }
    }
    let mut min_fuel = u32::MAX;
    for p in min_pos..=max_pos {
        let fuel_used = calc_offsets(&positions, p, part2);
        if fuel_used < min_fuel {
            println!("Found a new minimum: {} at {}", fuel_used, p);
            min_fuel = fuel_used;
        }
    }
    min_fuel
}


/// calc_offsets; part2 switches on the increased cost of moving for part2
fn calc_offsets(positions: &Vec<u32>, pos: u32, part2: bool) -> u32 {
    let mut total: u32 = 0;
    for &p in positions.iter() {
        let distance = if p >= pos {
            p - pos
        } else {
            pos - p
        };
        if !part2 {
            total += distance;
        } else {
            // 1 + 2 + 3 ...  = n(n + 1)/2
            total += (distance * (distance + 1)) / 2;
        }
    }
    total
}

pub fn day7_1() {
    println!("Day 7-1: The Treachery of Whales");
    let lines = utils::read_file_single_result::<String>("./input/day07.txt")
        .expect("Error reading the file");
    if lines.len() != 1 {
        println!("Input file has either no lines or more than 1: {}", lines.len());
        return;
    }
    let positions = parse_line(&lines[0]).expect("Error parsing ints");
    let min_fuel = find_min_fuel(&positions, false);
    println!("Min fuel for {:?} is {}", positions, min_fuel);
}

pub fn day7_2() {
    println!("Day 7-2: The Treachery of Whales - exponential fuel");
    let lines = utils::read_file_single_result::<String>("./input/day07.txt")
        .expect("Error reading the file");
    if lines.len() != 1 {
        println!("Input file has either no lines or more than 1: {}", lines.len());
        return;
    }
    let positions = parse_line(&lines[0]).expect("Error parsing ints");
    let min_fuel = find_min_fuel(&positions, true);
    println!("Min fuel for {:?} is {}", positions, min_fuel);
}
