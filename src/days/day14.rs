//--- Day 14: Extended Polymerization ---

//The incredible pressures at this depth are starting to put a strain on your submarine. The submarine has polymerization equipment that would produce suitable materials to reinforce the submarine, and the nearby volcanically-active caves should even have the necessary input elements in sufficient quantities.


use std::fmt;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::utils;


#[derive(Clone, Debug)]
struct Input {
    template: String,
    rules: HashMap<(char, char), char>,
}


fn parse<S>(lines: &[S]) -> Result<Input, String>
    where S: AsRef<str> + fmt::Display
{
    let mut rules: HashMap<(char, char), char> = HashMap::new();
    let template: String;
    let mut line_iter = lines.iter();

    // parse the template
    if let Some(line) = line_iter.next() {
        template = line.as_ref().trim().to_string();
    } else {
        return Err(String::from("No lines supplied??"));
    }
    // skip the blank line.
    if let Some(line) = line_iter.next() {
        if line.as_ref().trim() != "" {
            return Err(String::from("No blank line after template?"));
        }
    }
    // now read the rules.
    while let Some(line) = line_iter.next() {
        let line = line.as_ref().trim();
        if line == "" {
            continue;
        }
        let parts = line.split(" -> ").into_iter().collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(format!("Rule '{}' isn't a valid rule?", &line));
        }
        let lhs = parts[0].chars().collect::<Vec<_>>();
        // split the lhs
        if lhs.len() != 2 {
            return Err(format!("Expecting just two characters?: {}", &line));
        }
        let rhs = parts[1].chars().collect::<Vec<_>>();
        if rhs.len() != 1 {
            return Err(format!("Expecting just one character?: {}", &line));
        }
        rules.insert((lhs[0], lhs[1]), rhs[0]);
    }
    Ok(Input {template, rules})
}


// perform an insertion
fn do_insertion<S>(template: S, rules: &HashMap<(char,char),char>) -> String
    where S: AsRef<str>
{
    let mut inserts: Vec<char> = Vec::new();
    let tchars = template.as_ref().chars().collect::<Vec<_>>();
    for i in 1..tchars.len() {
        inserts.push(rules.get(&(tchars[i-1], tchars[i])).unwrap().clone());
    }
    let mut out: String = String::from("");
    for (l, r) in tchars.iter().zip(inserts.iter()) {
        out.push(*l);
        out.push(*r);
    }
    out.push(tchars[tchars.len()-1]);
    out
}


// do insertions n times
fn do_n_insertions<S>(template: S, n: u64, rules: &HashMap<(char,char),char>) -> String
    where S: AsRef<str>
{
    let mut out: String = template.as_ref().to_string();
    for _ in 0..n {
        out = do_insertion(out, &rules);
    }
    out
}


fn counts<S>(poly: S) -> HashMap<char, u64>
    where S: AsRef<str>
{
    let mut out: HashMap<char, u64> = HashMap::new();
    for c in poly.as_ref().chars() {
        *out.entry(c).or_insert(0) += 1;
    }
    out
}


fn calc_result(freqs: &HashMap<char, u64>) -> u64 {
    let mut nums: Vec<u64> = freqs.values().cloned().collect();
    nums.sort_by(|a,b| b.cmp(a));
    nums[0] - nums[nums.len()-1]
}

// part 2 - too big more memory or file-systems!  2^40 increase in the template size is
// template * 1e12 bytes; i.e. lots
//
// So a different approach: let's just store the character counts for a 'step' and work down using
// recursive (or use a loop/stack; TBD) calls to resolve a level.  Let's take the initial state:
//
// NNCB
//
// CH -> B
// HH -> N
// CB -> H
// NH -> C
// HB -> C
// HC -> B
// HN -> C
// NN -> C
// BH -> H
// NC -> B
// NB -> B
// BN -> B
// BB -> N
// BC -> B
// CC -> N
// CN -> C
//
// So for level '0', CH has 'no letters' between C and H.
// level '1', CH has 'B=1' lettes between CH.
// level '2', CH has level '1' CB + level '1' BH => 'H=1' + 'H=1' == 'H=2' + 'B=1'
//
// We memoise at each level, so level 2 now has an entry for CH as {H=2, B=1}
// And so on.  So we need to a memoise (which is mut) and then a function that when asked to 'get'
// the value at a level, either has it, or calculates it by asking the next level.  It's turtles
// all the way down.


