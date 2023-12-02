use std::usize;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Grab<'a> = (usize, &'a str);

fn parse_input(input: &str) -> Vec<Vec<Vec<(usize, &str)>>> {
    return input.split("\n").map(|line| parse_line(line)).collect_vec();
}

fn parse_line(line: &str) -> Vec<Vec<(usize, &str)>> {
    return line
        .split(": ")
        .nth(1)
        .unwrap()
        .split(';')
        .map(|a| parse_packet(a))
        .collect();
}

fn parse_packet(packet: &str) -> Vec<(usize, &str)> {
    return packet
        .split(",")
        .map(|num_and_color| parse_grab(num_and_color))
        .collect_vec();
}

fn parse_grab(num_and_color: &str) -> Grab {
    let args: Vec<&str> = num_and_color.trim().split(' ').collect();
    return (args[0].parse::<usize>().expect("failed parsing"), args[1]);
}
#[aoc(day2, part1)]
fn solve_part1(input: &str) -> usize {
    let parsed_input: Vec<Vec<Vec<(usize, &str)>>> = parse_input(input);
    return parsed_input
        .iter()
        .enumerate()
        .map(|(index, curr_line)| {
            if line_is_valid(curr_line) {
                return index + 1;
            } else {
                return 0;
            }
        })
        .sum();
}

fn line_is_valid(curr_line: &Vec<Vec<(usize, &str)>>) -> bool {
    return curr_line.iter().all(|packet| packet_is_valid(packet));
}

fn packet_is_valid(packet: &[(usize, &str)]) -> bool {
    return packet.iter().all(|grab| grab_is_valid(*grab));
}

fn grab_is_valid(grab: (usize, &str)) -> bool {
    match grab.1 {
        "red" => return grab.0 <= 12,
        "green" => return grab.0 <= 13,
        "blue" => return grab.0 <= 14,
        _ => panic!("couleur bizzare")
    }
}

// #[aoc(day2, part2)]
// fn solve_part2() -> usize {
//     return 0;
// }
