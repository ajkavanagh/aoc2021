//--- Day 8: Seven Segment Search ---

//You barely reach the safety of the cave when the whale smashes into the cave mouth, collapsing it. Sensors indicate another exit to this cave at a much greater depth, so you have no choice but to press on.

use std::str::FromStr;
use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt;

use thiserror::Error;

use crate::utils;

// might not need this
const LEDS: &'static [&str] = &[
    "abcefg",  // 0
    "cf",      // 1
    "acdeg",   // 2
    "acdfg",   // 3
    "bcdf",    // 4
    "abdfg",   // 5
    "abdefg",  // 6
    "acf",     // 7
    "abcdefg", // 8
    "abcdfg",  // 9
];

#[derive(Error, Debug, Clone)]
pub enum DecodeError {
    #[error("Invalid command")]
    InvalidError(String),
    #[error("Too many chars left")]
    TooManyChars(String),
    #[error("Sequence not found?")]
    NotFound(String),
    #[error("Oddity happened?")]
    OddityError(String),
}


#[derive(Debug, Clone)]
struct Line {
    codes: Vec<String>,
    digits: Vec<String>,
}


impl FromStr for Line {
    type Err = DecodeError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // split at '|' first.
        let parts = line.split("|").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(DecodeError::InvalidError(format!("Line doesn't contain a '|': {}", line)));
        }
        // split the first line into strings, and strip them.
        let codes = parts[0].split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();
        let digits = parts[1].split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();
        Ok(Self{codes, digits})
    }
}


impl Line {
    fn new() -> Self {
        Line {codes: vec![], digits: vec![]}
    }

    /// find a code by a substring and size.  First it filters by size, and then by substring
    /// Note that it only returns a single item, or None.
    fn find_one_code_by_substr_and_len<S: AsRef<str>>(&self, sub: S, size: usize) -> Option<String> {
        let hs: HashSet<char> = sub.as_ref().chars().collect();
        if hs.len() > size {
            return None;
        }
        let rs: Vec<String> = self.codes.iter()
            .filter(|v| v.len() == size)
            .filter(|v| {
                let vs: HashSet<char> = v.chars().collect();
                vs.is_superset(&hs)
            })
            .cloned()
            .collect();
        if rs.len() == 1 {
            return rs.get(0).cloned();
        }
        None
    }
}

// count the 1,4, 7 and 8 digits.
// part 1 - function
fn count_unique_digits(lines: &[Line]) -> u32 {
    lines.iter()
        .map(|l| l.digits.iter().filter(|&d| {
                let ll = d.len();
                //    1          4          7          8
                ll == 2 || ll == 4 || ll == 3 || ll == 7
            }).count() as u32)
    .sum()
}


// part 2
// decode the single wires and digits.
// take the first line of the test data:
//
// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
//
// Digit 1: 'be' drives 'cf' as either 'b->c|f', and 'e->c|f'
// And 'cgeb' drives 'bcdf' as 'c->b|c|d|f', 'g->'b|c|d|f|, 'e->b|c|d|f'
// But 'e' also drives 'e->f|c' (from be).
// So we now 'know', 'e->f|c'
// Also fom digit 1, 'b->c|f', so that knocks out 'cf' from 'cgeb->bcdf'.
// This leaves 'gb->bd' (or 'g->bd' and 'b->bd')
//
// Use a HashMap<char, HashSet<char>> to capture 'scrambled-letter' -> 'set of letters'
// Then try to reduce down the sets of letters such that each letter maps to just one letter.
// (not sure if this is possible).

