//--- Day 8: Seven Segment Search ---

//You barely reach the safety of the cave when the whale smashes into the cave mouth, collapsing it. Sensors indicate another exit to this cave at a much greater depth, so you have no choice but to press on.

use std::str::FromStr;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::cell::RefCell;

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
    #[error("Sequence not found?")]
    NotFound(String),
    #[error("Oddity happened?")]
    OddityError(String),
}


#[derive(Clone, Hash, PartialEq, Eq)]
struct Bits(u8, usize);


#[derive(Clone)]
struct Led(String, Bits);


#[derive(Debug, Clone)]
struct Line {
    codes: Vec<Led>,
    digits: Vec<Led>,
}
//
// custom debug printing for Led
impl fmt::Debug for Bits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bits(0x{}-{}-{:8})", self.to_bit_str(), self.1, self.to_str())
    }
}

// custom display printing for Led
impl fmt::Display for Bits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x0{}", self.to_bit_str())
    }
}


impl Bits {
    fn new(v: u8) -> Self {
        Self(v, utils::count_bits(v as usize, 7))
    }

    fn bits(&self) -> u8 {
        self.0
    }

    fn u8_from_char(v: char) -> Result<u8, DecodeError> {
        match v {
            'a' => Ok(0b00000001),
            'b' => Ok(0b00000010),
            'c' => Ok(0b00000100),
            'd' => Ok(0b00001000),
            'e' => Ok(0b00010000),
            'f' => Ok(0b00100000),
            'g' => Ok(0b01000000),
            _ => Err(DecodeError::InvalidError(format!("Got '{}' for an led segment?", v))),
        }
    }

    fn to_str(&self) -> String {
        let mut cs: Vec<char> = Vec::with_capacity(7);
        let mut bit: u8 = 1;
        for c in "abcdefg".chars() {
            if self.0 & bit == bit {
                cs.push(c);
            }
            bit *= 2;
        }
        cs.iter().collect()
    }

    fn to_bit_str(&self) -> String {
        let mut cs: Vec<char> = Vec::with_capacity(7);
        let mut bit: u8 = 1;
        for _ in 0..7 {
            if self.0 & bit == bit {
                cs.push('1');
            } else {
                cs.push('0');
            }
            bit *= 2;
        }
        cs.reverse();
        cs.iter().collect()
    }

    fn len(&self) -> usize {
        self.1
    }

    fn is_subset(&self, other: &Self) -> bool {
        other.0 & self.0 == self.0
    }

    // bits that are in self but not in other
    fn difference(&self, other: &Self) -> u8 {
        self.0 ^ (self.0 & other.0)
    }

    // bits that are in both
    fn intersection(&self, other: &Self) -> Bits {
        Bits::new(self.0 & other.0)
    }

    // combine bits in both
    fn union(&self, other: &Self) -> Bits {
        Bits::new(self.0 | other.0)
    }

}


impl FromStr for Bits {
    type Err = DecodeError;

    fn from_str(sequence: &str) -> Result<Self, Self::Err> {
        let mut led: u8 = 0;
        for segment in sequence.chars() {
            let bit: u8 = Self::u8_from_char(segment)?;
            led |= bit;
        }
        Ok(Self::new(led))
    }

}


impl FromStr for Led {
    type Err = DecodeError;

    fn from_str(sequence: &str) -> Result<Self, Self::Err> {
        let bits = sequence.parse::<Bits>()?;
        Ok(Self(sequence.to_string(), bits))
    }

}

// custom debug printing for Led
impl fmt::Debug for Led {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Led(0x{}-{})", self.to_bit_str(), self.to_str())
    }
}

// custom display printing for Led
impl fmt::Display for Led {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Led:0x0{}", self.to_bit_str())
    }
}

impl Led {

    fn to_str(&self) -> String {
        format!("{:7}-({:7})", self.0, self.1.to_str())
    }

    fn str_part(&self) -> String {
        self.0.clone()
    }

    fn to_bit_str(&self) -> String {
        self.1.to_bit_str()
    }

    fn len(&self) -> usize {
        self.1.len()
    }

