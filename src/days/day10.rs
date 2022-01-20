//--- Day 10: Syntax Scoring ---

//You ask the submarine to determine the best route out of the deep-sea cave, but it only replies:

//Syntax error in navigation subsystem on line: all of them

//All of them?! The damage is worse than you thought. You bring up a copy of the navigation subsystem (your puzzle input).


use crate::utils;


#[derive(Clone, Debug)]
enum SyntaxResult {
    Corrupted(char),
    Incomplete(String),
    Invalid(char),
    Valid,
}

use SyntaxResult::*;


fn parse_lines_p1(lines: &[String]) -> u32 {
    let mut syntax_sum: u32 = 0;
    for line in lines.iter() {
        let result = check_line(&line);
        println!("Line '{}' decodes as '{:?}'", &line, &result);
        match result {
            Corrupted(c) => {
                if let Some(v) = char_to_value_p1(&c) {
                    syntax_sum += v;
                }
            },
            _ => {},
        }
    }
    syntax_sum
}


fn parse_lines_p2(lines: &[String]) -> u64 {
    let mut scores: Vec<u64> = Vec::new();
    for line in lines.iter() {
        let result = check_line(&line);
        println!("Line '{}' decodes as '{:?}'", &line, &result);
        match result {
            Incomplete(s) => {
                let v = s.chars().fold(0, |acc, c| {
                    acc * 5 + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => 0,
                    }
                });
                println!("The score is {}", v);
                scores.push(v);
            },
            _ => {},
        }
    }
    scores.sort();
    println!("Scores are: {:?}", scores);
    scores[scores.len()/2]
}


/// parse a line of braces.  Return an error with the brace if it went wrong
/// If the line is okay, return None, otherwise return the offending char.
fn check_line(line: &String) -> SyntaxResult {
    let mut pairs: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            '{' => { pairs.push('}') },
            '(' => { pairs.push(')') },
            '<' => { pairs.push('>') },
            '[' => { pairs.push(']') },
            '}' | ')' | '>' | ']' => {
                if let Some(r) = pairs.pop() {
                    if r != c {
                        return Corrupted(c);
                    }
                } else {
                    // Received more closes that opens?
                    return Invalid(c);
                }
            },
            _ => return Invalid(c),
        }
    }
    if pairs.len() > 0 {
        return Incomplete(pairs.iter().rev().collect());
    }
    Valid
}


fn char_to_value_p1(c: &char) -> Option<u32> {
    match c {
        ')' => Some(3),
        ']' => Some(57),
        '}' => Some(1197),
        '>' => Some(25137),
        _ => None,
    }
}


pub fn day10_1() {
    println!("Day 10: Syntax Scoring, part 1");
    let lines = utils::read_file_single_result::<String>("./input/day10.txt").expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let res = parse_lines_p1(&lines);
    println!("Syntax sum is {}", res);
}


pub fn day10_2() {
    println!("Day 10: Syntax Scoring, part 2");
    let lines = utils::read_file_single_result::<String>("./input/day10.txt").expect("Couldn't read file");
    println!("Input: {:?}", &lines);
    let res = parse_lines_p2(&lines);
    println!("Syntax sum is {}", res);
}