//        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
// sorted be abcdefg bcdefg acdefg bceg cdefg abdefg bcdef abcdf bde
// length 2  7       6      6      4    5     6      5     5     3
// digit  1  8       9      6      4    5     0      3     2     7
//
// for above line (real -> scrambled):
// '1' -> cf -> be
// '7' -> a(cf) -> d(be), leaving a->d **
// '4' -> bcdf -> bceg, cf(bd) -> be(cg), thus: bd->cg
// '3' -> acdfg => acf(dg) -> bde(??) [bde(cf), so dg -> cf]
//  as bd->cg and dg->cf, then d->c AND b->g AND g->f **
//  mapping so far (abdg -> dgcf)
// '5' -> abdfg => abdg(f) -> dgcf(e) so, f->e **
//  mapping so far (abdfg -> dgcef)
// '2' -> acdeg => adg(ce) -> dcf(ab), so ce -> ab
//  and cf -> be (from '7'), hence c->b (and f->e also fom '5')
//
// '0' -> abcefg => abc(e)fg -> dgb(?)ef  e -> a **
// '6' -> abdefg => abd(e)fg -> dgc(?)ef, so e->a
// '9' -> abcdfg => abdcfg -> dgcbef
//
// final mapping (abcdefg -> dgbcaef)
//
// Order of digits 1, 7, 4, 8 (based on lengths).
// Then 3,5,2,9, (and thus 0 and 6)


#[derive(Debug, Clone)]
struct Sequence {
    from: String,
    to: String,
}


impl Sequence {
    fn new<S: ToString>(from: S, to: S) -> Self {
        let fs = from.to_string();
        let ts = to.to_string();
        if fs.len() != ts.len() {
            panic!("Programming error; passed non equal length strings to add: {}, {}", fs, ts);
        }
        Self {from: sort_string(fs), to: sort_string(ts) }
    }
}


fn sort_string<S: ToString>(s: S) -> String {
    let mut cs = s.to_string().chars().collect::<Vec<char>>();
    cs.sort_by(|a,b| a.cmp(b));
    cs.iter().collect()
}



#[derive(Debug, Clone)]
struct Mapping {
    map: HashMap<String, HashSet<char>>,
    digits: HashMap<usize, HashSet<char>>,
}

impl fmt::Display for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Map:\n")?;
        for (k, v) in self.map.iter() {
            write!(f, "  {} -> {}\n", k, sort_string(v.iter().collect::<String>()))?;
        }
        write!(f, "Digits:\n")?;
        for (d, v) in self.digits.iter() {
            write!(f, "  ({}) -> {}\n", d, sort_string(v.iter().collect::<String>()))?;
        }
        Ok(())
    }
}


impl Mapping {
    fn new() -> Self {
        Self {map: HashMap::new(), digits: HashMap::new()}
    }

    fn insert(&mut self, sequence: &Sequence) {
        self.map.insert(sequence.from.clone(), sequence.to.chars().collect());
    }

    fn add_digit<S: AsRef<str>>(&mut self, digit: usize, seq: S) {
        if digit > 9 {
            panic!("Can't handle a digit larger than 9");
        }
        let to: HashSet<char> = sort_string(seq.as_ref()).chars().collect();
        self.digits.insert(digit, to.clone());                // map the digit to the scrambled sequence
        self.map.insert(LEDS[digit].to_owned(), to);  // map the real to the scrambled sequence
    }

    fn map_has<S: AsRef<str>>(&self, seq: S) -> bool {
        let s = sort_string(seq.as_ref());
        self.map.contains_key(&s)
    }

    fn map_get<S: AsRef<str>>(&self, seq: S) -> Option<String> {
        let s = sort_string(seq.as_ref());
        self.map.get(&s).map(|hs| hs.iter().collect::<String>())
    }


    fn get_digit(&self, digit: usize) -> Result<HashSet<char>, DecodeError> {
        self.digits.get(&digit).map(|v| v.clone())
            .ok_or(DecodeError::NotFound(format!("Digit {} not found", digit)))
    }