    fn bits(&self) -> Bits {
        Bits::new(self.1.bits())
    }
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
        let codes = parts[0]
            .split_whitespace()
            .map(|s| s.parse::<Led>())
            .collect::<Result<Vec<_>, _>>()?;
        let digits = parts[1]
            .split_whitespace()
            .map(|s| s.parse::<Led>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self{codes, digits})
    }
}


impl Line {

    /// find a code by a substring and size.  First it filters by size, and then by substring
    /// Note that it only returns a single item, or None.
    fn find_one_code_by_bits_and_len(&self, bits: &Bits, size: usize) -> Result<String, DecodeError>
    {
        let cs = self.codes.iter()
            .filter(|led| led.len() == size && bits.is_subset(&led.bits()))
            .collect::<Vec<_>>();

        if cs.len() != 1 {
            return Err(DecodeError::OddityError(format!("No codes with substr {} and size {}?", &bits, &size)));
        }
        Ok(cs.get(0).unwrap().str_part())
    }

    // find the first sequence of length passed;
    // TODO: need to switch it to return the Led, and not the string of the Led
    fn find_first_code_len(&self, length: usize) -> Result<String, DecodeError> {
        for code in self.codes.iter() {
            if code.len() == length {
                return Ok(code.0.clone());
            }
        }
        Err(DecodeError::NotFound(format!("Couldn't find sequence of length {}", length)))
    }
}

// -------------------------------------------------------------------------------------------------

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


// -------------------------------------------------------------------------------------------------

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


// Interior mutability as it's actually easier to just stop fighting
// the borrow checking when using iterators and lifetimes of the things.
// I do have to clone, though.  Probably could've got away without the
// interior mutability.
#[derive(Debug, Clone)]
struct Mapping {
    map: RefCell<HashMap<u8, Bits>>,      // may the bits of a u8 to a Bits (u8 + count of bits)
}


impl fmt::Display for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Map:\n")?;
        for (k, v) in self.map.borrow().iter() {
            write!(f, "  {:?} -> {:?}\n", Bits::new(*k), v)?;
        }
        Ok(())
    }
}


impl Mapping {
    fn new() -> Self {
        Self { map: RefCell::new(HashMap::new()) }
    }

    fn insert<S>(&self, from: S, to: S) -> Result<(), DecodeError>
        where S: AsRef<str>
    {
        let from = from.as_ref().parse::<Bits>()?;
        let to = to.as_ref().parse::<Bits>()?;
        if from.len() != to.len() {
            return Err(DecodeError::OddityError(
                format!("Mapping.insert - lengths to match: {:?} -> {:?}", from, to)));
        }
        self.map.borrow_mut().insert(from.bits(), to);
        Ok(())
    }

    fn insert_u8(&self, from: &u8, to: &Bits) -> Result<(), DecodeError> {
        if utils::count_bits(*from as usize, 7) != to.len() {
            return Err(DecodeError::OddityError(
                format!("Mapping.insert - lengths don't match: {:?} -> {:?}", Bits::new(*from), to)));
        }
        self.map.borrow_mut().insert(*from, to.clone());
        Ok(())
    }

    fn add_digit<S>(&self, digit: usize, seq: S) -> Result<(), DecodeError>
        where S: AsRef<str>
    {
        if digit > 9 {
            panic!("Can't handle a digit larger than 9");
        }
        let to = seq.as_ref().parse::<Led>()?;
        //self.digits.borrow_mut().insert(digit, to.clone());                // map the digit to the scrambled sequence
        let from = LEDS[digit].parse::<Bits>()?;
        self.map.borrow_mut().insert(from.bits(), to.bits());  // map the real to the scrambled sequence
        Ok(())
    }

    fn map_get<S: AsRef<str>>(&self, seq: S) -> Result<Bits, DecodeError> {
        let s = seq.as_ref().parse::<Bits>()?;
        self.map.borrow().get(&s.bits())
            .ok_or(DecodeError::NotFound(format!("Sequence '{}' not found?", &s)))
            .map(|v| v.clone())
    }

    fn map_get_u8(&self, bits: &u8) -> Result<Bits, DecodeError> {
        self.map.borrow().get(&bits)
            .ok_or(DecodeError::NotFound(format!("Sequence '{}' not found?", &bits)))
            .map(|v| v.clone())
    }

