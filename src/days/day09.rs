// --- Day 9: Smoke Basin ---

//These caves seem to be lava tubes. Parts are even still volcanically active; small hydrothermal vents release smoke into the caves that slowly settles like rain.

//If you can model how the smoke flows through the caves, you might be able to avoid it and be that much safer. The submarine generates a heightmap of the floor of the nearby caves for you (your puzzle input).

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
