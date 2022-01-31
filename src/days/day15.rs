//--- Day 15: Chiton ---

//You've almost reached the exit of the cave, but the walls are getting closer together. Your submarine can barely still fit, though; the main problem is that the walls of the cave are covered in chitons, and it would be best not to bump any of them.


use std::fmt;
use std::collections::HashMap;
//use std::cell::RefCell;
use std::cmp::Reverse;

use priority_queue::PriorityQueue;

use crate::utils;


trait MapAt {
    // Get the item at (down, across).  If out of bounds, returns None.
    fn at(&self, down: usize, across: usize) -> Option<u32>;

    // Get the size of the Map in (down, across).  Note if size is 10, valid incices are 0 to 9
    // inclusive.
    fn bounds(&self) -> (usize, usize);
}


// part 1 solution - real map
#[derive(Clone, Debug)]
struct Map(Vec<Vec<u32>>);

impl Map {

    fn parse<S>(lines: &[S]) -> Result<Self, String>
        where S: AsRef<str> + fmt::Display
    {
        let l = lines
             .iter()
             .map(|l| l.as_ref()
                       .chars()
                       .map(|c| c.to_digit(10).unwrap())
                       .collect::<Vec<u32>>())
             .collect::<Vec<_>>();
        Ok(Self(l))
    }
}


impl MapAt for Map {

    fn at(&self, down: usize, across: usize) -> Option<u32> {
        self.0.get(down).and_then(|v| v.get(across).cloned())
    }

    fn bounds(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }
}

// --- part 2 - a virtual map 5 times the size of the real one.
#[derive(Clone, Debug)]
struct VirtualMap {
    map: Map,
    md: usize,             // the multiplier for the downward direction
    ma: usize,             // the multiplier for the across direction
}


impl VirtualMap {

    fn parse<S>(lines: &[S], md: usize, ma: usize) -> Result<Self, String>
        where S: AsRef<str> + fmt::Display
    {
        let map = Map::parse(lines)?;
        Ok(Self {map, md, ma})
    }
}

impl MapAt for VirtualMap {

    fn at(&self, down: usize, across: usize) -> Option<u32> {
        let (vd, va) = self.map.bounds();
        let rd = down % vd;
        let ra = across % va;
        self.map.at(rd, ra).and_then(|v|
            {
                let md = (down / vd) as u32;
                let ma = (across / va) as u32;
                Some((((v - 1) + md + ma) % 9) + 1)
            })
    }

    fn bounds(&self) -> (usize, usize) {
        let (vd, va) = self.map.bounds();
        (vd * self.md, va * self.ma)
    }
}

// ---

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
fn least_costly_path(map: &dyn MapAt) -> usize {
    let (down_max, across_max) = map.bounds();
    let mut pq = PriorityQueue::new();
    let mut been_there: HashMap<(usize, usize), usize> = HashMap::new();
    //pq.push(Item {down: 0, across: 0, cost: 0}, distance(&map, 0, 0));
    pq.push(Item {down: 0, across: 0, cost: 0}, Reverse(0));
    been_there.insert((0,0), 0);

    // now loop taking the least cost route and moving it forwards.
    while let Some((item, _)) = pq.pop() {
        let cost = map.at(item.down, item.across).unwrap();
        println!("Item is {:?}, cost there: {}, pq length is: {}", &item, &cost, pq.len());
        for &(dd, da) in [(-1,0),(1,0),(0,-1),(0,1)].iter() {
            let dn = dd + (item.down as i32);
            let an = da + (item.across as i32);
            if dn < 0 {
                continue;
            }
            if (dn as usize) >= down_max {
                continue;
            }
            if an < 0 {
                continue;
            }
            if (an as usize) >= across_max {
                continue;
            }
            // see if we've reached the end.
            let udn = dn as usize;
            let uan = an as usize;
            if udn == down_max -1 && uan == across_max -1 {
                return item.cost + (map.at(udn, uan).unwrap() as usize);
            }
            // otherwise push it as a new locations, only replacing an existing on
            // if it costs less.
            let n_item = Item { down: udn, across: uan, cost: item.cost + (map.at(udn, uan).unwrap() as usize) };
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
    let input = Map::parse(&lines).expect("Parsing went wrong?");
    println!("least costly path: {}", least_costly_path(&input));
}


pub fn day15_2() {
    println!("Day 15: Chiton, part 2");
    let lines = utils::read_file_single_result::<String>("./input/day15.txt")
        .expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let input = VirtualMap::parse(&lines, 5, 5).expect("Parsing went wrong?");
    println!("least costly path: {}", least_costly_path(&input));
}
