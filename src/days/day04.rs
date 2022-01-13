// --- Day 4: Giant Squid ---

//You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep that you can't see any sunlight. What you can see, however, is a giant squid that has attached itself to the outside of your submarine.

//Maybe it wants to play bingo?


use std::str::FromStr;
use std::collections::HashSet;

use crate::utils;


use thiserror::Error;


// we need to decode the data as:
//
// 1. line of drawn numbers, split by ','
// 2. A blank line.
// 3. 5 lines of numbers split by spaces; 5 numbers in a row.

#[derive(Error, Debug, Clone)]
pub enum DecodeError {
    #[error("Invalid command")]
    InvalidError(String),
}


#[derive(Clone, Debug)]
struct Draws(Vec<u32>);


impl FromStr for Draws {
    type Err = DecodeError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        line
            .split(",")
            .map(|l| l.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .map(|v| Self(v))
            .map_err(|e| DecodeError::InvalidError(format!("Couldn't parse: {}", e)))
    }
}

#[derive(Clone, Debug)]
struct Line(Vec<u32>);


impl FromStr for Line {
    type Err = DecodeError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let rs = line
            .split_whitespace()
            .map(|l| l.parse::<u32>())
            .take(5)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| DecodeError::InvalidError(format!("Couldn't parse: {}", e)))?;
        if rs.len() != 5 {
            return Err(DecodeError::InvalidError(format!("Not enough numbers on line: {}", line)));
        }
        Ok(Self(rs))
    }
}


// 5 lines of 5 numbers.  One array arranged as 5 * lines of 5 numbers
// contiguously.  To access line i, column j = i*5 + j
#[derive(Clone, Debug)]
struct Board(Vec<u32>);


impl Board {

    fn at(&self, line: usize, column: usize) -> u32 {
        self.0[line*5+column]
    }

    pub fn check_board_at_move(&self, so_far: &HashSet<u32>) -> bool {
        for i in 0..=4 {
            // check the line
            if (0..=4).fold(true, |acc, j| acc && so_far.contains(&self.at(i, j))) {
                return true;
            }
            // check the column
            if (0..=4).fold(true, |acc, j| acc && so_far.contains(&self.at(j, i))) {
                return true;
            }
        }
        false
    }

    pub fn sum_unmarked_numbers(&self, so_far: &HashSet<u32>) -> u32 {
        self.0
            .iter()
            .fold(0, |acc, v| acc + if so_far.contains(v) {0} else {*v})
    }

    // parse a block of 5 lines into a board
    pub fn parse_one<S>(lines: &[S]) -> Result<Self, DecodeError>
        where S: AsRef<str>
    {
        if lines.len() != 5 {
            return Err(DecodeError::InvalidError(format!("Not passed 5 board lines: {}", lines.len())));
        }
        // get a Vec<Line> parsed from 5 lines.
        let rs = lines
            .iter()
            .map(|l| l.as_ref().parse::<Line>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| DecodeError::InvalidError(format!("Couldn't pase the board: {}", e)))?;
        // convert the v lines into a single Vec<u32>
        Ok(Board(
            rs.iter()
            .map(|l| l.0.clone())
            .into_iter()
            .flatten()
            .collect()))
    }
}


// parse the lines; an empty line indicates the start of a board; so then take 5 lines and parse
// the Lines from that tand then add them to the Board.
fn parse_boards<S>(lines: &[S]) -> Result<Vec<Board>, DecodeError>
    where S: AsRef<str>
{
    let length = lines.len();
    let mut i: usize = 0;
    let mut boards: Vec<Board> = Vec::with_capacity((length / 6) + 1);
    while i < length {
        if lines[i].as_ref().len() == 0 {
            i += 1;
            continue;
        }
        // pass the next 5 lines to parse_a_board
        if i + 4 >= length {
            return Err(DecodeError::InvalidError("Insufficient lines for a board?".to_string()));
        }
        let board = Board::parse_one(&lines[i..i+5])?;
        boards.push(board);
        i += 5;
    }
    Ok(boards)
}


// finds the first board that has won (check_board_at_move).
fn check_for_a_winning_board(so_far: &HashSet<u32>, boards: &[Board]) -> Option<usize> {
    for (i, board) in boards.iter().enumerate() {
        if board.check_board_at_move(so_far) {
            // return the index of the board that has a line or column
            return Some(i);
        }
    }
    None
}


// find the winning board index using the draws in sequence until we have a winning board
fn find_first_winning_board(draws: &Draws, boards: &[Board]) -> u32 {
    let mut so_far: HashSet<u32> = HashSet::with_capacity(draws.0.len());
    for &draw in draws.0.iter() {
        so_far.insert(draw);
        if let Some(index) = check_for_a_winning_board(&so_far, boards) {
            // sum up the unmarked numbers
            let sum = boards[index].sum_unmarked_numbers(&so_far);
            // return the full sum
            return sum * draw;
        }
    }
    panic!("Should have found a winning board?")
}


// see if ALL the boards have won; returns the indexes of boards that haven't yet won.
fn check_for_losing_boards(so_far: &HashSet<u32>, boards: &[Board]) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity(boards.len());
    for (i, board) in boards.iter().enumerate() {
        if !board.check_board_at_move(so_far) {
            // add a board index if the board is not complete
            result.push(i);
        }
    }
    result
}


// find the winning board index using the draws in sequence until we have a winning board
fn find_last_winning_board(draws: &Draws, boards: &[Board]) -> u32 {
    let mut so_far: HashSet<u32> = HashSet::with_capacity(draws.0.len());
    let mut last_round: Vec<usize> = vec![];
    for &draw in draws.0.iter() {
        so_far.insert(draw);
        let round = check_for_losing_boards(&so_far, boards);
        if round.len() == 0 {
            // we've just had the last board.
            if last_round.len() != 1 {
                panic!("last round wasn't a single board when the final board was won??");
            }
            // sum up the unmarked numbers
            let sum = boards[last_round[0]].sum_unmarked_numbers(&so_far);
            // return the full sum
            return sum * draw;
        }
        last_round = round;
    }
    panic!("Should have found a winning board?")
}


pub fn day4_1() {
    println!("Giant Squid bingo!");
    let read_lines = utils::read_file::<String>("./input/day04.txt");
    let lines = read_lines.iter().cloned().collect::<Result<Vec<_>, _>>().expect("Failed to read file");

    let draws = lines[0].parse::<Draws>();
    //println!("Draws are {:?}", draws);
    let boards = parse_boards(&lines[1..]);
    //println!("Boards are: {:?}", boards);
    let winner = find_first_winning_board(&draws.unwrap(), &boards.unwrap());
    println!("Winner index: {}", winner);
}


pub fn day4_2() {
    println!("Giant Squid bingo! Squid has to win.");
    let read_lines = utils::read_file::<String>("./input/day04.txt");
    let lines = read_lines.iter().cloned().collect::<Result<Vec<_>, _>>().expect("Failed to read file");

    let draws = lines[0].parse::<Draws>();
    //println!("Draws are {:?}", draws);
    let boards = parse_boards(&lines[1..]);
    //println!("Boards are: {:?}", boards);
    let winner = find_last_winning_board(&draws.unwrap(), &boards.unwrap());
    println!("Winner index: {}", winner);
}
