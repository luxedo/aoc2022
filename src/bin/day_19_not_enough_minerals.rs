// Advent Of Code
// https://adventofcode.com/2022/day/4
//
// --- Day 19: Not Enough Minerals ---
// Your scans show that the lava did indeed form obsidian!
//
// The wind has changed direction enough to stop sending lava droplets toward you, so you and the elephants exit the cave. As you do, you notice a collection of geodes around the pond. Perhaps you could use the obsidian to create some geode-cracking robots and break them open?
//
// To collect the obsidian from the bottom of the pond, you'll need waterproof obsidian-collecting robots. Fortunately, there is an abundant amount of clay nearby that you can use to make them waterproof.
//
// In order to harvest the clay, you'll need special-purpose clay-collecting robots. To make any type of robot, you'll need ore, which is also plentiful but in the opposite direction from the clay.
//
// Collecting ore requires ore-collecting robots with big drills. Fortunately, you have exactly one ore-collecting robot in your pack that you can use to kickstart the whole operation.
//
// Each robot can collect 1 of its resource type per minute. It also takes one minute for the robot factory (also conveniently from your pack) to construct any type of robot, although it consumes the necessary resources available when construction begins.
//
// The robot factory has many blueprints (your puzzle input) you can choose from, but once you've configured it with a blueprint, you can't change it. You'll need to work out which blueprint is best.
//
// For example:
//
// Blueprint 1:
//   Each ore robot costs 4 ore.
//   Each clay robot costs 2 ore.
//   Each obsidian robot costs 3 ore and 14 clay.
//   Each geode robot costs 2 ore and 7 obsidian.
//
// Blueprint 2:
//   Each ore robot costs 2 ore.
//   Each clay robot costs 3 ore.
//   Each obsidian robot costs 3 ore and 8 clay.
//   Each geode robot costs 3 ore and 12 obsidian.
// (Blueprints have been line-wrapped here for legibility. The robot factory's actual assortment of blueprints are provided one blueprint per line.)
//
// The elephants are starting to look hungry, so you shouldn't take too long; you need to figure out which blueprint would maximize the number of opened geodes after 24 minutes by figuring out which robots to build and when to build them.
//
// Using blueprint 1 in the example above, the largest number of geodes you could open in 24 minutes is 9. One way to achieve that is:
//
// == Minute 1 ==
// 1 ore-collecting robot collects 1 ore; you now have 1 ore.
//
// == Minute 2 ==
// 1 ore-collecting robot collects 1 ore; you now have 2 ore.
//
// == Minute 3 ==
// Spend 2 ore to start building a clay-collecting robot.
// 1 ore-collecting robot collects 1 ore; you now have 1 ore.
// The new clay-collecting robot is ready; you now have 1 of them.
//
// == Minute 4 ==
// 1 ore-collecting robot collects 1 ore; you now have 2 ore.
// 1 clay-collecting robot collects 1 clay; you now have 1 clay.
//
// == Minute 5 ==
// Spend 2 ore to start building a clay-collecting robot.
// 1 ore-collecting robot collects 1 ore; you now have 1 ore.
// 1 clay-collecting robot collects 1 clay; you now have 2 clay.
// The new clay-collecting robot is ready; you now have 2 of them.
//
// == Minute 6 ==
// 1 ore-collecting robot collects 1 ore; you now have 2 ore.
// 2 clay-collecting robots collect 2 clay; you now have 4 clay.
//
// == Minute 7 ==
// Spend 2 ore to start building a clay-collecting robot.
// 1 ore-collecting robot collects 1 ore; you now have 1 ore.
// 2 clay-collecting robots collect 2 clay; you now have 6 clay.
// The new clay-collecting robot is ready; you now have 3 of them.
//
// == Minute 8 ==
// 1 ore-collecting robot collects 1 ore; you now have 2 ore.
// 3 clay-collecting robots collect 3 clay; you now have 9 clay.
//
// == Minute 9 ==
// 1 ore-collecting robot collects 1 ore; you now have 3 ore.
// 3 clay-collecting robots collect 3 clay; you now have 12 clay.
//
// == Minute 10 ==
// 1 ore-collecting robot collects 1 ore; you now have 4 ore.
// 3 clay-collecting robots collect 3 clay; you now have 15 clay.
//
// == Minute 11 ==
// Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
// 1 ore-collecting robot collects 1 ore; you now have 2 ore.
// 3 clay-collecting robots collect 3 clay; you now have 4 clay.
// The new obsidian-collecting robot is ready; you now have 1 of them.
//
// == Minute 12 ==
// Spend 2 ore to start building a clay-collecting robot.
// 1 ore-collecting robot collects 1 ore; you now have 1 ore.
// 3 clay-collecting robots collect 3 clay; you now have 7 clay.
// 1 obsidian-collecting robot collects 1 obsidian; you now have 1 obsidian.
// The new clay-collecting robot is ready; you now have 4 of them.
//
// == Minute 13 ==
// 1 ore-collecting robot collects 1 ore; you now have 2 ore.
// 4 clay-collecting robots collect 4 clay; you now have 11 clay.
// 1 obsidian-collecting robot collects 1 obsidian; you now have 2 obsidian.
//
// == Minute 14 ==
// 1 ore-collecting robot collects 1 ore; you now have 3 ore.
// 4 clay-collecting robots collect 4 clay; you now have 15 clay.
// 1 obsidian-collecting robot collects 1 obsidian; you now have 3 obsidian.
//
// == Minute 15 ==
// Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
// 1 ore-collecting robot collects 1 ore; you now have 1 ore.
// 4 clay-collecting robots collect 4 clay; you now have 5 clay.
// 1 obsidian-collecting robot collects 1 obsidian; you now have 4 obsidian.
// The new obsidian-collecting robot is ready; you now have 2 of them.
//
// == Minute 16 ==
// 1 ore-collecting robot collects 1 ore; you now have 2 ore.
// 4 clay-collecting robots collect 4 clay; you now have 9 clay.
// 2 obsidian-collecting robots collect 2 obsidian; you now have 6 obsidian.
//
// == Minute 17 ==
// 1 ore-collecting robot collects 1 ore; you now have 3 ore.
// 4 clay-collecting robots collect 4 clay; you now have 13 clay.
// 2 obsidian-collecting robots collect 2 obsidian; you now have 8 obsidian.
//
// == Minute 18 ==
// Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
// 1 ore-collecting robot collects 1 ore; you now have 2 ore.
// 4 clay-collecting robots collect 4 clay; you now have 17 clay.
// 2 obsidian-collecting robots collect 2 obsidian; you now have 3 obsidian.
// The new geode-cracking robot is ready; you now have 1 of them.
//
// == Minute 19 ==
// 1 ore-collecting robot collects 1 ore; you now have 3 ore.
// 4 clay-collecting robots collect 4 clay; you now have 21 clay.
// 2 obsidian-collecting robots collect 2 obsidian; you now have 5 obsidian.
// 1 geode-cracking robot cracks 1 geode; you now have 1 open geode.
//
// == Minute 20 ==
// 1 ore-collecting robot collects 1 ore; you now have 4 ore.
// 4 clay-collecting robots collect 4 clay; you now have 25 clay.
// 2 obsidian-collecting robots collect 2 obsidian; you now have 7 obsidian.
// 1 geode-cracking robot cracks 1 geode; you now have 2 open geodes.
//
// == Minute 21 ==
// Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
// 1 ore-collecting robot collects 1 ore; you now have 3 ore.
// 4 clay-collecting robots collect 4 clay; you now have 29 clay.
// 2 obsidian-collecting robots collect 2 obsidian; you now have 2 obsidian.
// 1 geode-cracking robot cracks 1 geode; you now have 3 open geodes.
// The new geode-cracking robot is ready; you now have 2 of them.
//
// == Minute 22 ==
// 1 ore-collecting robot collects 1 ore; you now have 4 ore.
// 4 clay-collecting robots collect 4 clay; you now have 33 clay.
// 2 obsidian-collecting robots collect 2 obsidian; you now have 4 obsidian.
// 2 geode-cracking robots crack 2 geodes; you now have 5 open geodes.
//
// == Minute 23 ==
// 1 ore-collecting robot collects 1 ore; you now have 5 ore.
// 4 clay-collecting robots collect 4 clay; you now have 37 clay.
// 2 obsidian-collecting robots collect 2 obsidian; you now have 6 obsidian.
// 2 geode-cracking robots crack 2 geodes; you now have 7 open geodes.
//
// == Minute 24 ==
// 1 ore-collecting robot collects 1 ore; you now have 6 ore.
// 4 clay-collecting robots collect 4 clay; you now have 41 clay.
// 2 obsidian-collecting robots collect 2 obsidian; you now have 8 obsidian.
// 2 geode-cracking robots crack 2 geodes; you now have 9 open geodes.
// However, by using blueprint 2 in the example above, you could do even better: the largest number of geodes you could open in 24 minutes is 12.
//
// Determine the quality level of each blueprint by multiplying that blueprint's ID number with the largest number of geodes that can be opened in 24 minutes using that blueprint. In this example, the first blueprint has ID 1 and can open 9 geodes, so its quality level is 9. The second blueprint has ID 2 and can open 12 geodes, so its quality level is 24. Finally, if you add up the quality levels of all of the blueprints in the list, you get 33.
//
// Determine the quality level of each blueprint using the largest number of geodes it could produce in 24 minutes. What do you get if you add up the quality level of all of the blueprints in your list?
//
//
//--- Part Two ---
// While you were choosing the best blueprint, the elephants found some food on their own, so you're not in as much of a hurry; you figure you probably have 32 minutes before the wind changes direction again and you'll need to get out of range of the erupting volcano.
//
// Unfortunately, one of the elephants ate most of your blueprint list! Now, only the first three blueprints in your list are intact.
//
// In 32 minutes, the largest number of geodes blueprint 1 (from the example above) can open is 56. One way to achieve that is:
//
// == Minute 1 ==
// 1 ore-collecting robot collects 1 ore; you now have 1 ore.
//
// == Minute 2 ==
// 1 ore-collecting robot collects 1 ore; you now have 2 ore.
//
// == Minute 3 ==
// 1 ore-collecting robot collects 1 ore; you now have 3 ore.
//
// == Minute 4 ==
// 1 ore-collecting robot collects 1 ore; you now have 4 ore.
//
// == Minute 5 ==
// Spend 4 ore to start building an ore-collecting robot.
// 1 ore-collecting robot collects 1 ore; you now have 1 ore.
// The new ore-collecting robot is ready; you now have 2 of them.
//
// == Minute 6 ==
// 2 ore-collecting robots collect 2 ore; you now have 3 ore.
//
// == Minute 7 ==
// Spend 2 ore to start building a clay-collecting robot.
// 2 ore-collecting robots collect 2 ore; you now have 3 ore.
// The new clay-collecting robot is ready; you now have 1 of them.
//
// == Minute 8 ==
// Spend 2 ore to start building a clay-collecting robot.
// 2 ore-collecting robots collect 2 ore; you now have 3 ore.
// 1 clay-collecting robot collects 1 clay; you now have 1 clay.
// The new clay-collecting robot is ready; you now have 2 of them.
//
// == Minute 9 ==
// Spend 2 ore to start building a clay-collecting robot.
// 2 ore-collecting robots collect 2 ore; you now have 3 ore.
// 2 clay-collecting robots collect 2 clay; you now have 3 clay.
// The new clay-collecting robot is ready; you now have 3 of them.
//
// == Minute 10 ==
// Spend 2 ore to start building a clay-collecting robot.
// 2 ore-collecting robots collect 2 ore; you now have 3 ore.
// 3 clay-collecting robots collect 3 clay; you now have 6 clay.
// The new clay-collecting robot is ready; you now have 4 of them.
//
// == Minute 11 ==
// Spend 2 ore to start building a clay-collecting robot.
// 2 ore-collecting robots collect 2 ore; you now have 3 ore.
// 4 clay-collecting robots collect 4 clay; you now have 10 clay.
// The new clay-collecting robot is ready; you now have 5 of them.
//
// == Minute 12 ==
// Spend 2 ore to start building a clay-collecting robot.
// 2 ore-collecting robots collect 2 ore; you now have 3 ore.
// 5 clay-collecting robots collect 5 clay; you now have 15 clay.
// The new clay-collecting robot is ready; you now have 6 of them.
//
// == Minute 13 ==
// Spend 2 ore to start building a clay-collecting robot.
// 2 ore-collecting robots collect 2 ore; you now have 3 ore.
// 6 clay-collecting robots collect 6 clay; you now have 21 clay.
// The new clay-collecting robot is ready; you now have 7 of them.
//
// == Minute 14 ==
// Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
// 2 ore-collecting robots collect 2 ore; you now have 2 ore.
// 7 clay-collecting robots collect 7 clay; you now have 14 clay.
// The new obsidian-collecting robot is ready; you now have 1 of them.
//
// == Minute 15 ==
// 2 ore-collecting robots collect 2 ore; you now have 4 ore.
// 7 clay-collecting robots collect 7 clay; you now have 21 clay.
// 1 obsidian-collecting robot collects 1 obsidian; you now have 1 obsidian.
//
// == Minute 16 ==
// Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
// 2 ore-collecting robots collect 2 ore; you now have 3 ore.
// 7 clay-collecting robots collect 7 clay; you now have 14 clay.
// 1 obsidian-collecting robot collects 1 obsidian; you now have 2 obsidian.
// The new obsidian-collecting robot is ready; you now have 2 of them.
//
// == Minute 17 ==
// Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
// 2 ore-collecting robots collect 2 ore; you now have 2 ore.
// 7 clay-collecting robots collect 7 clay; you now have 7 clay.
// 2 obsidian-collecting robots collect 2 obsidian; you now have 4 obsidian.
// The new obsidian-collecting robot is ready; you now have 3 of them.
//
// == Minute 18 ==
// 2 ore-collecting robots collect 2 ore; you now have 4 ore.
// 7 clay-collecting robots collect 7 clay; you now have 14 clay.
// 3 obsidian-collecting robots collect 3 obsidian; you now have 7 obsidian.
//
// == Minute 19 ==
// Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
// 2 ore-collecting robots collect 2 ore; you now have 3 ore.
// 7 clay-collecting robots collect 7 clay; you now have 7 clay.
// 3 obsidian-collecting robots collect 3 obsidian; you now have 10 obsidian.
// The new obsidian-collecting robot is ready; you now have 4 of them.
//
// == Minute 20 ==
// Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
// 2 ore-collecting robots collect 2 ore; you now have 3 ore.
// 7 clay-collecting robots collect 7 clay; you now have 14 clay.
// 4 obsidian-collecting robots collect 4 obsidian; you now have 7 obsidian.
// The new geode-cracking robot is ready; you now have 1 of them.
//
// == Minute 21 ==
// Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
// 2 ore-collecting robots collect 2 ore; you now have 2 ore.
// 7 clay-collecting robots collect 7 clay; you now have 7 clay.
// 4 obsidian-collecting robots collect 4 obsidian; you now have 11 obsidian.
// 1 geode-cracking robot cracks 1 geode; you now have 1 open geode.
// The new obsidian-collecting robot is ready; you now have 5 of them.
//
// == Minute 22 ==
// Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
// 2 ore-collecting robots collect 2 ore; you now have 2 ore.
// 7 clay-collecting robots collect 7 clay; you now have 14 clay.
// 5 obsidian-collecting robots collect 5 obsidian; you now have 9 obsidian.
// 1 geode-cracking robot cracks 1 geode; you now have 2 open geodes.
// The new geode-cracking robot is ready; you now have 2 of them.
//
// == Minute 23 ==
// Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
// 2 ore-collecting robots collect 2 ore; you now have 2 ore.
// 7 clay-collecting robots collect 7 clay; you now have 21 clay.
// 5 obsidian-collecting robots collect 5 obsidian; you now have 7 obsidian.
// 2 geode-cracking robots crack 2 geodes; you now have 4 open geodes.
// The new geode-cracking robot is ready; you now have 3 of them.
//
// == Minute 24 ==
// Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
// 2 ore-collecting robots collect 2 ore; you now have 2 ore.
// 7 clay-collecting robots collect 7 clay; you now have 28 clay.
// 5 obsidian-collecting robots collect 5 obsidian; you now have 5 obsidian.
// 3 geode-cracking robots crack 3 geodes; you now have 7 open geodes.
// The new geode-cracking robot is ready; you now have 4 of them.
//
// == Minute 25 ==
// 2 ore-collecting robots collect 2 ore; you now have 4 ore.
// 7 clay-collecting robots collect 7 clay; you now have 35 clay.
// 5 obsidian-collecting robots collect 5 obsidian; you now have 10 obsidian.
// 4 geode-cracking robots crack 4 geodes; you now have 11 open geodes.
//
// == Minute 26 ==
// Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
// 2 ore-collecting robots collect 2 ore; you now have 4 ore.
// 7 clay-collecting robots collect 7 clay; you now have 42 clay.
// 5 obsidian-collecting robots collect 5 obsidian; you now have 8 obsidian.
// 4 geode-cracking robots crack 4 geodes; you now have 15 open geodes.
// The new geode-cracking robot is ready; you now have 5 of them.
//
// == Minute 27 ==
// Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
// 2 ore-collecting robots collect 2 ore; you now have 4 ore.
// 7 clay-collecting robots collect 7 clay; you now have 49 clay.
// 5 obsidian-collecting robots collect 5 obsidian; you now have 6 obsidian.
// 5 geode-cracking robots crack 5 geodes; you now have 20 open geodes.
// The new geode-cracking robot is ready; you now have 6 of them.
//
// == Minute 28 ==
// 2 ore-collecting robots collect 2 ore; you now have 6 ore.
// 7 clay-collecting robots collect 7 clay; you now have 56 clay.
// 5 obsidian-collecting robots collect 5 obsidian; you now have 11 obsidian.
// 6 geode-cracking robots crack 6 geodes; you now have 26 open geodes.
//
// == Minute 29 ==
// Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
// 2 ore-collecting robots collect 2 ore; you now have 6 ore.
// 7 clay-collecting robots collect 7 clay; you now have 63 clay.
// 5 obsidian-collecting robots collect 5 obsidian; you now have 9 obsidian.
// 6 geode-cracking robots crack 6 geodes; you now have 32 open geodes.
// The new geode-cracking robot is ready; you now have 7 of them.
//
// == Minute 30 ==
// Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
// 2 ore-collecting robots collect 2 ore; you now have 6 ore.
// 7 clay-collecting robots collect 7 clay; you now have 70 clay.
// 5 obsidian-collecting robots collect 5 obsidian; you now have 7 obsidian.
// 7 geode-cracking robots crack 7 geodes; you now have 39 open geodes.
// The new geode-cracking robot is ready; you now have 8 of them.
//
// == Minute 31 ==
// Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
// 2 ore-collecting robots collect 2 ore; you now have 6 ore.
// 7 clay-collecting robots collect 7 clay; you now have 77 clay.
// 5 obsidian-collecting robots collect 5 obsidian; you now have 5 obsidian.
// 8 geode-cracking robots crack 8 geodes; you now have 47 open geodes.
// The new geode-cracking robot is ready; you now have 9 of them.
//
// == Minute 32 ==
// 2 ore-collecting robots collect 2 ore; you now have 8 ore.
// 7 clay-collecting robots collect 7 clay; you now have 84 clay.
// 5 obsidian-collecting robots collect 5 obsidian; you now have 10 obsidian.
// 9 geode-cracking robots crack 9 geodes; you now have 56 open geodes.
// However, blueprint 2 from the example above is still better; using it, the largest number of geodes you could open in 32 minutes is 62.
//
// You no longer have enough blueprints to worry about quality levels. Instead, for each of the first three blueprints, determine the largest number of geodes you could open; then, multiply these three values together.
//
// Don't worry about quality levels; instead, just determine the largest number of geodes you could open using each of the first three blueprints. What do you get if you multiply these numbers together?