    // derive the difference between two numbers and record their Sequence to the map.
    fn derive_from_digits(&mut self, d1: usize, d2: usize) -> Result<(), DecodeError> {
        let hs1 = self.get_digit(d1)?;
        let hs2 = self.get_digit(d2)?;
        let diff: String = if hs1.len() > hs2.len() {
            hs1.difference(&hs2).collect()
        } else {
            hs2.difference(&hs1).collect()
        };
        // now diff the real (unscrambled sequences).
        let s1: HashSet<char> = LEDS[d1].chars().collect();
        let s2: HashSet<char> = LEDS[d2].chars().collect();
        let ds: String = if s1.len() > s2.len() {
            s1.difference(&s2).collect()
        } else {
            s2.difference(&s1).collect()
        };
        if ds.len() == 0 {
            return Err(DecodeError::OddityError(format!("No difference between {} and {}?", d1, d2)));
        }
        self.insert(&Sequence::new(ds, diff));
        Ok(())
    }

    fn as_hashset<S: AsRef<str>>(s: S) -> HashSet<char> {
        s.as_ref().chars().collect()
    }

    fn diff_strings<S: AsRef<str>>(s1: S, s2: S) -> HashSet<char> {
        let hs1: HashSet<_> = s1.as_ref().chars().collect();
        let hs2: HashSet<_> = s2.as_ref().chars().collect();
        if hs1.len() > hs2.len() {
            hs1.difference(&hs2).cloned().collect()
        } else {
            hs2.difference(&hs1).cloned().collect()
        }
    }

    fn derive_with<S: AsRef<str> + fmt::Display>(&mut self, s1: S, s2: S) -> Result<(), DecodeError> {
        let hs1 = Self::as_hashset(&s1);
        let hs2 = Self::as_hashset(&s2);
        let diff: String = if hs1.len() > hs2.len() {
            hs1.difference(&hs2).collect()
        } else {
            hs2.difference(&hs1).collect()
        };
        // now diff the real (unscrambled sequences).
        let ts1 = self.map.get(&sort_string(s1.as_ref().clone()))
            .ok_or(DecodeError::OddityError(format!("Couldn't find {}", s1)))?;
        let ts2 = self.map.get(&sort_string(s2.as_ref().clone()))
            .ok_or(DecodeError::OddityError(format!("Couldn't find {}", s1)))?;
        let ds: String = if ts1.len() > ts2.len() {
            ts1.difference(&ts2).collect()
        } else {
            ts2.difference(&ts1).collect()
        };
        if ds.len() == 0 {
            return Err(DecodeError::OddityError(format!("No difference between {} and {}?", s1, s2)));
        }
        self.insert(&Sequence::new(diff, ds));
        Ok(())
    }

    fn derive_from<S>(&mut self, c: char, s1: S, s2: S) -> Result<(), DecodeError>
        where S: AsRef<str> + fmt::Display,
    {
        let ts1 = self.map.get(&sort_string(s1.as_ref().clone()))
            .ok_or(DecodeError::OddityError(format!("Couldn't find {}", s1)))?;
        let ts2 = self.map.get(&sort_string(s2.as_ref().clone()))
            .ok_or(DecodeError::OddityError(format!("Couldn't find {}", s1)))?;
        if ts1.len() != ts2.len() {
            return Err(DecodeError::OddityError(format!("Didn't pass same length strings: {} - {}", &s1, &s2)));
        }
        let ds: String = ts1.intersection(&ts2).collect();
        if ds.len() != 1 {
            return Err(DecodeError::OddityError(format!("Expected only a single digit, got {}", &ds)));
        }
        self.insert(&Sequence::new(c.to_string(), ds));
        Ok(())
    }

