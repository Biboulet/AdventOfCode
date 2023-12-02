use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;



#[aoc(day1, part1)]
fn solve_part1(input: &str) -> usize {
    return input
        .split("\n")
        .map(|line| {
            let first_digit: usize = line
                .chars()
                .find_map(|a| a.to_string().parse().ok())
                .unwrap();
            let second_digit: usize = line
                .chars()
                .rev()
                .find_map(|a| a.to_string().parse().ok())
                .unwrap();
            return 10 * first_digit + second_digit;
        })
        .sum();
}

#[aoc(day1, part2)]
fn solve_part2(input: &str) -> usize {
    let real_input = input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");
    return solve_part1(&real_input);
}
