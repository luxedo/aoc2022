// Advent Of Code
// https://adventofcode.com/2022/day/2
//
// --- Day 2: Rock Paper Scissors ---
// The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.
//
// Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.
//
// Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.
//
// The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.
//
// The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
//
// Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.
//
// For example, suppose you were given the following strategy guide:
//
// A Y
// B X
// C Z
// This strategy guide predicts and recommends the following:
//
// In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
// In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
// The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
// In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
//
// What would your total score be if everything goes exactly according to your strategy guide?
// --- Part Two ---
// The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
//
// The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:
//
// In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
// In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
// In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
// Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.
//
// Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?

use aoc2022::load_input;
use std::cmp::Ordering;
use std::error::Error;
use std::slice::Iter;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
#[derive(Debug)]
struct ParseMoveError;
impl FromStr for Move {
    type Err = ParseMoveError;
    fn from_str(str_move: &str) -> Result<Self, Self::Err> {
        const E_ROCK: &str = "A";
        const E_PAPER: &str = "B";
        const E_SCISSORS: &str = "C";
        const P_ROCK: &str = "X";
        const P_PAPER: &str = "Y";
        const P_SCISSORS: &str = "Z";
        match str_move {
            E_ROCK | P_ROCK => Ok(Self::Rock),
            E_PAPER | P_PAPER => Ok(Self::Paper),
            E_SCISSORS | P_SCISSORS => Ok(Self::Scissors),
            _ => Err(ParseMoveError),
        }
    }
}

impl Move {
    fn iter() -> Iter<'static, Move> {
        static MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];
        MOVES.iter()
    }
    fn losing_move(&self) -> Move {
        for mv in Self::iter() {
            if mv.compare(self) == Ordering::Less {
                return *mv;
            }
        }
        panic!("No losing move!");
    }
    fn winning_move(&self) -> Move {
        for mv in Self::iter() {
            if mv.compare(self) == Ordering::Greater {
                return *mv;
            }
        }
        panic!("No winning_move move!");
    }
    fn compare(&self, other: &Self) -> Ordering {
        match other {
            _ if self == other => Ordering::Equal,
            Move::Rock => match self {
                Move::Paper => Ordering::Greater,
                _ => Ordering::Less,
            },
            Move::Paper => match self {
                Move::Scissors => Ordering::Greater,
                _ => Ordering::Less,
            },
            Move::Scissors => match self {
                Move::Rock => Ordering::Greater,
                _ => Ordering::Less,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}
#[derive(Debug)]
struct ParseOutcomeError;
impl FromStr for Outcome {
    type Err = ParseOutcomeError;
    fn from_str(str_outcome: &str) -> Result<Self, Self::Err> {
        const P_LOSE: &str = "X";
        const P_DRAW: &str = "Y";
        const P_WIN: &str = "Z";
        match str_outcome {
            P_LOSE => Ok(Outcome::Lose),
            P_DRAW => Ok(Outcome::Draw),
            P_WIN => Ok(Outcome::Win),
            _ => Err(ParseOutcomeError),
        }
    }
}

struct Game {
    player_move: Move,
    enemy_move: Move,
    outcome: Outcome,
    score: usize,
}

impl Game {
    fn from_str(game_str: &str) -> Self {
        let (enemy_move, player_move) = game_str.split_once(" ").expect("It's allright");
        let player_move = player_move.parse::<Move>().unwrap();
        let enemy_move = enemy_move.parse::<Move>().unwrap();
        let outcome = Self::play(&player_move, &enemy_move);
        let score = Self::score(&player_move, &outcome);
        Self {
            player_move,
            enemy_move,
            outcome,
            score,
        }
    }
    fn from_outcome_str(game_str: &str) -> Self {
        let (enemy_move, outcome) = game_str.split_once(" ").expect("It's allright");
        let enemy_move = enemy_move.parse::<Move>().unwrap();
        let outcome = outcome.parse::<Outcome>().unwrap();
        let player_move = Self::find_move(&enemy_move, &outcome);
        let score = Self::score(&player_move, &outcome);
        Self {
            player_move,
            enemy_move,
            outcome,
            score,
        }
    }
    fn play(player_move: &Move, enemy_move: &Move) -> Outcome {
        match player_move.compare(enemy_move) {
            Ordering::Equal => Outcome::Draw,
            Ordering::Greater => Outcome::Win,
            Ordering::Less => Outcome::Lose,
        }
    }
    fn score(player_move: &Move, outcome: &Outcome) -> usize {
        *outcome as usize + *player_move as usize
    }
    fn find_move(enemy_move: &Move, condition: &Outcome) -> Move {
        match condition {
            Outcome::Draw => *enemy_move,
            Outcome::Lose => enemy_move.losing_move(),
            Outcome::Win => enemy_move.winning_move(),
        }
    }
}

fn solve_pt1(input_text: &str) -> usize {
    input_text
        .trim()
        .split("\n")
        .map(|game_str| Game::from_str(game_str).score)
        .sum::<usize>()
}

fn solve_pt2(input_text: &str) -> usize {
    input_text
        .trim()
        .split("\n")
        .map(|game_str| Game::from_outcome_str(game_str).score)
        .sum::<usize>()
}

fn main() -> Result<(), Box<dyn Error>> {
    const FILENAME: &str = "data/day_02_input.txt";
    let input_text = load_input(FILENAME);

    let score_pt1 = solve_pt1(&input_text);
    print!("Part One: {:#?}\n", score_pt1);
    // Correct: 13221

    let score_pt2 = solve_pt2(&input_text);
    print!("Part Two: {:#?}\n", score_pt2);
    // Correct: 13131

    Ok(())
}

// Example tests
#[cfg(test)]
mod example {
    use super::*;

    const TEST_DATA: &str = "A Y
B X
C Z";
    const ANS_PT1: usize = 15;
    const ANS_PT2: usize = 12;

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(&TEST_DATA), ANS_PT1);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2(&TEST_DATA), ANS_PT2);
    }
}
