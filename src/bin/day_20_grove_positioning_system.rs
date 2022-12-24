// Advent Of Code
// https://adventofcode.com/2022/day/4
//
// --- Day 20: Grove Positioning System ---
// It's finally time to meet back up with the Elves. When you try to contact them, however, you get no reply. Perhaps you're out of range?
// 
// You know they're headed to the grove where the star fruit grows, so if you can figure out where that is, you should be able to meet back up with them.
// 
// Fortunately, your handheld device has a file (your puzzle input) that contains the grove's coordinates! Unfortunately, the file is encrypted - just in case the device were to fall into the wrong hands.
// 
// Maybe you can decrypt it?
// 
// When you were still back at the camp, you overheard some Elves talking about coordinate file encryption. The main operation involved in decrypting the file is called mixing.
// 
// The encrypted file is a list of numbers. To mix the file, move each number forward or backward in the file a number of positions equal to the value of the number being moved. The list is circular, so moving a number off one end of the list wraps back around to the other end as if the ends were connected.
// 
// For example, to move the 1 in a sequence like 4, 5, 6, 1, 7, 8, 9, the 1 moves one position forward: 4, 5, 6, 7, 1, 8, 9. To move the -2 in a sequence like 4, -2, 5, 6, 7, 8, 9, the -2 moves two positions backward, wrapping around: 4, 5, 6, 7, 8, -2, 9.
// 
// The numbers should be moved in the order they originally appear in the encrypted file. Numbers moving around during the mixing process do not change the order in which the numbers are moved.
// 
// Consider this encrypted file:
// 
// 1
// 2
// -3
// 3
// -2
// 0
// 4
// Mixing this file proceeds as follows:
// 
// Initial arrangement:
// 1, 2, -3, 3, -2, 0, 4
// 
// 1 moves between 2 and -3:
// 2, 1, -3, 3, -2, 0, 4
// 
// 2 moves between -3 and 3:
// 1, -3, 2, 3, -2, 0, 4
// 
// -3 moves between -2 and 0:
// 1, 2, 3, -2, -3, 0, 4
// 
// 3 moves between 0 and 4:
// 1, 2, -2, -3, 0, 3, 4
// 
// -2 moves between 4 and 1:
// 1, 2, -3, 0, 3, 4, -2
// 
// 0 does not move:
// 1, 2, -3, 0, 3, 4, -2
// 
// 4 moves between -3 and 0:
// 1, 2, -3, 4, 0, 3, -2
// Then, the grove coordinates can be found by looking at the 1000th, 2000th, and 3000th numbers after the value 0, wrapping around the list as necessary. In the above example, the 1000th number after 0 is 4, the 2000th is -3, and the 3000th is 2; adding these together produces 3.
// 
// Mix your encrypted file exactly once. What is the sum of the three numbers that form the grove coordinates?

use aoc2022::load_input;
use std::error::Error;

fn solve_pt1(input_text: &str) -> u64 {
    0
}

fn solve_pt2(input_text: &str) -> u64 {
    1
}

fn main() -> Result<(), Box<dyn Error>> {
    const FILENAME: &str = "data/day_20_input.txt";
    let input_text = load_input(FILENAME);

    print!("Part one: {:#?}\n", solve_pt1(&input_text));
    // Correct: AAA

    print!("Part two: {:#?}\n", solve_pt2(&input_text));
    // Correct: BBB

    Ok(())
}

// Example tests
#[cfg(test)]
mod example {
    use super::*;

    const TEST_DATA: &str = "1
2
-3
3
-2
0
4";
    const ANS_PT1: u64 = 3;
    const ANS_PT2: u64 = 1;

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(TEST_DATA), ANS_PT1);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2(TEST_DATA), ANS_PT2);
    }
}
