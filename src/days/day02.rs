//--- Day 2: Dive! ---

//Now, you need to figure out how to pilot this thing.

//It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:

    //forward X increases the horizontal position by X units.
    //down X increases the depth by X units.
    //up X decreases the depth by X units.

//Note that since you're on a submarine, down and up affect your depth, and so they have the
//opposite result of what you might expect.

use std::str::FromStr;

use crate::utils;


use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CommandError {
    #[error("Can't decode line")]
    DecodeError(String),
    #[error("Invalid command")]
    InvalidError(String),
}


#[derive(Debug, Clone)]
pub enum Direction {
    Forward,
    Down,
    Up,
}


#[derive(Debug, Clone)]
pub struct Command {
    direction: Direction,
    value: u32,
}


impl FromStr for Command {
    type Err = CommandError;

    fn from_str(cmd: &str) -> Result<Self, Self::Err> {
        let parts = cmd.split(" ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(CommandError::DecodeError(format!("Missing space in passed string: {}", cmd)));
        }
        let command = match parts[0] {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(CommandError::InvalidError(format!("Command is not one of 'forward', 'down', or 'up'"))),
        }?;
        let value = parts[1].parse::<u32>().map_err(|s| CommandError::InvalidError(format!("Command value didn't decode to u32: {} - {}", parts[1], s)))?;
        Ok(Self {direction: command, value})
    }
}


fn determine_value(commands: &[Command]) -> u32 {
    let mut f: u32 = 0;
    let mut d: u32 = 0;
    for command in commands.iter() {
        match command.direction {
            Direction::Forward => { f += command.value; },
            Direction::Up => { d -= command.value; },
            Direction::Down => { d += command.value; },
        };
    }
    f * d
}


fn determine_value_with_aim(commands: &[Command]) -> u32 {
    let mut f: u32 = 0;
    let mut d: u32 = 0;
    let mut aim: u32 = 0;
    for command in commands.iter() {
        match command.direction {
            Direction::Forward => { f += command.value; d += aim * command.value },
            Direction::Up => { aim -= command.value; },
            Direction::Down => { aim += command.value; },
        };
    }
    f * d
}


pub fn day2_1() {
    println!("Day2-1, using the test data.");
    println!("'forward 10' {:?}", "forward 10".parse::<Command>());
    println!("'up 22' {:?}", "up 22".parse::<Command>());
    println!("'down 1' {:?}", "down 1".parse::<Command>());
    println!("Now get the test data");

    let parsed_commands = utils::read_file::<Command>("./input/day02-test.txt");
    //println!("{:?}", parsed_commands);
    match parsed_commands.iter().cloned().collect::<Result<Vec<Command>, _>>() {
        Ok(commands) => {
            println!("{:?}", commands);
            println!("Result value is: {:?}", determine_value(&commands));
        },
        Err(s) => println!("Parsing failed: {}", s),
    };

    println!("Now do it with the actual day 2 data.");
    let parsed_commands = utils::read_file::<Command>("./input/day02.txt");
    match parsed_commands.iter().cloned().collect::<Result<Vec<Command>, _>>() {
        Ok(commands) => {
            println!("Result value is: {:?}", determine_value(&commands));
        },
        Err(s) => println!("Parsing failed: {}", s),
    };
}


pub fn day2_2() {
    println!("Day2-1, using the test data.");
    println!("Now get the test data");

    let parsed_commands = utils::read_file::<Command>("./input/day02-test.txt");
    //println!("{:?}", parsed_commands);
    match parsed_commands.iter().cloned().collect::<Result<Vec<Command>, _>>() {
        Ok(commands) => {
            println!("{:?}", commands);
            println!("Result value is: {:?}", determine_value_with_aim(&commands));
        },
        Err(s) => println!("Parsing failed: {}", s),
    };

    println!("Now do it with the actual day 2 data.");
    let parsed_commands = utils::read_file::<Command>("./input/day02.txt");
    match parsed_commands.iter().cloned().collect::<Result<Vec<Command>, _>>() {
        Ok(commands) => {
            println!("Result value is: {:?}", determine_value_with_aim(&commands));
        },
        Err(s) => println!("Parsing failed: {}", s),
    };
}
