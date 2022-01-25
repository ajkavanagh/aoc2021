//--- Day 12: Passage Pathing ---

//With your submarine's subterranean subsystems subsisting suboptimally, the only way you're getting out of this cave anytime soon is by finding a path yourself. Not just a path - the only way to know if you've found the best path is to find all of them.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

use thiserror::Error;

use crate::utils;

#[derive(Error, Debug, Clone)]
pub enum Day12Error {
    #[error("Invalid command")]
    InvalidError(String),
}


use Day12Error::*;


#[derive(Error, Clone, Debug)]
struct Line {
    start: String,
    end: String,
}


#[derive(Clone, Debug)]
struct Network(HashMap<String, HashSet<String>>);


impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl FromStr for Line {
    type Err = Day12Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts = line.split("-").into_iter().collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(InvalidError(format!("line '{}' isn't a valid start-finish", line)));
        }
        Ok(Self {start: parts[0].to_string(), end: parts[1].to_string() })
    }

}


fn load_network(lines: &[Line]) -> Network {
    let mut network: HashMap<String, HashSet<String>> = HashMap::new();
    for line in lines.iter() {
        (*network.entry(line.start.clone()).or_insert(HashSet::new())).insert(line.end.clone());
        (*network.entry(line.end.clone()).or_insert(HashSet::new())).insert(line.start.clone());
    }
    Network(network)
}


fn is_small_cave(cave: &String) -> bool {
    cave.chars().take(1).all(|c| c.is_lowercase())
}


fn all_paths(network: &Network) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = Vec::new();
    let node: String = String::from("start");
    let mut partials: Vec<Vec<String>> = vec![vec![node.clone()]];
    let mut been_there: Vec<Vec<String>> = vec![];
    while let Some(stack) = partials.pop() {
        // stack is a list of paths (reversed) starting with the end point.
        // peak the top of the stack and then grab a list of neighbours
        // and then make sure it's not a partial we already have and
        // that we've not visited a small cave twice.
        if let Some(head) = stack.get(stack.len()-1) {
            if let Some(set) = network.0.get(head) {
                for next in set.iter() {
                    let mut next_stack = stack.clone();
                    if is_small_cave(&next) && next_stack.contains(&next) {
                        continue;
                    }
                    next_stack.push(next.clone());
                    if next == "end" {
                        paths.push(next_stack);
                    } else if !been_there.contains(&next_stack) {
                        been_there.push(next_stack.clone());
                        partials.push(next_stack);
                    }
                }
            } else {
                println!("network at {} contained no neighbours?", &head);
                panic!("Something very wrong went.");
            }
        } else {
            panic!("stack didn't have a head, wierd!");
        }
    }
    paths
}


pub fn day12_1() {
    println!("Day 12: Passage Pathing, part 1");
    let lines = utils::read_file_single_result::<Line>("./input/day12.txt").expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let network = load_network(&lines);
    println!("network is: {:?}", &network);
    let all_paths = all_paths(&network);
    println!("Num paths: {:?}", all_paths.len());
}