    fn derive_from<S>(&self, c: char, s1: S, s2: S) -> Result<(), DecodeError>
        where S: AsRef<str> + fmt::Display,
    {
        if s1.as_ref().len() != s1.as_ref().len() {
            return Err(DecodeError::OddityError(
                format!("derive_from: strings not same length: '{}' and '{}'", &s1, &s2)));
        }
        let bits1 = self.map_get(s1.as_ref())?;
        let bits2 = self.map_get(s2.as_ref())?;
        let ds_bits = bits1.intersection(&bits2);
        if ds_bits.len() != 1 {
            return Err(DecodeError::OddityError(format!("Expected only a single digit, got {:?}", &ds_bits)));
        }
        let c_bits = Bits::from_str(&c.to_string())?;
        self.insert_u8(&c_bits.bits(), &ds_bits)?;
        Ok(())
    }

    /// derive the sequence s using the other sequences that are availably in the mapping.
    /// Essentially, find all the codes that contain 's', sorted by size, and attempt to remove
    /// already identified mappings, until 's' is known.
    fn derive<S>(&self, s: S) -> Result<(), DecodeError>
        where S: AsRef<str> + fmt::Display,
    {
        // get all the mappings that contain 's', if we find s, then just return
        let bits = s.as_ref().parse::<Bits>()?;
        //println!("Trying to derive: {:?}", &bits);
        let mut keys = self.map.borrow().keys()
            .map(|ks| Bits::new(*ks))         // turn a u8 back into a Bits with a length
            .filter(|ks| bits.is_subset(ks))
            .collect::<Vec<_>>();
        keys.sort_by(|a,b| a.len().cmp(&b.len()));
        //println!("Found {:?}", &keys);
        let diffs: Vec<_> = keys.iter().map(|k| self.map_get_u8(&k.difference(&bits))).collect();
        for (key, diff_res) in keys.iter().zip(diffs.iter()) {
            match diff_res {
                Ok(dv) => {
                    // we matched the remaining part of the key, so we can resolve the difference.
                    let to = self.map_get_u8(&key.bits())
                                 .map(|v| v.difference(&dv))?;
                    //println!("matched key {:?} remainders= {:?} -> {:?}.", &key, &bits, &Bits::new(to));
                    self.insert_u8(&bits.bits(), &Bits::new(to))?;
                    return Ok(());
                },
                Err(_) => {},
            }
        }
        Err(DecodeError::OddityError(format!("Couldn't derive {}?", &s)))
    }

    fn combine<S>(&self, s1: S, s2: S) -> Result<(), DecodeError>
        where S: AsRef<str> + fmt::Display,
    {
        let t: String = vec![s1.as_ref(), s2.as_ref()].concat();
        let t_bits = Bits::from_str(&t)?;
        // early return if it already exists
        if let Ok(_) = self.map_get_u8(&t_bits.bits()) {
            return Ok(());
        }
        let bits1 = self.map_get(s1.as_ref())?;
        let bits2 = self.map_get(s2.as_ref())?;
        let u_bits = bits1.union(&bits2);
        self.insert_u8(&t_bits.bits(), &u_bits)?;
        Ok(())
    }

    /// Resolve a segment set from the available mappings of single characters
    fn resolve<S>(&self, r: S) -> Result<(), DecodeError>
        where S: AsRef<str> + fmt::Display,
    {
        let mut s = String::with_capacity(7);
        let mut r_iter = r.as_ref().chars();
        match r_iter.next() {
            Some(c) => s.push(c),
            None => return Err(DecodeError::OddityError("No string passed to resolve?".to_string())),
        };
        while let Some(c) = r_iter.next() {
            self.combine(c.to_string(), s.clone())?;
            s.push(c);
        }
        Ok(())
    }

}

