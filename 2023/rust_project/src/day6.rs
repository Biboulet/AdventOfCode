use std::ops::Div;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Race = (usize, usize);
#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<Race> {
    return input
        .split("\n")
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .flat_map(|a| a.parse::<usize>())
        .zip(
            input
                .split("\n")
                .nth(1)
                .unwrap()
                .split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split(" ")
                .flat_map(|a| a.parse::<usize>()),
        )
        .collect_vec();
}

#[aoc(day6, part1)]
fn solve_part1(races: &Vec<Race>) -> usize {
    return races
        .iter()
        .map(|race| get_num_of_way_to_win(race))
        .product();
}

#[aoc(day6, part2)]
fn solve_part2(races: &Vec<Race>) -> usize {
    return 0;
    let full_race: Race = (
        races
            .iter()
            .map(|a| a.0.to_string())
            .join("")
            .parse()
            .unwrap(),
        races
            .iter()
            .map(|a| a.1.to_string())
            .join("")
            .parse()
            .unwrap(),
    );

    return get_num_of_way_to_win(&full_race);
}

fn get_num_of_way_to_win(race: &Race) -> usize {
    let sum = (1..race.0)
        .into_iter()
        .map(|curr_push_duration| is_winning_race(race, curr_push_duration))
        .sum();
    return sum;
}

fn is_winning_race(race: &(usize, usize), curr_push_duration: usize) -> usize {
    (curr_push_duration * (race.0 - curr_push_duration) > race.1) as usize
}
