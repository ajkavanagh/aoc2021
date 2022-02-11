//--- Day 18: Snailfish ---

//You descend into the ocean trench and encounter some snailfish. They say they saw the sleigh keys! They'll even tell you which direction the keys went if you help one of the smaller snailfish with his math homework.
//
use std::fmt;
use std::str::FromStr;

use thiserror::Error;

use crate::utils;


#[derive(Error, Debug, Clone)]
pub enum DecodeError {
    #[error("Invalid command")]
    InvalidError(String),
}


#[derive(Clone, Debug)]
enum PairItemEnum {
    IsNatural(u32),
    IsPair(Box<Pair>),
}

#[derive(Clone, Debug)]
struct PairItem(PairItemEnum);


impl fmt::Display for PairItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            PairItemEnum::IsNatural(n) => write!(f, "{}", n),
            PairItemEnum::IsPair(ref bp) => write!(f, "{}", &*bp),
        }
    }
}



impl PairItem {
    fn is_natural(&self) -> bool {
        match self.0 {
            PairItemEnum::IsNatural(_) => true,
            _ => false,
        }
    }

    fn is_pair(&self) -> bool {
        !self.is_natural()
    }

    fn new_natural(v: u32) -> Self {
        Self(PairItemEnum::IsNatural(v))
    }

    fn new_pair(pair: Pair) -> Self {
        Self(PairItemEnum::IsPair(Box::new(pair)))
    }

    fn depth(&self) -> u32 {
        match &self.0 {
            PairItemEnum::IsNatural(_) => 1,
            PairItemEnum::IsPair(p) => 1 + p.depth(),
        }
    }

    fn natural(&self) -> Option<u32> {
        match self.0 {
            PairItemEnum::IsNatural(v) => Some(v),
            _ => None,
        }
    }

    fn pair_ref(&self) -> Option<&Pair> {
        match self.0 {
            PairItemEnum::IsPair(ref bp) => Some(&*bp),
            _ => None,
        }
    }

    fn pair_mut_ref(&mut self) -> Option<&mut Pair> {
        match self.0 {
            PairItemEnum::IsPair(ref mut bp) => Some(&mut *bp),
            _ => None,
        }
    }
    // explode a snailfish pair (maybe) by returning the left, right, carries and the new Pair
    // note the pi is either the left or right side of a snailfish.  The algorithm finds the
    // deepest pair and explodes that.
    fn explode(&self) -> (u32, Self, u32) {
        assert!(self.is_pair(), "Can only explode a pair!");
        if let Some(pair_ref) = self.pair_ref() {
            if pair_ref.depth() > 1 {
            // need to workout which side to go down.
                if pair_ref.left.depth() >= pair_ref.right.depth() {
                    let (cl, new_left, cr) = pair_ref.left.explode();
                    // here add the cr to the first natural on the left
                    let new_right = pair_ref.right.add_left(cr);
                    let new_pair = Pair::new_from_pairitems(new_left, new_right);
                    return (cl, Self::new_pair(new_pair), 0);
                } else {
                    let (cl, new_right, cr) = pair_ref.right.explode();
                    let new_left = pair_ref.left.add_right(cl);
                    let new_pair = Pair::new_from_pairitems(new_left, new_right);
                    return (0, Self::new_pair(new_pair), cr);
                }
            } else {
                // we are at the deepest level.  Convert a [3,4] into (3, Value(0), 4)
                assert!(pair_ref.left.is_natural());
                assert!(pair_ref.right.is_natural());
                let left = pair_ref.left.natural().unwrap();
                let right = pair_ref.right.natural().unwrap();
                return (left, PairItem::new_natural(0), right)
            }
        }
        unreachable!();
    }

    fn add_left(&self, natural: u32) -> Self {
        if let Some(pair_ref) = self.pair_ref() {
            Self::new_pair(Pair::new_from_pairitems(pair_ref.left.add_left(natural), pair_ref.right.clone()))
        } else {
            Self::new_natural(natural + self.natural().unwrap())
        }
    }

    fn add_right(&self, natural: u32) -> Self {
        if let Some(pair_ref) = self.pair_ref() {
            Self::new_pair(Pair::new_from_pairitems(pair_ref.left.clone(), pair_ref.right.add_right(natural)))
        } else {
            Self::new_natural(natural + self.natural().unwrap())
        }
    }

    /// if the item is a natural and > 9, then split it, otherwise, if it is a pari, see if the
    /// pair needs spliting (i.e. recursive).  Returns true when a split has occurred.
    fn split(&mut self) -> bool {
        if let Some(v) = self.natural() {
            if v > 9 {
                self.0 = PairItemEnum::IsPair(Box::new(Self::split_it(v)));
                return true;
            }
        }
        self.pair_mut_ref()
            .and_then(|pair_ref| Some(pair_ref.split()))
            .unwrap_or(false)
    }

    fn split_it(num: u32) -> Pair {
        assert!(num >= 10);
        let left = num / 2;
        let right = num - left;
        Pair::new_from_naturals(left, right)
    }
}


#[derive(Clone, Debug)]
struct Pair {
    left: PairItem,
    right: PairItem,
}


impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}


impl Pair {

    fn new_from_naturals(left: u32, right: u32) -> Self {
        Self::new_from_pairitems(PairItem::new_natural(left), PairItem::new_natural(right))
    }

