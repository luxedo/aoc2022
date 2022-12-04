// Advent Of Code
// https://adventofcode.com/2022/day/4
//
//  --- PROBLEM DESCRIPTION ---

use aoc2022::load_input;
use std::error::Error;

fn solve_pt1(input_text: String) -> u64 {
    0
}

fn solve_pt2(input_text: String) -> u64 {
    1
}

fn main() -> Result<(), Box<dyn Error>> {
    const FILENAME: &str = "data/day_X_input.txt";
    let input_text = load_input(FILENAME);

    print!("Part one: {:#?}\n", solve_pt1(input_text.clone()));
    // Correct: AAA

    print!("Part two: {:#?}\n", solve_pt2(input_text.clone()));
    // Correct: BBB

    Ok(())
}

// Example tests
#[cfg(test)]
mod example {
    use super::*;

    const TEST_DATA: &str = "Your data here";
    const ANS_PT1: u64 = 0;
    const ANS_PT2: u64 = 1;

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(TEST_DATA.to_string()), ANS_PT1);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2(TEST_DATA.to_string()), ANS_PT2);
    }
}
