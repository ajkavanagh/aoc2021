// --- Day 9: Smoke Basin ---

//These caves seem to be lava tubes. Parts are even still volcanically active; small hydrothermal vents release smoke into the caves that slowly settles like rain.

//If you can model how the smoke flows through the caves, you might be able to avoid it and be that much safer. The submarine generates a heightmap of the floor of the nearby caves for you (your puzzle input).

use std::collections::HashSet;

use crate::utils;

#[derive(Debug, Clone)]
struct Map(Vec<Vec<u32>>);


fn parse_lines(lines: &[String]) -> Map {
    let r = lines.iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Map(r)
}


fn neighbours(map: &Map, down: usize, across: usize) -> [u32; 4] {
    let above = if down == 0 {
        10
    } else {
        map.0[down-1][across]
    };
    let below = if down >= map.0.len() -1 {
        10
    } else {
        map.0[down+1][across]
    };
    let left = if across == 0 {
        10
    } else {
        map.0[down][across-1]
    };
    let right = if across >= map.0[0].len() -1 {
        10
    } else {
        map.0[down][across+1]
    };
    [above, right, left, below]  // NEWS in the map
}


/// part 1 - just find the low points.
fn find_low_points(map: &Map) -> Vec<(usize, usize, u32)> {
    let mut points: Vec<(usize, usize, u32)> = vec![];
    for down in 0..map.0.len() {
        for across in 0..map.0[0].len() {
            let here = map.0[down][across];
            let ns = neighbours(&map, down, across);
            if ns.iter().all(|h| *h > here) {
                points.push((down, across, here));
            }
        }
    }
    println!("Found {} places", &points.len());
    points
}

// ----

// Part 2 -- find the basin sizes.  e.g. from a low point expand outwards from that point upwards
// until the 9 is hit. e.g. don't count the 9 in the size.
//
// Strategy: stating at the low point; explore the height of the neighbours.  If higher add to a
// queue to search next.  Each time around, drop one off the queue and search for the next one.
// Use find_low_points to find the lowest points, and then use neighbours to check the
// surroundings.

type Point = (usize, usize, u32);  // down, across, height

/// find all the basins, sum them up and return the value.
fn find_basins(map: &Map) -> u32 {
    let points = find_low_points(&map);
    let mut sizes: Vec<u32> = Vec::new();
    for point in points.iter() {
        let size = find_basin_size(&map, &point);
        println!("Size of {:?} is {}", &point, &size);
        sizes.push(size);
    }
    sizes.sort_by(|a,b| b.cmp(a));
    sizes.iter().take(3).product()
}


// find the basin size
fn find_basin_size(map: &Map, point: &Point) -> u32 {
    println!("--");
    println!(" Find basin starting at: {:?}", &point);
    let mut queue: Vec<Point> = Vec::new();
    let mut count: u32 = 0;
    let mut visited: HashSet<Point> = HashSet::new();
    queue.push(*point);
    while let Some(at) = queue.pop() {
        if !visited.contains(&at) {
            count += 1;
            println!("Explore at: {:?}, count is: {}", &at, &count);
            explore_at(&map, &mut queue, &mut visited, &at);
        }
    }
    count
}


// explore at a point, adding any new points to the queue
fn explore_at(map: &Map, queue: &mut Vec<Point>, visited: &mut HashSet<Point>, at: &Point) {
    visited.insert(*at);
    let (down, across, height) = *at;
    let neighbour_heights = neighbours(&map, down, across); // [above, right, left, below]  // NEWS in the map
    for (i, (dd, da)) in [(-1,0), (0,1), (0,-1), (1,0)].iter().enumerate() {
        if neighbour_heights[i] < 9 && neighbour_heights[i] > height {
            let new_down = ((down as i32) + dd) as usize;
            let new_across = ((across as i32) + da) as usize;
            println!("  new at: {}, {}, height: {}", &new_down, &new_across, &neighbour_heights[i]);
            queue.push((new_down, new_across, neighbour_heights[i]));
        }
    }
}

pub fn day9_1() {
    println!("Day 9: Smoke Basin, part 1");
    let lines = utils::read_file_single_result::<String>("./input/day09.txt").expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let map = parse_lines(&lines);
    println!("Map: {:?}", &map);
    let points = find_low_points(&map);
    let t: u32 = points.iter().map(|&(_,_,h)| h+1).sum();
    println!("Result is: {}", t);
}


pub fn day9_2() {
    println!("Day 9: Smoke Basin, part 2");
    let lines = utils::read_file_single_result::<String>("./input/day09.txt").expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let map = parse_lines(&lines);
    println!("Map: {:?}", &map);
    let sum = find_basins(&map);
    println!("Result is: {}", sum);
}
