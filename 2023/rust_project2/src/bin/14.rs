use std::{collections::HashMap, vec};

use itertools::Itertools;

advent_of_code::solution!(14);

type Coord = (usize, usize);
pub fn part_one(input: &str) -> Option<usize> {
    let map: HashMap<Coord, char> = parse_input(input);
    let map_rolled_north = roll_north(&map);
    Some(total_load(&map_rolled_north))
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: HashMap<Coord, char> = parse_input(input);
    Some(total_load(&cycle(&map, 10000000)))
}

fn cycle(map: &HashMap<(usize, usize), char>, num_of_cycle: i32) -> HashMap<(usize, usize), char> {
    let mut all_previous_map_state: Vec<HashMap<Coord, char>> = vec![];
    let mut current_map: HashMap<(usize, usize), char> = map.clone();
    for i in 0..num_of_cycle {
        current_map = full_roll(&current_map);
        //seen map already
        if all_previous_map_state
            .iter()
            .any(|sub_map| sub_map == &current_map)
        {
            let start_of_cycle = all_previous_map_state
                .iter()
                .find_position(|curr_map| *curr_map == &current_map)
                .unwrap()
                .0;
            let length_of_cycle = all_previous_map_state.len() - start_of_cycle;
            let remaining_steps = (num_of_cycle - i) as usize;
            return all_previous_map_state
                [start_of_cycle + (remaining_steps % length_of_cycle) + 2]
                .clone();
        }
        all_previous_map_state.push(current_map.clone());
    }
    return map.clone();
}

fn full_roll(map: &HashMap<(usize, usize), char>) -> HashMap<(usize, usize), char> {
    return roll_east(&roll_south(&roll_west(&roll_north(&map))));
}

fn total_load(map_rolled_north: &HashMap<(usize, usize), char>) -> usize {
    let max_y = map_rolled_north.keys().map(|(_, y)| y).max().unwrap();
    return map_rolled_north
        .iter()
        .filter(|(_, char)| **char == 'O')
        .map(|(coord, _)| max_y - coord.1 + 1)
        .sum::<usize>();
}

fn roll_east(map: &HashMap<(usize, usize), char>) -> HashMap<(usize, usize), char> {
    let max_x = map.keys().map(|(x, _)| x).max().unwrap();
    let max_y = map.keys().map(|(_, y)| y).max().unwrap();
    let mut new_map: HashMap<Coord, char> = map.clone();

    for x in (0..=*max_x).rev() {
        for y in 0..=*max_y {
            let curr_coord = (x, y);
            if new_map.contains_key(&curr_coord) && new_map.get(&curr_coord).unwrap() == &'O' {
                new_map.remove(&curr_coord);
                new_map.insert(get_first_available_pos_to_east(curr_coord, &new_map), 'O');
            }
        }
    }
    return new_map.clone();
}

fn get_first_available_pos_to_east(
    curr_coord: (usize, usize),
    map: &HashMap<(usize, usize), char>,
) -> (usize, usize) {
    let max_x = map.keys().map(|(x, _)| x).max().unwrap();
    for x in curr_coord.0 + 1..=*max_x {
        if map.contains_key(&(x, curr_coord.1)) {
            return ((x as i32 - 1) as usize, curr_coord.1);
        }
    }
    return (*max_x, curr_coord.1);
}

fn roll_south(map: &HashMap<Coord, char>) -> HashMap<Coord, char> {
    let max_x = map.keys().map(|(x, _)| x).max().unwrap();
    let max_y = map.keys().map(|(_, y)| y).max().unwrap();
    let mut new_map: HashMap<Coord, char> = map.clone();

    for y in (0..=*max_y).rev() {
        for x in 0..=*max_x {
            let curr_coord = (x, y);
            if new_map.contains_key(&curr_coord) && new_map.get(&curr_coord).unwrap() == &'O' {
                new_map.remove(&curr_coord);
                new_map.insert(get_first_available_pos_to_south(curr_coord, &new_map), 'O');
            }
        }
    }
    return new_map.clone();
}

fn get_first_available_pos_to_south(
    curr_coord: (usize, usize),
    map: &HashMap<(usize, usize), char>,
) -> (usize, usize) {
    let max_y = map.keys().map(|(_, y)| y).max().unwrap();
    for y in curr_coord.1 + 1..=*max_y {
        if map.contains_key(&(curr_coord.0, y)) {
            return (curr_coord.0, (y as i32 - 1) as usize);
        }
    }
    return (curr_coord.0, *max_y);
}

fn roll_west(map: &HashMap<(usize, usize), char>) -> HashMap<(usize, usize), char> {
    let max_x = map.keys().map(|(x, _)| x).max().unwrap();
    let max_y = map.keys().map(|(_, y)| y).max().unwrap();
    let mut new_map: HashMap<Coord, char> = map.clone();

    for x in 0..=*max_x {
        for y in 0..=*max_y {
            let curr_coord = (x, y);
            if new_map.contains_key(&curr_coord) && new_map.get(&curr_coord).unwrap() == &'O' {
                new_map.remove(&curr_coord);
                new_map.insert(get_first_available_pos_to_west(curr_coord, &new_map), 'O');
            }
        }
    }
    return new_map.clone();
}
fn get_first_available_pos_to_west(
    curr_coord: (usize, usize),
    map: &HashMap<(usize, usize), char>,
) -> (usize, usize) {
    for x in (0..=curr_coord.0).rev() {
        if map.contains_key(&(x, curr_coord.1)) {
            return ((x as i32 + 1) as usize, curr_coord.1);
        }
    }
    return (0, curr_coord.1);
}

fn roll_north(map: &HashMap<(usize, usize), char>) -> HashMap<(usize, usize), char> {
    let max_x = map.keys().map(|(x, _)| x).max().unwrap();
    let max_y = map.keys().map(|(_, y)| y).max().unwrap();
    let mut new_map: HashMap<Coord, char> = map.clone();

    for y in 0..=*max_y {
        for x in 0..=*max_x {
            let curr_coord = (x, y);
            if new_map.contains_key(&curr_coord) && new_map.get(&curr_coord).unwrap() == &'O' {
                new_map.remove(&curr_coord);
                new_map.insert(get_first_available_pos_to_north(curr_coord, &new_map), 'O');
            }
        }
    }
    return new_map.clone();
}

fn get_first_available_pos_to_north(
    curr_coord: (usize, usize),
    map: &HashMap<(usize, usize), char>,
) -> (usize, usize) {
    for y in (0..=curr_coord.1).rev() {
        if map.contains_key(&(curr_coord.0, y)) {
            return (curr_coord.0, y + 1);
        }
    }
    return (curr_coord.0, 0);
}

fn parse_input(input: &str) -> HashMap<(usize, usize), char> {
    let mut map = HashMap::new();
    for (y, line) in input.split("\r\n").enumerate() {
        for (x, _char) in line.chars().enumerate() {
            if _char != '.' {
                map.insert((x, y), _char);
            }
        }
    }
    return map;
}
