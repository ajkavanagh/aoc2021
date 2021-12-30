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