// Decode a line (such as the one below) into their correspending mappings of 7-segment display,
// and decode the digits after the '|' into a a int in the form of '8153'
// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
fn decode_line(line: &Line) -> Result<u32, DecodeError>  {
    let mapping: Mapping = Mapping::new();
    for (digit, size) in [(1, 2), (4, 4), (7, 3), (8, 7)] {
        let seq = line.find_first_code_len(size)?;
        mapping.add_digit(digit, seq)?;
    }
    // derive 'a' from digits '7' (acf) and '1' (cf)
    //mapping.derive_from_digits(1, 7)?;
    //println!("1 - Mapping now is: {}", &mapping);
    mapping.derive("a")?;
    // derive 'bd' from digit '4' -> bcdf, cf(bd) -> be(??), thus: bd->cg
    //mapping.derive_from_digits(1, 4)?;
    mapping.derive("bd")?;
    // '3' -> acdfg => acf(dg) -> bde(??) [bde(cf), so dg -> cf]
    //  as bd->cg and dg->cf, then d->c AND b->g AND g->f **
    let acf = mapping.map_get("acf")?;
    let acdfg = line.find_one_code_by_bits_and_len(&acf, 5)?;
    // add mapping for acdfg -> ????? above, and then derive 'dg'
    mapping.insert("acdfg", &acdfg)?;
    // derive dg by acfdg - acf
    //mapping.derive_with("acdfg", "acf")?;
    mapping.derive("dg")?;
    // now derive 'd' from 'dg' and 'bd'
    mapping.derive_from('d', "dg", "bd")?;
    //mapping.derive("d")?;  // Not sure if this is possible?
    mapping.derive("b")?;
    mapping.derive("g")?;

//  mapping so far (abdg -> dgcf)
// '5' -> abdfg => abdg(f) -> dgcf(e) so, f->e **
    mapping.combine("a", "dg")?;
    mapping.combine("adg", "b")?;
    let abdg = mapping.map_get("abdg")?;
    let abdfg = line.find_one_code_by_bits_and_len(&abdg, 5)?;
    mapping.insert("abdfg", &abdfg)?;
    mapping.derive("f")?;
//  mapping so far (abdfg -> dgcef)
// '2' -> acdeg => adg(ce) -> dcf(ab), so ce -> ab
//  and cf -> be (from '7'), hence c->b (and f->e also fom '5')
//
// '0' -> abcefg => abc(e)fg -> dgb(?)ef  e -> a **
// '6' -> abdefg => abd(e)fg -> dgc(?)ef, so e->a
// '9' -> abcdfg => abdcfg -> dgcbef
    mapping.derive("c")?;
    mapping.derive("a")?;
    mapping.combine("acdfg", "b")?;
    mapping.derive("e")?;
//
    // Now derive the remaining digits.
    let mut digit_map: HashMap<Bits, usize> = HashMap::with_capacity(10);
    for (i, led) in LEDS.iter().enumerate() {
        let l = Led::from_str(led)?;
        let m = match mapping.map_get_u8(&l.bits().bits()) {
            Ok(v) => v,
            Err(_) => {
                //println!("Resolve {:?}", l);
                mapping.resolve(l.str_part())?;
                //println!("Resolved?");
                mapping.map_get_u8(&l.bits().bits()).unwrap()
            },
        };
        //println!("{} - {:?} - {:?}", i, l, m);
        digit_map.insert(m, i);
    }

    // finially, let's work out what the digits are:
    let digits = line.digits.iter()
        .map(|d| Bits::from_str(&d.str_part())
            .and_then(|bits| digit_map.get(&bits)
                                      .ok_or(DecodeError::OddityError(format!("Couldn't find {:?}", &bits)))))
        .collect::<Result<Vec<_>, _>>()?;
    //println!("Line {:?}", &line);
    //println!("Digits {:?}", &digits);
    let mut sum: usize = 0;
    let mut d: usize = 1;
    for c in digits.iter().rev() {
        sum += *c * d;
        d *= 10;
    }
    println!("Sum is {}", sum);
    Ok(sum as u32)
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
    let lines = utils::read_file_single_result::<Line>("./input/day08.txt").expect("Couldn't read file");
    println!("Input: {:?}", lines);
    let mut v: u32 = 0;
    for line in lines {
        let d = decode_line(&line).expect("Damn!");
        v += d;
    }
    println!("Total is {}", v);
}