use aoc2022::load_input;
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};
use std::error::Error;
use std::fmt;

const ORE: &str = "ore";
const CLAY: &str = "clay";
const OBSIDIAN: &str = "obsidian";
const GEODE: &str = "geode";

#[derive(Debug, Clone, PartialEq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
impl Resources {
    fn has_resources(&self, ore: usize, clay: usize, obsidian: usize, geode: usize) -> bool {
        self.ore >= ore && self.clay >= clay && self.obsidian >= obsidian && self.geode >= geode
    }
    fn remove(&mut self, ore: usize, clay: usize, obsidian: usize, geode: usize) {
        self.ore -= ore;
        self.clay -= clay;
        self.obsidian -= obsidian;
        self.geode -= geode;
    }
}

#[derive(Debug, Clone)]
struct Robot {
    mines: Resource,
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: u8,
    robots: Vec<Robot>,
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct Resources {
    geode: usize,
    obsidian: usize,
    clay: usize,
    ore: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
struct MineState {
    pile: Resources,
    mining_robots: Resources,
    time: usize,
}
impl Ord for MineState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pile
            .geode
            .cmp(&other.pile.geode)
            .then(self.mining_robots.cmp(&other.mining_robots))
            .then(self.time.cmp(&other.time))
            .then(self.pile.obsidian.cmp(&other.pile.obsidian))
            .then(self.pile.clay.cmp(&other.pile.clay))
            .then(self.pile.ore.cmp(&other.pile.ore))
    }
}

