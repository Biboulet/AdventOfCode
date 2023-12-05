use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use std::usize;

#[derive(Clone, Debug)]
struct Convertor {
    ranges: Vec<Range>,
}
#[derive(Clone, Debug)]
struct Range {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}
impl Convertor {
    pub fn new(package: &str) -> Self {
        return Convertor {
            ranges: package
                .split("\n")
                .skip(1)
                .map(|line| Range::new(line))
                .collect_vec(),
        };
    }
    pub fn reversed(&self) -> Self {
        return Convertor {
            ranges: self
                .ranges
                .iter()
                .map(|curr_range| curr_range.reversed())
                .collect_vec(),
        };
    }
    fn convert_number(&self, input_num: &usize) -> usize {
        if let Some(valid_convertor) = self
            .ranges
            .iter()
            .find(|curr_range| curr_range.is_in_range(input_num))
        {
            return valid_convertor.convert_number(input_num);
        } else {
            return *input_num;
        }
    }
}

impl Range {
    pub fn new(line: &str) -> Self {
        let args = line
            .trim()
            .split(" ")
            .map(|value| value.parse().unwrap())
            .collect_vec();
        return Range {
            destination_range_start: args[0],
            source_range_start: args[1],
            range_length: args[2],
        };
    }
    pub fn reversed(&self) -> Self {
        return Range {
            destination_range_start: self.source_range_start,
            source_range_start: self.destination_range_start,
            range_length: self.range_length,
        };
    }
    fn is_in_range(&self, input_num: &usize) -> bool {
        self.source_range_start <= *input_num
            && *input_num < self.source_range_start + self.range_length
    }
    fn convert_number(&self, input_num: &usize) -> usize {
        return self.destination_range_start + (*input_num - self.source_range_start);
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> (Vec<usize>, Vec<Convertor>) {
    let seeds: Vec<usize> = input
        .split("\n")
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .map(|num| num.parse::<usize>().unwrap())
        .collect_vec();

    let convertors: Vec<Convertor> = input
        .split("\n\n")
        .into_iter()
        .skip(1)
        .map(|package| Convertor::new(package))
        .collect_vec();

    return (seeds.clone(), convertors.clone());
}

#[aoc(day5, part1)]
fn solve_part1(input: &(Vec<usize>, Vec<Convertor>)) -> usize {
    let (seeds, convertors) = input;
    return seeds
        .iter()
        .map(|curr_seed| {
            convertors
                .iter()
                .fold(*curr_seed, |curr_value, curr_convertor| {
                    return curr_convertor.convert_number(&curr_value);
                })
        })
        .min()
        .expect("failed founding minimum");
}

#[aoc(day5, part2)]
fn solve_part2(input: &(Vec<usize>, Vec<Convertor>)) -> usize {
    let reversed_convertor = input
        .1
        .iter()
        .map(|convertor| convertor.reversed())
        .rev()
        .collect_vec();

    let max_location = reversed_convertor
        .first()
        .unwrap()
        .ranges
        .iter()
        .map(|curr_range| curr_range.source_range_start + curr_range.range_length)
        .max()
        .unwrap();

    for location in 0..max_location {
        let corresponding_seed = reversed_convertor
            .iter()
            .fold(location, |curr_value, curr_convertor| {
                curr_convertor.convert_number(&curr_value)
            });

        if is_seed_valid(input.0.clone(), corresponding_seed) {
            return location;
        }
    }
    unreachable!();
}

fn is_seed_valid(seeds: Vec<usize>, seed_to_test: usize) -> bool {
    return (0..seeds.len() / 2)
        .into_iter()
        .any(|index| seeds[index * 2] <= seed_to_test && seed_to_test < seeds[index * 2] + seeds[2 * index + 1]);
}
