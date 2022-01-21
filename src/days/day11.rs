//--- Day 11: Dumbo Octopus ---

//You enter a large cavern full of rare bioluminescent dumbo octopuses! They seem to not like the Christmas lights on your submarine, so you turn them off for now.

//There are 100 octopuses arranged neatly in a 10 by 10 grid. Each octopus slowly gains energy over time and flashes brightly for a moment when its energy is full. Although your lights are off, maybe you could navigate through the cave without disturbing the octopuses if you could predict when the flashes of light will happen.


use crate::utils;


// we need to model the octopuses.

#[derive(Clone, Debug)]
struct Octos {
    height: usize,
    width: usize,
    map: Vec<Vec<u32>>,
    flashed: Vec<Vec<bool>>,
}


fn parse_lines(lines: &[String]) -> Octos {
    let height = lines.len();
    let width = lines[0].len();
    let mut map: Vec<Vec<u32>> = Vec::with_capacity(height);
    for line in lines.iter() {
        if line.len() != width {
            panic!("Line length didn't match.");
        }
        map.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>());
    }
    Octos { height, width, map, flashed: vec![]}
}


fn inc_map(octos: &mut Octos) {
    for down in 0..octos.height {
        for across in 0..octos.width {
            octos.map[down][across] += 1;
        }
    }
    octos.flashed = vec![vec![false; octos.width]; octos.height];
}


// see if anything needs to flash, and if so then flash them.
// returns true if flashed, which means it might need to be called again.
// Also does the increments for the neighbours, and sets any that flashed
// to flashed.
fn check_for_flash(octos: &mut Octos) -> u32 {
    let mut to_flash: Vec<(usize, usize)> = Vec::new();
    let mut flashed: u32 = 0;
    // check for octopuses that will flash
    for down in 0..octos.height {
        for across in 0..octos.width {
            if !octos.flashed[down][across] && octos.map[down][across] > 9 {
                to_flash.push((down, across));
                octos.flashed[down][across] = true;
                octos.map[down][across] = 0;
                flashed += 1;
            }
        }
    }
    // now increment all the neighbours that haven't flashed
    for (down, across) in to_flash.iter() {
        for (nd, na) in neighbours(octos.height, octos.width, *down, *across).iter() {
            if !octos.flashed[*nd][*na] {
                octos.map[*nd][*na] += 1;
            }
        }
    }
    flashed
}


// return a list of neighbours as (down, across) for a map
fn neighbours(height: usize, width: usize, down: usize, across: usize) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::with_capacity(8);
    for dd in [-1, 0, 1].iter() {
        for da in [-1, 0, 1].iter() {
            let d = (down as i32) + dd;
            let a = (across as i32) + da;
            if d >= 0 && d < (height as i32) && a >= 0 && a < (width as i32) {
                neighbours.push((d as usize, a as usize));
            }
        }
    }
    neighbours
}


fn do_step(octos: &mut Octos) -> u32 {
    inc_map(octos);
    let mut total_flashes: u32 = 0;
    loop {
        let flashes = check_for_flash(octos);
        total_flashes += flashes;
        if flashes == 0 {
            break;
        }
    }
    total_flashes
}

fn print_map(octos: &Octos) {
    for down in 0..octos.height {
        println!("{}", octos.map[down].iter().map(|d| d.to_string()).collect::<String>());
    }
}

pub fn day11_1() {
    println!("Day 11: Dumbo Octopus, part 1");
    let lines = utils::read_file_single_result::<String>("./input/day11.txt").expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let mut map = parse_lines(&lines);
    print_map(&map);
    let mut all_flashes: u32 = 0;
    for _i in 0..100 {
        all_flashes += do_step(&mut map);
    }
    println!("after 100 steps:");
    print_map(&map);
    println!("Total flashes: {}", all_flashes);
}


pub fn day11_2() {
    println!("Day 11: Dumbo Octopus, part 2");
    let lines = utils::read_file_single_result::<String>("./input/day11.txt").expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let mut map = parse_lines(&lines);
    print_map(&map);
    let size = (map.height * map.width) as u32;
    let mut all_flashes: u32 = 0;
    let mut steps: u32 = 0;
    loop {
        steps += 1;
        let flashes = do_step(&mut map);
        all_flashes += flashes;
        if flashes == size {
            break;
        }
    }
    println!("after {} steps:", steps);
    print_map(&map);
    println!("Total flashes: {}", all_flashes);
}

