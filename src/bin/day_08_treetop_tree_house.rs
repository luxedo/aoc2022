// Advent Of Code
// https://adventofcode.com/2022/day/8
//
// --- Day 8: Treetop Tree House ---
// The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.
//
// First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.
//
// The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:
//
// 30373
// 25512
// 65332
// 33549
// 35390
// Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.
//
// A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.
//
// All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:
//
// The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
// The top-middle 5 is visible from the top and right.
// The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
// The left-middle 5 is visible, but only from the right.
// The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
// The right-middle 3 is visible from the right.
// In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
// With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.
//
// Consider your map; how many trees are visible from outside the grid?
//
// --- Part Two ---
// Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house: they would like to be able to see a lot of trees.
//
// To measure the viewing distance from a given tree, look up, down, left, and right from that tree; stop if you reach an edge or at the first tree that is the same height or taller than the tree under consideration. (If a tree is right on the edge, at least one of its viewing distances will be zero.)
//
// The Elves don't care about distant trees taller than those found by the rules above; the proposed tree house has large eaves to keep it dry, so they wouldn't be able to see higher than the tree house anyway.
//
// In the example above, consider the middle 5 in the second row:
//
// 30373
// 25512
// 65332
// 33549
// 35390
// Looking up, its view is not blocked; it can see 1 tree (of height 3).
// Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next to it).
// Looking right, its view is not blocked; it can see 2 trees.
// Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the tree of height 5 that blocks its view).
// A tree's scenic score is found by multiplying together its viewing distance in each of the four directions. For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).
//
// However, you can do even better: consider the tree of height 5 in the middle of the fourth row:
//
// 30373
// 25512
// 65332
// 33549
// 35390
// Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
// Looking left, its view is not blocked; it can see 2 trees.
// Looking down, its view is also not blocked; it can see 1 tree.
// Looking right, its view is blocked at 2 trees (by a massive tree of height 9).
// This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.
//
// Consider each tree on your map. What is the highest scenic score possible for any tree?
//
use aoc2022::load_input;
use std::error::Error;

#[derive(Debug)]
enum Side {
    Left,
    Top,
    Right,
    Bottom,
}

struct Forest {
    trees: Vec<usize>,
    height: usize,
    width: usize,
}
impl Forest {
    fn visible_from_borders(&self) -> usize {
        [Side::Left, Side::Top, Side::Right, Side::Bottom]
            .iter()
            .map(|side| self.visible_from(side))
            .reduce(|acc, cur| Self::vec_or(acc, cur))
            .unwrap()
            .into_iter()
            .map(|vis| vis as usize)
            .sum()
    }
    fn indexes(&self, side: &Side) -> Vec<Vec<usize>> {
        match side {
            Side::Left => (0..self.height)
                .map(|y| {
                    (0..self.width)
                        .map(|x| x + y * self.width)
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>(),
            Side::Top => (0..self.width)
                .map(|x| {
                    (0..self.height)
                        .map(|y| x + y * self.width)
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>(),
            Side::Right => (0..self.height)
                .map(|y| {
                    (0..self.width)
                        .map(|x| x + y * self.width)
                        .rev()
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>(),
            Side::Bottom => (0..self.width)
                .map(|x| {
                    (0..self.height)
                        .map(|y| x + y * self.width)
                        .rev()
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>(),
        }
    }
    fn visible_from(&self, side: &Side) -> Vec<bool> {
        let mut visible = self
            .indexes(side)
            .iter()
            .flat_map(|row| {
                let mut max_height: isize = -1;
                row.iter().map(move |i| {
                    let tree = self.trees[*i] as isize;
                    if tree > max_height {
                        max_height = tree;
                        return (*i, true);
                    }
                    return (*i, false);
                })
            })
            .collect::<Vec<(usize, bool)>>();
        visible.sort_by(|(i, _), (j, _)| i.cmp(j));
        visible.iter().map(|(_, v)| *v).collect()
    }
    fn vec_or(m1: Vec<bool>, m2: Vec<bool>) -> Vec<bool> {
        m1.iter()
            .zip(m2.iter())
            .map(|(c1, c2)| *c1 || *c2)
            .collect::<Vec<bool>>()
    }

    fn scenic_scores(&self) -> Vec<usize> {
        (0..self.height)
            .flat_map(|y| {
                (0..self.width)
                    .map(|x| self.scenic_score(x, y))
                    .collect::<Vec<usize>>()
            })
            .collect()
    }
    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let tree = self.get(x, y);
        let row = self.get_row(y);
        let col = self.get_col(x);

        let s_left = Self::score_vector(row[..x].iter().cloned().rev().collect(), tree);
        let s_top = Self::score_vector(col[..y].iter().cloned().rev().collect(), tree);
        let s_right = Self::score_vector(row[(x + 1)..].iter().cloned().collect(), tree);
        let s_bottom = Self::score_vector(col[(y + 1)..].iter().cloned().collect(), tree);

        s_left * s_top * s_right * s_bottom
    }
    fn score_vector(vector: Vec<usize>, tree: usize) -> usize {
        let mut max_t: isize = -1;
        vector.into_iter().fold(0, |acc, t| {
            if max_t >= tree as isize {
                acc
            } else {
                max_t = if t as isize > max_t {
                    t as isize
                } else {
                    max_t
                };
                acc + 1
            }
        })
    }
    fn get(&self, x: usize, y: usize) -> usize {
        self.trees[x + self.width * y]
    }
    fn get_row(&self, y: usize) -> Vec<usize> {
        (0..self.width)
            .map(|x| self.trees[x + y * self.width])
            .collect()
    }
    fn get_col(&self, x: usize) -> Vec<usize> {
        (0..self.height)
            .map(|y| self.trees[x + y * self.width])
            .collect()
    }
}

fn parse_input(input_text: &str) -> Forest {
    let &height = &input_text.lines().count();
    let &width = &input_text.lines().next().expect("At least one line").len();
    let trees = input_text
        .lines()
        .flat_map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).expect("It's allright") as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Forest {
        trees,
        height,
        width,
    }
}

fn solve_pt1(input_text: &str) -> usize {
    let forest = parse_input(input_text);
    forest.visible_from_borders()
}

fn solve_pt2(input_text: &str) -> usize {
    let forest = parse_input(input_text);
    let scenic_scores = forest.scenic_scores();
    scenic_scores.into_iter().max().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    const FILENAME: &str = "data/day_08_input.txt";
    let input_text = load_input(FILENAME);

    print!("Part one: {:#?}\n", solve_pt1(&input_text));
    // Correct: 1814

    print!("Part two: {:#?}\n", solve_pt2(&input_text));
    // Correct: 330786

    Ok(())
}

// Example tests
#[cfg(test)]
mod example {
    use super::*;

    const TEST_DATA: &str = "30373
25512
65332
33549
35390";
    const ANS_PT1: usize = 21;
    const ANS_PT2: usize = 8;

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(TEST_DATA), ANS_PT1);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2(TEST_DATA), ANS_PT2);
    }
}
