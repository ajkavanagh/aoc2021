// --- Day 3: Binary Diagnostic ---

//The submarine has been making some odd creaking noises, so you ask it to produce a diagnostic report just in case.

//The diagnostic report (your puzzle input) consists of a list of binary numbers which, when decoded properly, can tell you many useful things about the conditions of the submarine. The first parameter to check is the power consumption.

//You need to use the binary numbers in the diagnostic report to generate two new binary numbers (called the gamma rate and the epsilon rate). The power consumption can then be found by multiplying the gamma rate by the epsilon rate.

//Each bit in the gamma rate can be determined by finding the most common bit in the corresponding position of all numbers in the diagnostic report. For example, given the following diagnostic report:

use std::str::FromStr;
use std::num::ParseIntError;

use crate::utils;


use thiserror::Error;


fn aggregate_strings(nums: &[String]) -> String {
    let size = nums[0].len();
    let mut results: Vec<usize> = vec![0; size];
    for num in nums.iter() {
        for (i, c) in num.chars().enumerate() {
            match c {
                '1' => { results[i] += 1; },
                _ => {},
            };
        }
    }
    let half = nums.len() / 2;
    results.iter().map(|&v| if v > half { '1' } else { '0' }).collect::<String>()
}


fn bin_to_nums(s: String) -> (u32, u32) {
    let mut num1: u32 = 0;
    let mut num2: u32 = 0;
    for c in s.chars() {
        num1 *= 2;
        num2 *= 2;
        match c {
            '1' => { num1 += 1; },
            '0' => { num2 += 1; },
            _ => {},
        };
    }
    (num1, num2)
}

fn filter_at(nums: &[String], pos: usize, most: bool) -> Vec<String> {
    // need to count the most common bit at 'pos' and then filter by that.
    // Note the most inverts whether it's most common => least common.
    let mut count: usize = 0;
    let length = nums.len();
    let half = length / 2;
    for num in nums.iter() {
        match num.chars().nth(pos) {
            Some(c) => {
                if c == '1' { count += 1 }
            },
            None => {},
        };
    }
    let mut by_bool = if half * 2 == length {
        count >= half
    } else {
        count > half
    };
    if most == false {
        by_bool = !by_bool;
    }
    let by = if by_bool { '1' } else { '0' };
    nums.iter()
        .filter(|n| n.chars().nth(pos).map_or_else(|| false, |c| c == by))
        .cloned()
        .collect::<Vec<_>>()
}

fn filter_by(nums: &[String], most: bool) -> String {
    // filter the list of nums by filter_at progressively moving across (by pos, starting at 0)
    // until only one number remains.  There must be at least 1 num in nums.
    let mut ns: Vec<String> = nums.to_vec();
    for pos in 0..nums[0].len() {
        ns = filter_at(&ns[..], pos, most);
        if ns.len() == 1 {
            break;
        }
    }
    ns[0].clone()
}

pub fn day3_1() {
    println!("First let's just get the binary test numbers:");
    let r_strings = utils::read_file::<String>("./input/day03.txt");
    match r_strings.iter().cloned().collect::<Result<Vec<String>, _>>() {
        Ok(strings) => {
            println!("Result value is: {:?}", strings);
            let (ones, zeros) = bin_to_nums(aggregate_strings(&strings));
            println!("Calculation {} * {} = {}", ones, zeros, ones * zeros);
        },
        Err(s) => println!("Parsing failed: {}", s),
    };
}


pub fn day3_2() {
    println!("Calculate the Day3 part 2 numbers.");
    let r_strings = utils::read_file::<String>("./input/day03.txt");
    match r_strings.iter().cloned().collect::<Result<Vec<String>, _>>() {
        Ok(strings) => {
            println!("Result value is: {:?}", strings);
            let o2 = filter_by(&strings, true);
            let co2 = filter_by(&strings, false);
            println!("o2: {}, co2: {}", o2, co2);
            let (o2v, _) = bin_to_nums(o2);
            let (co2v, _) = bin_to_nums(co2);
            println!("Result: o2: {}, co2: {}, o2 * co2 = {}", o2v, co2v, o2v * co2v);
        },
        Err(s) => println!("Parsing failed: {}", s),
    };
}
