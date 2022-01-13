// --- Day 5: Hydrothermal Venture ---

//You come across a field of hydrothermal vents on the ocean floor! These vents constantly produce large, opaque clouds, so it would be best to avoid them if possible.


use std::str::FromStr;
use std::ops::Add;
use std::ops::Sub;
use std::collections::HashMap;

use crate::utils;


use thiserror::Error;


// we need to decode the data as:
//
// 1. line of drawn numbers, split by ','
// 2. A blank line.
// 3. 5 lines of numbers split by spaces; 5 numbers in a row.

#[derive(Error, Debug, Clone)]
pub enum DecodeError {
    #[error("Invalid command")]
    InvalidError(String),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PointXY {
    x: i32,
    y: i32,
}

impl Add for PointXY {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


impl Sub for PointXY {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}


impl FromStr for PointXY {
    type Err = DecodeError;

    // parse "6,4" into a PointXY
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let vec = line.split(",").collect::<Vec<_>>();
        if vec.len() != 2 {
            return Err(DecodeError::InvalidError(format!("Line part doesn't look like (x,y): {}", line)));
        }
        let x = vec[0].parse::<i32>()
            .map_err(|e| DecodeError::InvalidError(format!("invalid tuple?: {} in {}", e, line)))?;
        let y = vec[1].parse::<i32>()
            .map_err(|e| DecodeError::InvalidError(format!("invalid tuple?: {} in {}", e, line)))?;
        Ok(Self {x, y})
    }
}

#[derive(Clone, Debug)]
struct Line {
    start: PointXY,
    end: PointXY,
}


impl FromStr for Line {
    type Err = DecodeError;

    // parse "6,4 -> 2,0" into a Line.
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(DecodeError::InvalidError(format!("Two many or few parts in line: {}", line)));
        }
        if parts[1] != "->" {
            return Err(DecodeError::InvalidError(format!("Missing '->' in input?: {}", line)));
        }
        let start = parts[0].parse::<PointXY>()?;
        let end = parts[2].parse::<PointXY>()?;
        Ok(Self {start, end})
    }
}


impl Line {
    fn draw_iter(&self) -> LineIterator {
        LineIterator::new(&self)
    }
}

#[derive(Clone, Debug)]
struct LineIterator {
    end: PointXY,
    dx: i32,
    dy: i32,
    sx: i32,
    sy: i32,
    err: i32,
    pos: Option<PointXY>,
}

// drawing algorithm from https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
impl LineIterator {

    fn new(line: &Line) -> Self {
        let dx = (line.end.x-line.start.x).abs();
        let dy = -(line.end.y-line.start.y).abs();
        let sx = if line.start.x < line.end.x { 1 } else { -1 };
        let sy = if line.start.y < line.end.y { 1 } else { -1 };
        let err = dx + dy;
        Self {end: line.end, dx, dy, sx, sy, err, pos: Some(line.start) }
    }
}
// implement iter() fo Line and then Iterator trait for LineIterator
impl Iterator for LineIterator {
    type Item = PointXY;

    fn next(&mut self) -> Option<Self::Item> {
        // if self.pos is Some(then we may be finishing).
        if let Some(pos) = self.pos {
            if pos == self.end {
                self.pos = None;
                return Some(pos);
            }
            let mut next_pos = pos;
            // otherwise, let's plot the next point
            let err = 2 * self.err;
            if err >= self.dy {
                self.err += self.dy;
                next_pos.x += self.sx;
            }
            if err <= self.dx {
                self.err += self.dx;
                next_pos.y += self.sy;
            }
            self.pos = Some(next_pos);
            return Some(pos);
        }
        None
    }
}

type Map = HashMap<(i32,i32), u32>;


// draw a line, assuming that the start -> end is horiz or vertical.
fn draw_line(line: &Line, points: &mut Map) {
    //println!("Line: {:?}", line);
    for p in line.draw_iter() {
        let (x, y) = (p.x, p.y);
        *points.entry((x,y)).or_insert(0) += 1;
    }
}

pub fn day5_1() {
    println!("Day 5: Hydrothermal Venture");
    let read_lines = utils::read_file::<Line>("./input/day05.txt");
    let lines = read_lines.iter().cloned().collect::<Result<Vec<_>, _>>().expect("Failed to read file");
    println!("Number lines: {}", lines.len());
    let mut points = Map::new();
    for line in lines.iter() {
        let (dx, dy) = (line.end.x - line.start.x, line.end.y - line.start.y);
        if dx != 0 && dy != 0 {
            println!("Ignoring line: {:?} as it is diagonal", line);
        } else {
            draw_line(line, &mut points);
        }
    }
    // now count up the number of points in the hashmap that are more than 1.
    //println!("points: {:?}", points);
    let num = points.values().filter(|&v| *v > 1).count();
    println!("Found {} points that overlap.", num);
}

pub fn day5_2() {
    println!("Day 5: Hydrothermal Venture - diagonal lines");
    let read_lines = utils::read_file::<Line>("./input/day05.txt");
    let lines = read_lines.iter().cloned().collect::<Result<Vec<_>, _>>().expect("Failed to read file");
    println!("Number lines: {}", lines.len());
    let mut points = Map::new();
    for line in lines.iter() {
        draw_line(line, &mut points);
    }
    // now count up the number of points in the hashmap that are more than 1.
    //println!("points: {:?}", points);
    let num = points.values().filter(|&v| *v > 1).count();
    println!("Found {} points that overlap.", num);
}
