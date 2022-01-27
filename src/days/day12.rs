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

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (item, set) in self.0.iter() {
            write!(f, "\n{:<5} = {}",
                item,
                set.iter().cloned().collect::<Vec<_>>().as_slice().join(", "))?;
        }
        write!(f, "\n")
    }
}


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


fn all_paths_part1(network: &Network) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = Vec::new();
    let node: String = String::from("start");
    let mut partials: Vec<Vec<String>> = vec![vec![node.clone()]];
    //let mut been_there: Vec<Vec<String>> = vec![];
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
                    //} else if !been_there.contains(&next_stack) {
                    } else {
                        //been_there.push(next_stack.clone());
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


#[derive(Debug, Clone)]
struct CaveBinMaps {
    string_to_u32: HashMap<String, u32>,
    u32_to_string: HashMap<u32, String>,
}


fn make_cave_bin_maps(network: &Network) -> CaveBinMaps {
    let mut string_to_u32: HashMap<String, u32> = HashMap::new();
    let mut u32_to_string: HashMap<u32, String> = HashMap::new();
    for (i, cave) in network.0.keys().enumerate() {
        let index = if is_small_cave(cave) {
            i as u32
        } else {
            (i + 1000) as u32
        };
        string_to_u32.insert(cave.clone(), index);
        u32_to_string.insert(index, cave.clone());
    }
    CaveBinMaps {string_to_u32, u32_to_string}
}


#[derive(Clone, Debug)]
struct Partial {
    visited_caves: HashSet<u32>,
    twice: bool,
    path: Vec<u32>,
}

impl Partial {
    fn new(cave: u32) -> Self {
        Partial {
            visited_caves: HashSet::from([cave]),
            twice: false,
            path: vec![cave],
        }
    }
}


#[derive(Clone, Debug)]
struct U32Network(HashMap<u32, HashSet<u32>>);

fn make_u32_network(network: &Network, cave_bin_map: &CaveBinMaps) -> U32Network {
    let mut u32network: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (cave, set) in network.0.iter() {
        let bin_cave = cave_bin_map.string_to_u32.get(cave).unwrap();
        u32network.insert(
            *bin_cave,
            set.iter().map(|c| *cave_bin_map.string_to_u32.get(c).unwrap()).collect::<HashSet<_>>());
    }
    U32Network(u32network)
}

// like part 1, but small caves with a single connection can be visited once, and other
// small caves can be visted twice
fn all_paths_part2(network: &Network) -> Vec<Vec<String>> {
    let cave_bin_map = make_cave_bin_maps(&network);
    let u32network = make_u32_network(&network, &cave_bin_map);
    let mut paths: Vec<Vec<u32>> = Vec::new();
    let start_cave: u32 = *cave_bin_map.string_to_u32.get("start").unwrap();
    let end_cave: u32 = *cave_bin_map.string_to_u32.get("end").unwrap();
    let mut partials: Vec<Partial> = vec![Partial::new(start_cave)];
    println!("Start cave is: {}", &start_cave);
    println!("End cave is: {}", &end_cave);
    while let Some(partial) = partials.pop() {
        //println!("Partial is {:?}", &partial);
        // partial is a current visited path.
        // peak at the end of the current partial path to work out where to go next.
        if let Some(head) = partial.path.get(partial.path.len()-1) {
            if let Some(set) = u32network.0.get(head) {
                'outer: for &next in set.iter() {
                    if next == start_cave {
                        continue;
                    }
                    let mut next_partial = partial.clone();
                    // need to continue if more than one cave has been visited twiice
                    if next < 1000 {  // it's a small cave
                        if next_partial.visited_caves.contains(&next) {
                            if next_partial.twice {
                                continue 'outer;
                            }
                            next_partial.twice = true;
                        } else {
                            next_partial.visited_caves.insert(next);
                        }
                    }
                    next_partial.path.push(next);
                    if next == end_cave {
                        paths.push(next_partial.path.clone());
                    } else {
                        partials.push(next_partial);
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
        .iter()
        .map(|p| p.iter()
                  .map(|v| cave_bin_map.u32_to_string
                                       .get(v)
                                       .unwrap()
                                       .clone())
                  .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}


pub fn day12_1() {
    println!("Day 12: Passage Pathing, part 1");
    let lines = utils::read_file_single_result::<Line>("./input/day12.txt").expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let network = load_network(&lines);
    println!("network is: {:?}", &network);
    let all_paths = all_paths_part1(&network);
    println!("Num paths: {:?}", all_paths.len());
}


pub fn day12_2() {
    println!("Day 12: Passage Pathing, part 2");
    let lines = utils::read_file_single_result::<Line>("./input/day12.txt").expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let network = load_network(&lines);
    println!("network is: {}", &network);
    let all_paths = all_paths_part2(&network);
    println!("Num paths: {:?}", all_paths.len());
}
