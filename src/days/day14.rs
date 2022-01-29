//--- Day 14: Extended Polymerization ---

//The incredible pressures at this depth are starting to put a strain on your submarine. The submarine has polymerization equipment that would produce suitable materials to reinforce the submarine, and the nearby volcanically-active caves should even have the necessary input elements in sufficient quantities.


use std::fmt;
use std::collections::HashMap;
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
fn do_n_insertions<S>(template: S, n: u32, rules: &HashMap<(char,char),char>) -> String
    where S: AsRef<str>
{
    let mut out: String = template.as_ref().to_string();
    for _ in 0..n {
        out = do_insertion(out, &rules);
    }
    out
}


fn counts<S>(poly: S) -> HashMap<char, u32>
    where S: AsRef<str>
{
    let mut out: HashMap<char, u32> = HashMap::new();
    for c in poly.as_ref().chars() {
        *out.entry(c).or_insert(0) += 1;
    }
    out
}


fn calc_part1_result(freqs: &HashMap<char, u32>) -> u32 {
    let mut nums: Vec<u32> = freqs.values().cloned().collect();
    nums.sort_by(|a,b| b.cmp(a));
    nums[0] - nums[nums.len()-1]
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
    println!("Result: {}", calc_part1_result(&freqs));
}


pub fn day14_2() {
    println!("Day 14: Extended Polymerization, part 2");
    let lines = utils::read_file_single_result::<String>("./input/day14.txt")
        .expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let input = parse(&lines).expect("Parsing went wrong?");
    println!("parsed input: {:?}", input);
}