type Memo = RefCell<HashMap<usize, HashMap<(char,char), HashMap<char, u64>>>>;
type Freq = HashMap<char, u64>;
type Rules = HashMap<(char,char), char>;
type Item = (char, char);


fn get_at_n<'r, 'm>(rules: &'r Rules, memo: &'m Memo, item: Item, step: usize) -> Freq
{
    // see if the item is memoised at this step
    let f = memo.borrow().get(&step).and_then(|freqs| freqs.get(&item).cloned());
    //if let Some(freq) = f {
        //return freq;
    //}
    // othewise we need to calculate the item by getting the one at the step below, if we are not
    // at step 0
    if f.is_none() {
        let c = rules.get(&item).unwrap();
        let mut freq: Freq = HashMap::from([(*c, 1)]);
        // if we are at step 0, we need to construct the step 0 insertion char.
        if step == 0 {
            let mut memob = memo.borrow_mut();
            let entry = memob.entry(step).or_insert(HashMap::new());
            entry.insert(item.clone(), freq.clone());
            return freq;
        }
        // otherwise we need to get the two sets of insertions at the step below.
        // e.g. if it's CH -> B (as a rule) and we want CH, then we get CB and BH freqs, sum them
        // and then add them as CH at this level.
        let freq_lhs = get_at_n(rules, memo, (item.0, *c), step-1);
        let freq_rhs = get_at_n(rules, memo, (*c, item.1), step-1);
        for (p, v) in freq_lhs.clone().iter() {
            *freq.entry(*p).or_insert(0) += *v;
        }
        for (p, v) in freq_rhs.clone().iter() {
            *freq.entry(*p).or_insert(0) += *v;
        }
        // now memoise the value, and then return it
        let mut memob = memo.borrow_mut();
        let entry = memob.entry(step).or_insert(HashMap::new());
        entry.insert(item.clone(), freq.clone());
        return freq;
    }
    f.unwrap()
}


// perform the calculation with the template
fn calc_part2<S>(template: S, rules: &Rules, memo: &Memo, step: usize) -> Freq
    where S: AsRef<str>
{
    let tchars = template.as_ref().chars().collect::<Vec<_>>();
    let mut freq: Freq = HashMap::new();
    for i in 1..tchars.len() {
        let item = (tchars[i-1], tchars[i]);
        let sfreq = get_at_n(rules, memo, item.clone(), step);
        println!("sfreq for {}-{:?} is {:?}", i, &item, &sfreq);
        for (p, v) in sfreq.iter() {
            *freq.entry(*p).or_insert(0) += *v;
        }
        // add in the FHS
        *freq.entry(tchars[i-1]).or_insert(0) += 1;
    }
    // add in the final character
    *freq.entry(tchars[tchars.len()-1]).or_insert(0) += 1;
    freq
}


pub fn day14_1() {
    println!("Day 14: Extended Polymerization, part 1");
    let lines = utils::read_file_single_result::<String>("./input/day14.txt")
        .expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let input = parse(&lines).expect("Parsing went wrong?");
    println!("parsed input: {:?}", input);
    let res = do_n_insertions(&input.template, 10, &input.rules);
    let freqs = counts(&res);
    println!("frequencies: {:?}", freqs);
    println!("Result: {}", calc_result(&freqs));
}


pub fn day14_2() {
    println!("Day 14: Extended Polymerization, part 2");
    let lines = utils::read_file_single_result::<String>("./input/day14.txt")
        .expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let input = parse(&lines).expect("Parsing went wrong?");
    println!("parsed input: {:?}", input);
    let memo: Memo = RefCell::new(HashMap::new());
    let freqs = calc_part2(&input.template, &input.rules, &memo, 39);
    println!("frequencies: {:?}", &freqs);
    println!("Result: {}", calc_result(&freqs));
}