fn parse_input(input_text: &str) -> Vec<Blueprint> {
    input_text
        .lines()
        .map(|line| {
            let (blue_id, blueprint) = line.split_once(":").unwrap();
            let blue_id = blue_id.split_once(" ").unwrap().1.parse::<u8>().unwrap();
            let robots = blueprint
                .trim()
                .split(". ")
                .map(|b| {
                    let sentence = b
                        .trim_end_matches(".")
                        .trim()
                        .split(" ")
                        .collect::<Vec<&str>>();
                    let mines = match sentence.get(1) {
                        Some(&ORE) => Resource::Ore,
                        Some(&CLAY) => Resource::Clay,
                        Some(&OBSIDIAN) => Resource::Obsidian,
                        Some(&GEODE) => Resource::Geode,
                        other => panic!("Resource not found"),
                    };
                    let mut costs = vec![];
                    let mut i = 5;
                    let mut robot = Robot {
                        mines,
                        ore: 0,
                        clay: 0,
                        obsidian: 0,
                    };
                    while let (Some(resource_name), Some(ammount)) =
                        (sentence.get(i), sentence.get(i - 1))
                    {
                        let ammount = ammount.parse::<usize>().unwrap();
                        costs.push(match resource_name {
                            &ORE => robot.ore = ammount,
                            &CLAY => robot.clay = ammount,
                            &OBSIDIAN => robot.obsidian = ammount,
                            other => panic!("Resource {other} not found"),
                        });
                        i += 3;
                    }
                    robot
                })
                .collect::<Vec<Robot>>();
            Blueprint {
                id: blue_id,
                robots,
            }
        })
        .collect::<Vec<Blueprint>>()
}