    /// derive the sequence s using the other sequences that are availably in the mapping.
    /// Essentially, find all the codes that contain 's', sorted by size, and attempt to remove
    /// already identified mappings, until 's' is known.
    fn derive<S>(&mut self, s: S) -> Result<(), DecodeError>
        where S: AsRef<str> + fmt::Display,
    {
        // get all the mappings that contain 's', if we find s, then just return
        let hs = Self::as_hashset(&s);
        let mut keys = self.map.keys()
            .map(|k| Self::as_hashset(k))
            .filter(|ks| hs.is_subset(ks))
            .collect::<Vec<_>>();
        keys.sort_by(|a,b| a.len().cmp(&b.len()));
        println!("Found {:?}", &keys);
        for k in keys {
            let ds: String = k.difference(&hs).collect::<String>();
            if let Some(dv) = self.map_get(&ds) {
                // we matched the remaining part of the key, so we can resolve the difference.
                let sv = self.map_get(&s)
                    .ok_or(DecodeError::OddityError(format!("Couldn't fetch {}, but it should exist?", s)))?;
                // difference between the the two hashes
                let diff = Self::diff_strings(&dv, &sv).iter().collect::<String>();
                self.insert(&Sequence::new(&ds, &diff));
                return Ok(());
            }
        }
        Err(DecodeError::OddityError(format!("Couldn't derive {}?", &s)))
    }

}

// not sure what the return type should be yet?
// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
fn decode_line(line: &Line) -> Result<String, DecodeError>  {
    let mut mapping: Mapping = Mapping::new();
    for (digit, size) in [(1, 2), (4, 4), (7, 3), (8, 7)] {
        let seq = find_sequence_len(&line, size)?;
        mapping.add_digit(digit, seq);
    }
    // derive 'a' from digits '7' (acf) and '1' (cf)
    mapping.derive_from_digits(1, 7)?;
    // derive 'bd' from digit '4' -> bcdf, cf(bd) -> be(??), thus: bd->cg
    mapping.derive_from_digits(1, 4)?;
    // '3' -> acdfg => acf(dg) -> bde(??) [bde(cf), so dg -> cf]
    //  as bd->cg and dg->cf, then d->c AND b->g AND g->f **
    let acf_mapping = mapping.map_get("acf")
        .ok_or(DecodeError::OddityError("Couldn't find acf??".to_owned()))?;
    let acdfg = line.find_one_code_by_substr_and_len(&acf_mapping, 5)
        .ok_or(DecodeError::OddityError("Couldn't extract acdfg code??".to_owned()))?;
    println!("acdfg is {}", sort_string(&acdfg));
    // add mapping for acdfg -> ????? above, and then derive 'dg'
    mapping.insert(&Sequence::new("acdfg", &acdfg));
    // derive dg by acfdg - acf
    mapping.derive_with("acdfg", "acf")?;
    // now derive 'd' from 'dg' and 'bd'
    mapping.derive_from('d', "dg", "bd")?;
    println!("Mapping now is: {}", &mapping);
    mapping.derive("b")?;
    mapping.derive("g")?;

//  mapping so far (abdg -> dgcf)
// '5' -> abdfg => abdg(f) -> dgcf(e) so, f->e **
//  mapping so far (abdfg -> dgcef)
// '2' -> acdeg => adg(ce) -> dcf(ab), so ce -> ab
//  and cf -> be (from '7'), hence c->b (and f->e also fom '5')
//
// '0' -> abcefg => abc(e)fg -> dgb(?)ef  e -> a **
// '6' -> abdefg => abd(e)fg -> dgc(?)ef, so e->a
// '9' -> abcdfg => abdcfg -> dgcbef
    unimplemented!()
}


fn find_sequence_len(line: &Line, length: usize) -> Result<String, DecodeError> {
    for code in line.codes.iter() {
        if code.len() == length {
            return Ok(code.clone());
        }
    }
    Err(DecodeError::NotFound(format!("Couldn't find sequence of length {}", length)))
}


pub fn day8_1() {
    println!("Day 8: Seven Segment Search");
    let lines = utils::read_file_single_result::<Line>("./input/day08.txt").expect("Couldn't read file");
    println!("Input: {:?}", lines);
    let s = count_unique_digits(&lines[..]);
    println!("Sum of unique segment lengths 1,4,7,8: {:?}", s);
}

pub fn day8_2() {
    println!("Day 8: Seven Segment Search - part 2: decoding");
    let lines = utils::read_file_single_result::<Line>("./input/day08-test.txt").expect("Couldn't read file");
    println!("Input: {:?}", lines);
    decode_line(&lines[0]).expect("Damn!");
}
