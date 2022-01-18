//--- Day 10: Syntax Scoring ---

//You ask the submarine to determine the best route out of the deep-sea cave, but it only replies:

//Syntax error in navigation subsystem on line: all of them

//All of them?! The damage is worse than you thought. You bring up a copy of the navigation subsystem (your puzzle input).


use crate::utils;


fn parse_lines(lines: &[String]) -> u32 {
    let mut syntax_sum: u32 = 0;
    for line in lines.iter() {
        let result = check_line(&line);
        println!("Line '{}' decodes as '{:?}'", &line, &result);
        if let Some(c) = result {
            if let Some(v) = char_to_value(&c) {
                syntax_sum += v;
            }
        }
    }
    syntax_sum
}


/// parse a line of braces.  Return an error with the brace if it went wrong
/// If the line is okay, return None, otherwise return the offending char.
fn check_line(line: &String) -> Option<char> {
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
                        return Some(c);
                    }
                } else {
                    // line is incomplete
                    //return Some(c);
                    return None;
                }
            },
            _ => panic!("Unexpected character {}.", c),
        }
    }
    None
}


fn char_to_value(c: &char) -> Option<u32> {
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
    let res = parse_lines(&lines);
    println!("Syntax sum is {}", res);
}