fn find_best_blueprint(blueprints: &Vec<Blueprint>, minutes: usize) -> Vec<(u8, usize)> {
    let starting_robots = Resources {
        ore: 1,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    let resource_piles = blueprints
        .into_iter()
        .map(|blueprint| optimize_mining(blueprint, minutes, &starting_robots))
        .collect::<Vec<Resources>>();
    // dbg!(&resource_piles);
    let results = blueprints
        .iter()
        .zip(resource_piles.iter())
        .map(|(blueprint, mined)| (blueprint.id, mined.geode))
        .collect::<Vec<(u8, usize)>>();
    dbg!(&results);
    results
}
fn optimize_mining(
    blueprint: &Blueprint,
    minutes: usize,
    starting_robots: &Resources,
) -> Resources {
    Resources {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
    }
}

fn optimize_mining_2(
    blueprint: &Blueprint,
    minutes: usize,
    starting_robots: &Resources,
) -> Resources {
    let initial_state = MineState {
        pile: Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        mining_robots: starting_robots.clone(),
        time: 0,
    };

    let max_ore = blueprint
        .robots
        .iter()
        .map(|robot| robot.ore)
        .max()
        .unwrap();
    let max_clay = blueprint
        .robots
        .iter()
        .map(|robot| robot.clay)
        .max()
        .unwrap();
    let max_obsidian = blueprint
        .robots
        .iter()
        .map(|robot| robot.obsidian)
        .max()
        .unwrap();

    let mut checked: BTreeSet<MineState> = BTreeSet::new();
    let mut frontier: BinaryHeap<MineState> = BinaryHeap::new();
    frontier.push(initial_state);
    while let Some(state) = frontier.pop() {
        if checked.contains(&state) {
            continue;
        }
        checked.insert(state.clone());

        if state.time == minutes {
            // Stop adding states after minutes
            continue;
        }

        // 1. Check possible paths/ Check avaliable resources
        let mrobots = &state.mining_robots;
        let possible_robots = blueprint
            .robots
            .iter()
            .filter_map(|robot| {
                // Heuristics here!
                // * you should always build the robot you are going for as early as you can (ie if your ore
                // costs, ignoring secondary ressources, are (1, 2, 3, 4) and you have 2 ore, then you should
                // not idle for 5 turns and then build a clay robot. idling for 2 turns to build a geode robot
                // is fine, though)
                // * you should never build more robots than your highest cost of that type of resource is (so
                // for the the sample blueprint 1, never build more than 4 ore, 14 clay and 7 obsidian robots)
                if state
                    .pile
                    .has_resources(robot.ore, robot.clay, robot.obsidian, 0)
                {
                    if (state.pile.ore - robot.ore <= robot.ore // Build as fast as possible
                        || state.pile.clay - robot.clay <= robot.clay
                        || state.pile.obsidian - robot.obsidian <= robot.obsidian)
                        && ((mrobots.ore <= max_ore)  // Don't build more robots than needed
                        && (mrobots.clay <= max_clay)
                        && (mrobots.obsidian <= max_obsidian))
                    {
                        return Some(robot);
                    }
                }
                None
            })
            .collect::<Vec<&Robot>>();

        // 2. Mine resources
        let mut new_state = state.clone();
        new_state.pile.ore += mrobots.ore;
        new_state.pile.clay += mrobots.clay;
        new_state.pile.obsidian += mrobots.obsidian;
        new_state.pile.geode += mrobots.geode;
        new_state.time += 1;

        // 3. Add new paths to the queue
        // 3.a. Case no new robots
        if (state.pile.ore <= max_ore)
            || (state.pile.clay <= max_clay)
            || (state.pile.obsidian <= max_obsidian)
        {
            frontier.push(new_state.clone());
        }

        // 3.a. Case new robots
        possible_robots.iter().for_each(|robot| {
            let mut new_state = new_state.clone();
            new_state
                .pile
                .remove(robot.ore, robot.clay, robot.obsidian, 0);
            match robot.mines {
                Resource::Ore => new_state.mining_robots.ore += 1,
                Resource::Clay => new_state.mining_robots.clay += 1,
                Resource::Obsidian => new_state.mining_robots.obsidian += 1,
                Resource::Geode => new_state.mining_robots.geode += 1,
            }
            // Find if a state of the same minute has more resources
            if !checked.contains(&new_state) {
                frontier.push(new_state);
            }
        })
    }
    // dbg!(&checked);
    let best_plan = checked.iter().max().unwrap();
    dbg!(&best_plan);
    best_plan.pile
}

fn solve_pt1(input_text: &str) -> u64 {
    let blueprints = parse_input(input_text);
    const MINUTES: usize = 24;
    let best_yield = find_best_blueprint(&blueprints, MINUTES);
    best_yield
        .iter()
        .fold(0, |acc, (id, geodes)| acc + (*id as u64 * *geodes as u64))
}

fn solve_pt2(input_text: &str) -> u64 {
    let blueprints = parse_input(input_text);
    let blueprints = &blueprints[0..2].to_vec();
    const MINUTES: usize = 32;
    let best_yield = find_best_blueprint(&blueprints, MINUTES);
    best_yield
        .iter()
        .fold(0, |acc, (id, geodes)| acc + (*id as u64 * *geodes as u64))
}

fn main() -> Result<(), Box<dyn Error>> {
    const FILENAME: &str = "data/day_19_input.txt";
    let input_text = load_input(FILENAME);

    print!("Part one: {:#?}\n", solve_pt1(&input_text));
    // Correct: 1703

    print!("Part two: {:#?}\n", solve_pt2(&input_text));
    // Correct: BBB

    Ok(())
}

// Example tests
#[cfg(test)]
mod example {
    use super::*;

    const TEST_DATA: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    // const TEST_DATA: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
    // const TEST_DATA: &str = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    const ANS_PT1: u64 = 33;
    const ANS_PT2: u64 = 1;

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(TEST_DATA), ANS_PT1);
    }

    // #[test]
    // fn test_pt2() {
    //     assert_eq!(solve_pt2(TEST_DATA), ANS_PT2);
    // }
}
