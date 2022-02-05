//--- Day 17: Trick Shot ---

//You finally decode the Elves' message. HI, the message says. You continue searching for the sleigh keys.

//Ahead of you is what appears to be a large ocean trench. Could the keys have fallen into it? You'd better send a probe to investigate.

use std::fmt;
use std::cmp;
use std::str::FromStr;

use thiserror::Error;

use crate::utils;


#[derive(Error, Debug, Clone)]
pub enum DecodeError {
    #[error("Invalid command")]
    InvalidError(String),
}

#[derive(Clone, Debug)]
struct Target {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}


impl FromStr for Target {
    type Err = DecodeError;

    // Decode the following string into a Target.
    // "target area: x=20..30, y=-10..-5"
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts = line.split(": ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(DecodeError::InvalidError(
                format!("Doesn't have ': ' in {}", line)));
        }
        if parts[0] != "target area" {
            return Err(DecodeError::InvalidError(
                format!("Doesn't start with 'target area' in {}", line)));
        }
        let xy = parts[1].split(", ").collect::<Vec<_>>();
        if xy.len() != 2 {
            return Err(DecodeError::InvalidError(
                format!("Doesn't have ', ' in {}", line)));
        }
        // deal with x=n..m
        let (cx, min_x, max_x) = decode_bounds(xy[0])?;
        if cx != 'x' {
            return Err(DecodeError::InvalidError(
                format!("first section isn't 'x' in {}", line)));
        }
        let (cy, min_y, max_y) = decode_bounds(xy[1])?;
        if cy != 'y' {
            return Err(DecodeError::InvalidError(
                format!("second section isn't 'y' in {}", line)));
        }
        Ok(Self {min_x, max_x, min_y, max_y})
    }
}


// Decode the following string into a char, min i32, max i32
// "x=20..30"
fn decode_bounds(section: &str) -> Result<(char, i32, i32), DecodeError> {
    let lr = section.split("=").collect::<Vec<_>>();
    if lr.len() != 2 {
        return Err(DecodeError::InvalidError(
            format!("No '=' in section {}", section)));
    }
    if lr[0].chars().count() != 1 {
        return Err(DecodeError::InvalidError(
            format!("Part before '=' not a single char in section {}", section)));
    }
    // now decode lr[1]
    let nums = lr[1].split("..").collect::<Vec<_>>();
    if nums.len() != 2 {
        return Err(DecodeError::InvalidError(
            format!("Missing '..' section {}", section)));
    }
    let n1 = nums[0].parse::<i32>()
        .map_err(|e| DecodeError::InvalidError(format!("{} not a number: {} ? in section {}", nums[0], e, section)))?;
    let n2 = nums[1].parse::<i32>()
        .map_err(|e| DecodeError::InvalidError(format!("{} not a number: {} ? in section {}", nums[1], e, section)))?;
    Ok((lr[0].chars().collect::<Vec<_>>()[0], cmp::min(n1, n2), cmp::max(n1, n2)))
}

// part1, just calculations
// See notebook for details of the equations!

fn dy0(target: &Target) -> i32 {
    -(target.min_y) - 1
}


fn max_y(dy0: i32) -> i32 {
    (dy0 * (dy0 + 1))/2
}


pub fn day17_1() {
    println!("Day 17: Trick Shot, part 1");
    let lines = utils::read_file_single_result::<String>("./input/day17.txt")
        .expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    //let input = U32Words::parse_lines(&lines).expect("Couldn't decode words");
    let input = Target::from_str(&lines[0]).expect("Couldn't decode the target?");
    println!("Input is: {:?}", &input);
    let dy_0 = dy0(&input);
    let max_yt = max_y(dy_0);
    println!("Highest point is: {}", max_yt);
}
