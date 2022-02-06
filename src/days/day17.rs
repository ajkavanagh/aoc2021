//--- Day 17: Trick Shot ---

//You finally decode the Elves' message. HI, the message says. You continue searching for the sleigh keys.

//Ahead of you is what appears to be a large ocean trench. Could the keys have fallen into it? You'd better send a probe to investigate.

use std::cmp;
use std::str::FromStr;
use std::collections::HashSet;

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


// part 2; work out the range of (dx,t) combinations that can hit the target.

fn possible_dxs(target: &Target) -> HashSet<(i32, i32)> {  // note that (dx, t) combinations.
    let mut dxs: HashSet<(i32, i32)> = HashSet::new();
    for x in target.min_x..=target.max_x {
        let min_dx = min_dx_for_x(x);
        let mut dxs_x = range_dxs_for(min_dx, x);
        dxs.extend(dxs_x.drain());
    }
    dxs
}


fn min_dx_for_x(x: i32) -> i32 {
    let dx_min_f: f64 = (-1.0 + (-1.0 + 8.0 * (x as f64)).sqrt()) / 2.0;
    let dx_min = dx_min_f.ceil() as i32;
    dx_min
}

// return is (dx, t)
fn range_dxs_for(dx_min: i32, x: i32) -> HashSet<(i32, i32)> {
    let mut dxs: HashSet<(i32, i32)> = HashSet::new();
    for dx in dx_min..=x {
        // note max_t is dx at the max point.
        for t in 1..=dx {
            let xt = t * dx - (t * (t-1))/2;
            if xt > x {
                break;
            }
            if xt == x {
                dxs.insert((dx, t));
            }
        }
    }
    dxs
}

// now for each y between dy0 (maximum +ve y) and min_y (most negative target),
// iterate through them and then look at all of the possible (dx, t) combinations
// to see if they match.
// Returns a list of (dx, dy) for every target hit.
fn find_shots(target: &Target) -> HashSet<(i32, i32)> {
    let mut dxys: HashSet<(i32, i32)> = HashSet::new();
    let max_dy0 = dy0(&target);
    let dx_candidates = possible_dxs(&target);
    for dy in target.min_y..=max_dy0 {
        for (dx, t) in dx_candidates.iter() {
            let yt = calc_yt(dy, *t);
            if yt >= target.min_y && yt <= target.max_y {
                dxys.insert((*dx, dy));
            }
            // try to run t forward in case we have reached the end of dx and the shot is
            // dropping straight downwards.  We'll just record any shots in the target area
            // and let the HashSet remove duplicates.
            let mut t_ = t + 1;
            loop {
                let xt = calc_xt(*dx, t_);
                let yt = calc_yt(dy, t_);
                if xt > target.max_x || yt < target.min_y {
                    break;
                }
                if yt >= target.min_y && yt <= target.max_y && xt >= target.min_x && xt <= target.max_x {
                    dxys.insert((*dx, dy));
                }
                t_ += 1;
            }
        }
    }
    dxys
}


fn calc_yt(dy: i32, t: i32) -> i32 {
    t * dy - ((t * (t-1)) / 2)
}


fn calc_xt(dx: i32, t: i32) -> i32 {
    let t_ = dx.min(t);
    dx * t_ - ((t_ * (t_ - 1)) /2)
}


pub fn day17_1() {
    println!("Day 17: Trick Shot, part 1");
    let lines = utils::read_file_single_result::<String>("./input/day17.txt")
        .expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let input = Target::from_str(&lines[0]).expect("Couldn't decode the target?");
    println!("Input is: {:?}", &input);
    let dy_0 = dy0(&input);
    let max_yt = max_y(dy_0);
    println!("Highest point is: {}", max_yt);
}


fn print_shots(shots: &HashSet<(i32, i32)>) {
    let mut shots = shots.iter().collect::<Vec<_>>();
    shots.sort_by(|a,b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    println!("Shots:");
    let mut dx = shots[0].0 -1;
    for shot in shots.iter() {
        if dx != shot.0 {
            dx = shot.0;
            println!("");
            print!("dx: {:2}, dys: ", shot.0);
        }
        print!("{:3} ", shot.1);
    }
    println!("");
}


pub fn day17_2() {
    println!("Day 17: Trick Shot, part 2");
    let lines = utils::read_file_single_result::<String>("./input/day17.txt")
        .expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let input = Target::from_str(&lines[0]).expect("Couldn't decode the target?");
    println!("Input is: {:?}", &input);
    let shots = find_shots(&input);
    //println!("Calculated shots:");
    //print_shots(&shots);
    println!("count of shots: {}", shots.len());
}