    fn new_from_pairitems(left: PairItem, right: PairItem) -> Self {
        Self {left, right}
    }

    // work out the depth of a snailfish; recurses to the bottom of the fish
    fn depth(&self) -> u32 {
        self.depth_left().max(self.depth_right())
    }

    fn depth_left(&self) -> u32 {
        self.left.depth()
    }

    fn depth_right(&self) -> u32 {
        self.right.depth()
    }

    /// Reduces itself if the depth is more than 4.
    fn explode(&mut self) {
        let (_, pi, _) = PairItem::new_pair(self.clone()).explode();
        if let PairItem(PairItemEnum::IsPair(pair)) = pi {
            self.left = pair.left;
            self.right = pair.right;
        } else {
            panic!("Must be able to pull the left and right into self!");
        }
    }

    /// if the left or right is greater than 9 then split into left and right numbers.
    fn split(&mut self) -> bool {
        self.left.split() || self.right.split()
    }

    /// reduce a Pair by exploding and then spliting and continuing that sequence until there are
    /// no more explodes or splits.
    fn reduce(&mut self) {
        loop {
            if self.depth() > 4 {
                self.explode();
                continue;
            }
            if !self.split() {
                break;
            }
        }
    }

    fn add(&self, other: &Self) -> Self {
        let mut sum: Self = Self::new_from_pairitems(
            PairItem::new_pair(self.clone()),
            PairItem::new_pair(other.clone()));
        sum.reduce();
        sum
    }

}


#[derive(Clone, Debug, PartialEq)]
enum DecodeState {
    Start,     // before a digit is received after the '[' char.
    Left,      // reading left digits (i.e. before the ,)
    Right,     // reading right digits (i.e. after a comma)
}

use DecodeState::*;

impl FromStr for Pair {
    type Err = DecodeError;

    // Decode the following string into a Target.
    // [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    // into a SnailFish recursively.
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut chars = line.chars();
        let mut state: DecodeState = Start;
        let mut stack: Vec<PairItem> = Vec::new();
        let mut oc: Option<char> = chars.next();
        loop {
            if let Some(c) = oc {
                if c == '[' {
                    // do we need to signal the start of a snailfish?
                    state = Left;
                    oc = chars.next();
                    continue
                } else if c == ']' {
                    // we need to take the two items off of the stack, turn them into a
                    // single SnailFish and then fetch the next character.
                    if state == Left {
                        return Err(DecodeError::InvalidError(
                                format!("Found a ']' before a , in a Pair? {}", line)));
                    }
                    let some_pair = stack.pop().and_then(
                        |right| stack.pop().and_then(
                            |left| Some(Pair::new_from_pairitems(left, right))));
                    if let Some(pair) = some_pair {
                        //stack.push(PairItem::SnailFish(Box::new(snailfish)));
                        stack.push(PairItem::new_pair(pair));
                        oc = chars.next();
                        state = Start;
                        continue;
                    }
                    return Err(DecodeError::InvalidError(
                            format!("Insufficient items on the stack to make a new pair? {}", line)));
                } else if c == ',' {
                    // need to check that that this isn't the third?
                    if state == Right {
                        return Err(DecodeError::InvalidError(
                                format!("More than 1 ',' in a Pair? {}", line)));
                    }
                    if state == Start && stack.len() == 0 {
                        return Err(DecodeError::InvalidError(
                                format!("Received ',' before any left in a Pair? {}", line)));
                    }
                    oc = chars.next();
                    state = Right;
                    continue;
                }
                if c.is_digit(10) {
                    // decode digits until there isn't one.
                    let mut num = c.to_digit(10).unwrap();
                    loop {
                        oc = chars.next();
                        if let Some(c) = oc {
                            if c.is_digit(10) {
                                num = num * 10 + c.to_digit(10).unwrap();
                                continue;
                            }
                        } else {
                            return Err(DecodeError::InvalidError(
                                    format!("Ran out of line whilst decoding digits? {} in {}",c, line)));
                        }
                        // now stack up the value in a PairItem
                        //stack.push(PairItem::Value(num));
                        stack.push(PairItem::new_natural(num));
                        break;
                    }
                    continue;
                }
                // it's not something we recognise, so error out
                return Err(DecodeError::InvalidError(format!("Invalid character received! {}", c)));

            }
            break;
        }
        if stack.len() != 1 {
            return Err(DecodeError::InvalidError(format!("Decode error, too many values on the stack: {:?}", &stack)));
        }
        if let PairItem(PairItemEnum::IsPair(pair)) = stack.pop().unwrap() {
            return Ok(*pair);
        }
        Err(DecodeError::InvalidError(String::from("Something strange went on??")))
    }
}



pub fn day18_1() {
    println!("Day 18: Snailfish maths, part 1");
    let pairs = utils::read_file_single_result::<Pair>("./input/day18-test.txt")
        .expect("Couldn't read file");
    //println!("Input: {:?}", &pairs);

    //let l1: Vec<Pair> = ["[2,2]","[3,3]","[4,4]","[5,5]","[6,6]"]
        //.iter().map(|s| Pair::from_str(s).unwrap()).collect();
    let mut v: Pair = pairs[0].clone();
    println!("Initial value: {}", &v);
    for p in pairs.iter().skip(1) {
        v = v.add(p);
    }
    println!("Result: {}", &v);

}

