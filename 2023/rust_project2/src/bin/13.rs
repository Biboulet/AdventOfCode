use std::{
    collections::HashMap,
    vec,
};

use itertools::Itertools;

advent_of_code::solution!(13);
type Coord = (usize, usize);

pub fn part_one(input: &str) -> Option<i32> {
    let patterns = parse_input(input);
    Some(
        patterns
            .iter()
            .map(|pattern| get_pattern_value(pattern).unwrap())
            .concat()
            .iter()
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let patterns = parse_input(input);
    Some(
        patterns
            .iter()
            .map(|pattern| get_modified_valid_pattern_value(pattern))
            .sum(),
    )
}

fn get_modified_valid_pattern_value(pattern: &HashMap<(usize, usize), char>) -> i32 {
    let mut all_modified_pattern = vec![pattern.clone(); pattern.keys().len()];
    for (i, key) in pattern.keys().enumerate() {
        let opposite_char: char = match pattern.get(key).unwrap() {
            '#' => '.',
            '.' => '#',
            _ => panic!(),
        };
        all_modified_pattern[i].insert(*key, opposite_char);
    }
    let default_pattern_value: i32 = get_pattern_value(pattern).unwrap().iter().sum();

    return *all_modified_pattern
        .iter()
        .flat_map(|curr_modified_pattern| get_pattern_value(curr_modified_pattern))
        .concat()
        .iter()
        .find(|val| **val != default_pattern_value).unwrap();
}

fn get_pattern_value(pattern: &HashMap<(usize, usize), char>) -> Result<Vec<i32>, &str> {
    // line[horizontal_mirror] = line[horizontal_mirror+1]
    let horizontal_mirror = get_horizontal_mirror(pattern);
    let vertical_mirror = get_vertical_mirror(pattern);

    let mut all_values = vec![];
    if horizontal_mirror.is_empty() && vertical_mirror.is_empty() {
        return Err("no symetrical axis in the pattern");
    }
    for curr_horizontal_axis in horizontal_mirror {
        all_values.push((curr_horizontal_axis + 1) * 100);
    }
    for curr_vertical_axis in vertical_mirror {
        all_values.push(curr_vertical_axis + 1)
    }
    return Ok(all_values.clone());
}

fn get_horizontal_mirror(pattern: &HashMap<(usize, usize), char>) -> Vec<i32> {
    let mut all_horizontal_axis = vec![];
    let y_max = pattern.keys().map(|(_, y)| y).max().unwrap();
    let x_max = pattern.keys().map(|(x, _)| x).max().unwrap();

    for y in 0..*y_max {
        let lines_before: Vec<Vec<char>> = (0..=y)
            .into_iter()
            .map(|sub_y| {
                (0..=*x_max)
                    .into_iter()
                    .map(|sub_x| pattern[&(sub_x, sub_y)])
                    .collect_vec()
            })
            .rev()
            .collect_vec();
        let lines_after: Vec<Vec<char>> = (y + 1..=*y_max)
            .into_iter()
            .map(|sub_y| {
                (0..=*x_max)
                    .into_iter()
                    .map(|sub_x| pattern[&(sub_x, sub_y)])
                    .collect_vec()
            })
            .collect_vec();
        let is_symetrical = lines_before
            .iter()
            .zip(lines_after)
            .all(|(line_a, line_b)| {
                line_a
                    .iter()
                    .zip(line_b)
                    .all(|(char_a, char_b)| *char_a == char_b)
            });
        if is_symetrical {
            all_horizontal_axis.push(y as i32);
        }
    }
    return all_horizontal_axis.clone();
}
fn get_vertical_mirror(pattern: &HashMap<(usize, usize), char>) -> Vec<i32> {
    let mut all_vertical_axis = vec![];
    let y_max = pattern.keys().map(|(_, y)| y).max().unwrap();
    let x_max = pattern.keys().map(|(x, _)| x).max().unwrap();

    for x in 0..*x_max {
        let row_before: Vec<Vec<char>> = (0..=x)
            .into_iter()
            .map(|sub_x| {
                (0..=*y_max)
                    .into_iter()
                    .map(|sub_y| pattern[&(sub_x, sub_y)])
                    .collect_vec()
            })
            .rev()
            .collect_vec();

        let row_after: Vec<Vec<char>> = (x + 1..=*x_max)
            .into_iter()
            .map(|sub_x| {
                (0..=*y_max)
                    .into_iter()
                    .map(|sub_y| pattern[&(sub_x, sub_y)])
                    .collect_vec()
            })
            .collect_vec();
        let is_symetrical = row_before.iter().zip(row_after).all(|(row_a, row_b)| {
            row_a
                .iter()
                .zip(row_b)
                .all(|(char_a, char_b)| *char_a == char_b)
        });
        if is_symetrical {
            all_vertical_axis.push(x as i32);
        }
    }
    return all_vertical_axis.clone();
}

fn parse_input(input: &str) -> Vec<HashMap<Coord, char>> {
    return input
        .split("\n\n")
        .map(|pattern| parse_pattern(pattern))
        .collect_vec();
}

fn parse_pattern(pattern: &str) -> HashMap<(usize, usize), char> {
    let mut map = HashMap::new();
    for (y, line) in pattern.split("\n").enumerate() {
        for (x, char) in line.chars().enumerate() {
            map.insert((x, y), char);
        }
    }
    return map.clone();
}
