//--- Day 15: Chiton ---

//You've almost reached the exit of the cave, but the walls are getting closer together. Your submarine can barely still fit, though; the main problem is that the walls of the cave are covered in chitons, and it would be best not to bump any of them.


use std::fmt;
use std::collections::HashMap;
//use std::cell::RefCell;
use std::cmp::Reverse;

use priority_queue::PriorityQueue;

use crate::utils;


type Map = Vec<Vec<u32>>;


fn parse<S>(lines: &[S]) -> Result<Map, String>
    where S: AsRef<str> + fmt::Display
{
    let l = lines
         .iter()
         .map(|l| l.as_ref()
                   .chars()
                   .map(|c| c.to_digit(10).unwrap())
                   .collect::<Vec<u32>>())
         .collect::<Vec<_>>();
    Ok(l)
}


#[derive(Clone, Debug, Eq, Hash)]
struct Item {
    down: usize,
    across: usize,
    cost: usize,
}

// two items are equal if there are in the same place.
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.down == other.down && self.across == other.across
    }
}

// find the least costly path by using a prioity queue to search from 0,0 to maxd, maxa.
// The priority will be the current path cost + the manhatten distance to the end.
fn least_costly_path(map: &Map) -> usize {
    let mut pq = PriorityQueue::new();
    let mut been_there: HashMap<(usize, usize), usize> = HashMap::new();
    //pq.push(Item {down: 0, across: 0, cost: 0}, distance(&map, 0, 0));
    pq.push(Item {down: 0, across: 0, cost: 0}, Reverse(0));
    been_there.insert((0,0), 0);

    // now loop taking the least cost route and moving it forwards.
    while let Some((item, _)) = pq.pop() {
        println!("Item is {:?}, pq length is: {}", &item, pq.len());
        for &(dd, da) in [(-1,0),(1,0),(0,-1),(0,1)].iter() {
        //for &dd in ([-1,0,1] as [i32; 3]).iter() {
            let dn = dd + (item.down as i32);
            let an = da + (item.across as i32);
            if dn < 0 {
                continue;
            }
            if (dn as usize) >= map.len() {
                continue;
            }
            if an < 0 {
                continue;
            }
            if (an as usize) >= map[0].len() {
                continue;
            }
            // see if we've reached the end.
            let udn = dn as usize;
            let uan = an as usize;
            if udn == map.len() -1 && uan == map[udn].len() -1 {
                return item.cost + (map[udn][uan] as usize);
            }
            // otherwise push it as a new locations, only replacing an existing on
            // if it costs less.
            let n_item = Item { down: udn, across: uan, cost: item.cost + (map[udn][uan] as usize) };
            if let Some(prev_cost) = been_there.get(&(udn, uan)) {
                if *prev_cost < n_item.cost {
                    continue;
                }
            }
            been_there.insert((udn, uan), n_item.cost);
            if let Some((c_item, _)) = pq.get(&n_item) {
                if c_item.cost < n_item.cost {
                    continue;
                }
            }
            let cost = n_item.cost;
            //pq.push(n_item, cost + distance(&map, udn, uan));
            pq.push(n_item, Reverse(cost));
        }
    }
    panic!("No more items and not reached end!")
}

pub fn day15_1() {
    println!("Day 15: Chiton, part 1");
    let lines = utils::read_file_single_result::<String>("./input/day15.txt")
        .expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let input = parse(&lines).expect("Parsing went wrong?");
    //println!("parsed input: {:?}", input);
    println!("least costly path: {}", least_costly_path(&input));
}
