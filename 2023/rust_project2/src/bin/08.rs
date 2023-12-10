advent_of_code::solution!(8);
use std::collections::HashMap;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let (navigation_instr, map) = parse_input(input);
    let mut current_location = "AAA";
    let mut index = 0;
    loop {
        if current_location == "ZZZ" {
            break Some(index);
        }
        let curr_instr = navigation_instr
            .chars()
            .nth(index % navigation_instr.len())
            .unwrap();
        current_location = match curr_instr == 'L' {
            true => map[current_location].0,
            false => map[current_location].1,
        };
        index += 1;
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let (navigation_instr, map) = parse_input(input);
    let mut all_current_locations = map
        .keys()
        .filter(|key| key.chars().last().unwrap() == 'A')
        .map(|key| *key)
        .collect_vec();

    let mut arrivals: Vec<usize> = vec![0; all_current_locations.len()];
    let mut num_of_steps = 0;
    while arrivals.iter().any(|arrival| *arrival == 0) {
        all_current_locations
            .iter()
            .enumerate()
            .for_each(|(index, val)| {
                if val.chars().last().unwrap() == 'Z' {
                    if arrivals[index] == 0 {
                        arrivals[index] = num_of_steps;
                    }
                }
            });
        let curr_instruction = navigation_instr
            .chars()
            .nth(num_of_steps % navigation_instr.len())
            .unwrap();

        all_current_locations = all_current_locations
            .iter()
            .map(|curr_loc| match curr_instruction == 'L' {
                true => map[curr_loc].0,
                false => map[curr_loc].1,
            })
            .collect_vec();
        num_of_steps += 1;
    }
    Some(lcm(&arrivals))
}

fn parse_input(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let navigation_instr = input.split("\n").next().unwrap();
    let map = input.split("\n").skip(2).fold(
        HashMap::new(),
        |mut map: HashMap<&str, (&str, &str)>, curr_line| {
            let node_parent = curr_line.split("=").next().unwrap().trim();
            let node_childs = curr_line
                .split("=")
                .nth(1)
                .unwrap()
                .split("(")
                .nth(1)
                .unwrap()
                .split(")")
                .next()
                .unwrap()
                .split(", ")
                .collect_tuple()
                .unwrap();
            map.insert(node_parent, node_childs);
            return map;
        },
    );
    return (navigation_instr, map);
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
