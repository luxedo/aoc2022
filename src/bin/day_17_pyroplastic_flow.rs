// Advent Of Code
// https://adventofcode.com/2022/day/4
//
//  --- Day 17: Pyroclastic Flow ---
// Your handheld device has located an alternative exit from the cave for you and the elephants. The ground is rumbling almost continuously now, but the strange valves bought you some time. It's definitely getting warmer in here, though.
//
// The tunnels eventually open into a very tall, narrow chamber. Large, oddly-shaped rocks are falling into the chamber from above, presumably due to all the rumbling. If you can't work out where the rocks will fall next, you might be crushed!
//
// The five types of rocks have the following peculiar shapes, where # is rock and . is empty space:
//
// ####
//
// .#.
// ###
// .#.
//
// ..#
// ..#
// ###
//
// #
// #
// #
// #
//
// ##
// ##
// The rocks fall in the order shown above: first the - shape, then the + shape, and so on. Once the end of the list is reached, the same order repeats: the - shape falls first, sixth, 11th, 16th, etc.
//
// The rocks don't spin, but they do get pushed around by jets of hot gas coming out of the walls themselves. A quick scan reveals the effect the jets of hot gas will have on the rocks as they fall (your puzzle input).
//
// For example, suppose this was the jet pattern in your cave:
//
// >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
// In jet patterns, < means a push to the left, while > means a push to the right. The pattern above means that the jets will push a falling rock right, then right, then right, then left, then left, then right, and so on. If the end of the list is reached, it repeats.
//
// The tall, vertical chamber is exactly seven units wide. Each rock appears so that its left edge is two units away from the left wall and its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).
//
// After a rock appears, it alternates between being pushed by a jet of hot gas one unit (in the direction indicated by the next symbol in the jet pattern) and then falling one unit down. If any movement would cause any part of the rock to move into the walls, floor, or a stopped rock, the movement instead does not occur. If a downward movement would have caused a falling rock to move into the floor or an already-fallen rock, the falling rock stops where it is (having landed on something) and a new rock immediately begins falling.
//
// Drawing falling rocks with @ and stopped rocks with #, the jet pattern in the example above manifests as follows:
//
// The first rock begins falling:
// |..@@@@.|
// |.......|
// |.......|
// |.......|
// +-------+
//
// Jet of gas pushes rock right:
// |...@@@@|
// |.......|
// |.......|
// |.......|
// +-------+
//
// Rock falls 1 unit:
// |...@@@@|
// |.......|
// |.......|
// +-------+
//
// Jet of gas pushes rock right, but nothing happens:
// |...@@@@|
// |.......|
// |.......|
// +-------+
//
// Rock falls 1 unit:
// |...@@@@|
// |.......|
// +-------+
//
// Jet of gas pushes rock right, but nothing happens:
// |...@@@@|
// |.......|
// +-------+
//
// Rock falls 1 unit:
// |...@@@@|
// +-------+
//
// Jet of gas pushes rock left:
// |..@@@@.|
// +-------+
//
// Rock falls 1 unit, causing it to come to rest:
// |..####.|
// +-------+
//
// A new rock begins falling:
// |...@...|
// |..@@@..|
// |...@...|
// |.......|
// |.......|
// |.......|
// |..####.|
// +-------+
//
// Jet of gas pushes rock left:
// |..@....|
// |.@@@...|
// |..@....|
// |.......|
// |.......|
// |.......|
// |..####.|
// +-------+
//
// Rock falls 1 unit:
// |..@....|
// |.@@@...|
// |..@....|
// |.......|
// |.......|
// |..####.|
// +-------+
//
// Jet of gas pushes rock right:
// |...@...|
// |..@@@..|
// |...@...|
// |.......|
// |.......|
// |..####.|
// +-------+
//
// Rock falls 1 unit:
// |...@...|
// |..@@@..|
// |...@...|
// |.......|
// |..####.|
// +-------+
//
// Jet of gas pushes rock left:
// |..@....|
// |.@@@...|
// |..@....|
// |.......|
// |..####.|
// +-------+
//
// Rock falls 1 unit:
// |..@....|
// |.@@@...|
// |..@....|
// |..####.|
// +-------+
//
// Jet of gas pushes rock right:
// |...@...|
// |..@@@..|
// |...@...|
// |..####.|
// +-------+
//
// Rock falls 1 unit, causing it to come to rest:
// |...#...|
// |..###..|
// |...#...|
// |..####.|
// +-------+
//
// A new rock begins falling:
// |....@..|
// |....@..|
// |..@@@..|
// |.......|
// |.......|
// |.......|
// |...#...|
// |..###..|
// |...#...|
// |..####.|
// +-------+
// The moment each of the next few rocks begins falling, you would see this:
//
// |..@....|
// |..@....|
// |..@....|
// |..@....|
// |.......|
// |.......|
// |.......|
// |..#....|
// |..#....|
// |####...|
// |..###..|
// |...#...|
// |..####.|
// +-------+
//
// |..@@...|
// |..@@...|
// |.......|
// |.......|
// |.......|
// |....#..|
// |..#.#..|
// |..#.#..|
// |#####..|
// |..###..|
// |...#...|
// |..####.|
// +-------+
//
// |..@@@@.|
// |.......|
// |.......|
// |.......|
// |....##.|
// |....##.|
// |....#..|
// |..#.#..|
// |..#.#..|
// |#####..|
// |..###..|
// |...#...|
// |..####.|
// +-------+
//
// |...@...|
// |..@@@..|
// |...@...|
// |.......|
// |.......|
// |.......|
// |.####..|
// |....##.|
// |....##.|
// |....#..|
// |..#.#..|
// |..#.#..|
// |#####..|
// |..###..|
// |...#...|
// |..####.|
// +-------+
//
// |....@..|
// |....@..|
// |..@@@..|
// |.......|
// |.......|
// |.......|
// |..#....|
// |.###...|
// |..#....|
// |.####..|
// |....##.|
// |....##.|
// |....#..|
// |..#.#..|
// |..#.#..|
// |#####..|
// |..###..|
// |...#...|
// |..####.|
// +-------+
//
// |..@....|
// |..@....|
// |..@....|
// |..@....|
// |.......|
// |.......|
// |.......|
// |.....#.|
// |.....#.|
// |..####.|
// |.###...|
// |..#....|
// |.####..|
// |....##.|
// |....##.|
// |....#..|
// |..#.#..|
// |..#.#..|
// |#####..|
// |..###..|
// |...#...|
// |..####.|
// +-------+
//
// |..@@...|
// |..@@...|
// |.......|
// |.......|
// |.......|
// |....#..|
// |....#..|
// |....##.|
// |....##.|
// |..####.|
// |.###...|
// |..#....|
// |.####..|
// |....##.|
// |....##.|
// |....#..|
// |..#.#..|
// |..#.#..|
// |#####..|
// |..###..|
// |...#...|
// |..####.|
// +-------+
//
// |..@@@@.|
// |.......|
// |.......|
// |.......|
// |....#..|
// |....#..|
// |....##.|
// |##..##.|
// |######.|
// |.###...|
// |..#....|
// |.####..|
// |....##.|
// |....##.|
// |....#..|
// |..#.#..|
// |..#.#..|
// |#####..|
// |..###..|
// |...#...|
// |..####.|
// +-------+
// To prove to the elephants your simulation is accurate, they want to know how tall the tower will get after 2022 rocks have stopped (but before the 2023rd rock begins falling). In this example, the tower of rocks will be 3068 units tall.
//
// How many units tall will the tower of rocks be after 2022 rocks have stopped falling?

use aoc2022::load_input;
use std::error::Error;

fn solve_pt1(input_text: &str) -> u64 {
    0
}

fn solve_pt2(input_text: &str) -> u64 {
    1
}

fn main() -> Result<(), Box<dyn Error>> {
    const FILENAME: &str = "data/day_X_input.txt";
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

    const abc: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";
    const TEST_DATA: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    const ANS_PT1: u64 = 3068;
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
