// Day 1 Part 1 - Expense report
//
// Specifically, they need you to find the two entries that sum to 2020 and then multiply those two
// numbers together.

//For example, suppose your expense report contained the following:

//1721
//979
//366
//299
//675
//1456

//In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together
//produces 1721 * 299 = 514579, so the correct answer is 514579.

use std::str::FromStr;
use std::num::ParseIntError;

use crate::utils;

const NUMBERS: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];


fn count_depth_increases(numbers: &[u32]) -> Result<u32, String> {
    let mut n1 = numbers.get(0).expect("No numbers passed");
    let mut count: u32 = 0;
    for n2 in numbers[1..].iter() {
        if n2 > n1 {
            count += 1;
        }
        n1 = n2;
    }
    Ok(count)
}


fn count_depth_increases3(numbers: &[u32]) -> Result<u32, String> {
    if numbers.len() < 4 {
        return Err("Not enough numbers".to_string());
    }
    let mut count: u32 = 0;
    let mut sum: u32 = 0;
    let mut ns: [u32; 3] = [0; 3];
    for (i, n) in numbers.iter().enumerate() {
        if i < 3 {
            ns[i] = *n;
            sum += n;
            continue;
        }
        let next_sum = sum - ns[i % 3] + n;
        if next_sum > sum {
            count += 1;
        }
        sum = next_sum;
        ns[i % 3] = *n;
    }
    Ok(count)
}



fn extract_numbers(parsed: &[Result<u32, ParseIntError>]) -> Vec<u32> {
    parsed
        .iter()
        .map(|x| {
            match *x {
                Ok(v) => v,
                Err(_) => 0 as u32,
            }
        })
        .collect()
}

pub fn day1_1() {
    println!("First let's just do the test with the depths:");
    match count_depth_increases(&NUMBERS) {
        Ok(n) => println!("The number of increases is {}", n),
        Err(s) => println!("{0}", s),
    }

    println!("Now let's read the depths file and then find the number of increases:");
    let parsed_numbers = utils::read_file::<u32>("./input/day1-1.txt");
    let numbers = extract_numbers(&parsed_numbers);
    match count_depth_increases(&numbers) {
        Ok(n) => println!("The number of increases is {}", n),
        Err(s) => println!("{0}", s),
    }
}


pub fn day1_2() {
    println!("First let's just do the test with the depths:");
    match count_depth_increases3(&NUMBERS) {
        Ok(n) => println!("The number of increases is {}", n),
        Err(s) => println!("{0}", s),
    }

    println!("Now let's read the depths file and then find the number of increases:");
    let parsed_numbers = utils::read_file::<u32>("./input/day1-1.txt");
    let numbers = extract_numbers(&parsed_numbers);
    match count_depth_increases3(&numbers) {
        Ok(n) => println!("The number of increases is {}", n),
        Err(s) => println!("{0}", s),
    }
}
