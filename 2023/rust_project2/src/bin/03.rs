use itertools::Itertools;
use std::{collections::HashMap, usize, vec};
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
    let map: HashMap<(usize, usize), char> = parse_input(input);
    let numbers = get_numbers(&map, input);
    Some(numbers
        .iter()
        .filter(|curr_num| curr_num.is_valid)
        .map(|curr_num| curr_num.value)
        .sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: HashMap<(usize, usize), char> = parse_input(input);
    let numbers: Vec<Number> = get_numbers(&map, input);
    let gears: Vec<(usize, usize)> = get_gears(&map);

    Some(gears
        .iter()
        .map(|curr_gear: &(usize, usize)| get_gear_ratio(curr_gear, &numbers))
        .sum())
}

#[derive(Clone, Debug)]
struct Number {
    positions_occupied: Vec<(usize, usize)>,
    value: usize,
    is_valid: bool,
}

fn parse_input(input: &str) -> HashMap<(usize, usize), char> {
    input
        .split("\n")
        .enumerate()
        .fold(&mut HashMap::new(), |map, (index_y, curr_line)| {
            curr_line.chars().enumerate().fold(
                map,
                |map: &mut HashMap<(usize, usize), char>, (index_x, curr_char)| {
                    map.insert((index_x, index_y), curr_char);
                    return map;
                },
            )
        })
        .clone()
}

fn get_gear_ratio(curr_gear: &(usize, usize), numbers: &Vec<Number>) -> usize {
    let adjacent_numbers: Vec<&Number> = numbers
        .iter()
        .filter(|curr_number| {
            curr_number
                .positions_occupied
                .iter()
                .any(|curr_occupied_pos| {
                    get_all_positive_adjacent_coords(curr_occupied_pos)
                        .iter()
                        .any(|curr_adj_coord| curr_adj_coord == curr_gear)
                })
        })
        .collect_vec();

    if adjacent_numbers.len() == 2 {
        return adjacent_numbers[0].value * adjacent_numbers[1].value;
    }
    return 0;
}

fn get_gears(map: &HashMap<(usize, usize), char>) -> Vec<(usize, usize)> {
    map.iter()
        .filter_map(|(key, value)| if value == &'*' { Some(*key) } else { None })
        .collect_vec()
}

fn get_numbers<'a>(map: &'a HashMap<(usize, usize), char>, input: &'a str) -> Vec<Number> {
    let max_ysize = input.split("\n").count();
    let max_xsize = input.split("\n").nth(0).unwrap().len();
    let mut numbers: Vec<Number> = vec![];

    let mut current_num: String = String::new();
    let mut all_positions_num: Vec<(usize, usize)> = Vec::new();

    for y in 0..max_ysize {
        for x in 0..max_xsize {
            let key: (usize, usize) = (x, y);
            let char = map.get(&key).unwrap();

            if char.is_ascii_digit() {
                current_num.push(*char);
                all_positions_num.push(key);
            } else {
                if !current_num.is_empty() {
                    numbers.push(Number {
                        positions_occupied: all_positions_num.clone(),
                        value: current_num.parse::<usize>().unwrap(),
                        is_valid: number_is_valid(&all_positions_num, &map, max_xsize, max_ysize),
                    });
                    current_num.clear();
                    all_positions_num.clear();
                }
            }
        }
    }
    return numbers.clone();
}

fn number_is_valid(
    all_positions_num: &Vec<(usize, usize)>,
    map: &HashMap<(usize, usize), char>,
    max_x: usize,
    max_y: usize,
) -> bool {
    all_positions_num
        .iter()
        .map(|key| get_all_positive_adjacent_coords(key))
        .concat()
        .iter()
        .filter(|(x, y)| x < &max_x && y < &max_y)
        .any(|adj_coord| {
            !map.get(adj_coord).unwrap().is_ascii_digit() && *map.get(adj_coord).unwrap() != '.'
        })
}

fn get_all_positive_adjacent_coords(key: &(usize, usize)) -> Vec<(usize, usize)> {
    return vec![
        ((key.0 as i32) - 1, (key.1 as i32) - 1),
        ((key.0 as i32) - 1, key.1 as i32),
        ((key.0 as i32) - 1, (key.1 as i32) + 1),
        ((key.0 as i32) + 1, (key.1 as i32) - 1),
        ((key.0 as i32) + 1, key.1 as i32),
        ((key.0 as i32) + 1, (key.1 as i32) + 1),
        (key.0 as i32, (key.1 as i32) - 1),
        (key.0 as i32, (key.1 as i32) + 1),
    ]
        .iter()
        .filter(|(a, b)| a >= &0 && b >= &0)
        .map(|(a, b)| (*a as usize, *b as usize))
        .collect_vec();
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
