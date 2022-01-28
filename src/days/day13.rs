//--- Day 13: Transparent Origami ---

//You reach another volcanically active part of the cave. It would be nice if you could do some kind of thermal imaging so you could tell ahead of time which caves are too hot to safely enter.

use std::fmt;
use std::collections::HashSet;
use crate::utils;


#[derive(Debug, Clone)]
enum Fold {
    Down,
    Across,
}

use Fold::*;

//type Dots = Vec<(i32, i32)>;
//type Folds = Vec<(Fold, i32)>;


#[derive(Debug, Clone)]
struct Paper {
    dots: Vec<(i32, i32)>,
    folds: Vec<(Fold, i32)>,
}


fn parse<S>(lines: &[S]) -> Result<Paper, String>
    where S: AsRef<str> + fmt::Display
{
    let mut dots: Vec<(i32, i32)> = Vec::new();
    let mut folds: Vec<(Fold, i32)> = Vec::new();
    let mut line_iter = lines.iter();

    while let Some(line) = line_iter.next() {
        let line = line.as_ref().trim();
        if line == "" {
            break;
        }
        let parts = line.split(",").into_iter().collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(format!("Location '{}' isn't a valid down-across", &line));
        }
        let down = parts[0].parse::<i32>().or(Err(format!("Couldn't parse {}", &parts[0])))?;
        let across = parts[1].parse::<i32>().or(Err(format!("Couldn't parse {}", &parts[1])))?;
        dots.push((down, across));
    }
    while let Some(line) = line_iter.next() {
        let parts = line.as_ref().split(" ").into_iter().collect::<Vec<&str>>();
        if parts.len() != 3 {
            return Err(format!("Fold '{}' isn't a valid", &line));
        }
        if parts[0] != "fold" || parts[1] != "along" {
            return Err(format!("Fold '{}' isn't a valid", &line));
        }
        let fold_bits = parts[2].split("=").into_iter().collect::<Vec<&str>>();
        if fold_bits.len() != 2 {
            return Err(format!("Fold spec is not parsable? {}", &parts[2]));
        }
        let orientation = match fold_bits[0] {
            "x" => Ok(Down),
            "y" => Ok(Across),
            _ => Err(format!("Orientation {} is not known", &fold_bits[0])),
        }?;
        let location = fold_bits[1].parse::<i32>()
            .or(Err(format!("Couldn't parse to number {} => {}", &line, &fold_bits[1])))?;
        folds.push((orientation, location));
    }
    Ok(Paper {dots, folds})
}


fn mirror(point: i32, fold_at: i32) -> Option<i32> {
    if point == fold_at {
        None
    } else if point < fold_at {
        Some(point)
    } else {
        let new_point = fold_at - (point - fold_at);
        if new_point >= 0 {
            Some(new_point)
        } else {
            None
        }
    }
}

// fold the dots along the fold.
// Essentially, mirror them and cut off any that go negative.
fn fold(dots: &Vec<(i32, i32)>, (orientation, location): &(Fold, i32)) -> HashSet<(i32, i32)> {
    let mut new_dots: HashSet<(i32, i32)> = HashSet::new();
    for &(down, across) in dots.iter() {
        if let Some((new_down, new_across)) = match orientation {
            Down => mirror(down, *location).and_then(|new_down| Some((new_down, across))),
            Across => mirror(across, *location).and_then(|new_across| Some((across, new_across))),
        } {
            new_dots.insert((new_down, new_across));
        }
    }
    new_dots
}


pub fn day13_1() {
    println!("Day 13: Transparent Origami, part 1");
    let lines = utils::read_file_single_result::<String>("./input/day13.txt")
        .expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let paper = parse(&lines).expect("Couldn't parse??");
    println!("Paper: {:?}", &paper);
    let new_dots = fold(&paper.dots, &paper.folds[0]);
    println!("Number of dots: {}", new_dots.len());
}
