use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Vec<i32>> {
    return input
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(|arg| arg.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
}

#[aoc(day9, part1)]
fn solve_part1(all_sequence: &Vec<Vec<i32>>) -> i32 {
    return all_sequence
        .iter()
        .map(|curr_sequence| get_next_value(curr_sequence))
        .sum();
}

#[aoc(day9, part2)]
fn solve_part2(all_sequence: &Vec<Vec<i32>>) -> i32 {
    return all_sequence
        .iter()
        .map(|curr_sequence| get_previous_value(curr_sequence))
        .sum();
}

fn get_previous_value(curr_sequence: &Vec<i32>) -> i32 {
    let differenciated_sequence: Vec<Vec<i32>> = get_all_differenciated_sequences(&curr_sequence);
    return differenciated_sequence
        .iter()
        .rev()
        .skip(1)
        .fold(0, |previous_first_value, curr_sequence: &Vec<i32>| {
            curr_sequence.first().unwrap() - previous_first_value
        });
}

fn get_next_value(curr_sequence: &Vec<i32>) -> i32 {
    let differenciated_sequence: Vec<Vec<i32>> = get_all_differenciated_sequences(&curr_sequence);
    return differenciated_sequence
        .iter()
        .rev()
        .skip(1)
        .fold(0, |previous_last_value, curr_sequence: &Vec<i32>| {
            previous_last_value + curr_sequence.last().unwrap()
        });
}

fn get_all_differenciated_sequences(root_sequence: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut all_differenciated_sequences: Vec<Vec<i32>> = vec![root_sequence.clone()];
    while !all_differenciated_sequences
        .last()
        .unwrap()
        .iter()
        .all(|a| *a == 0)
    {
        all_differenciated_sequences.push(
            all_differenciated_sequences
                .last()
                .unwrap()
                .iter()
                .zip(all_differenciated_sequences.last().unwrap().iter().skip(1))
                .map(|(val1, val2)| val2 - val1)
                .collect_vec(),
        );
    }

    return all_differenciated_sequences;
}
