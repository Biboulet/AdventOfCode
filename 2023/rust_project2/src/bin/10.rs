use itertools::{enumerate, Itertools};
use std::collections::{HashMap, HashSet};
use std::vec;
advent_of_code::solution!(10);

type coord = (i32, i32);

pub fn part_one(input: &str) -> Option<usize> {
    let (map, start) = parse_input(input);
    let adjacent_to_start = get_all_positive_adjacent_coords(&start);
    let first_move = adjacent_to_start
        .iter()
        .find(|coord| map.get(*coord).unwrap().contains(&&start))
        .unwrap();
    let all_positions_of_the_loop = navigate(&start, first_move, &map);
    Some(all_positions_of_the_loop.len() / 2)
}

pub fn part_two(input: &str) -> Option<f32> {
    let (map, start) = parse_input(input);
    let adjacent_to_start = get_all_positive_adjacent_coords(&start);
    let first_move = adjacent_to_start
        .iter()
        .find(|coord| map.get(*coord).unwrap().contains(&&start))
        .unwrap();

    let all_positions_of_the_loop: Vec<(i32, i32)> = navigate(&start, first_move, &map);
    //map with only the char from the main loop
    let char_map = get_filtered_char_map(input, all_positions_of_the_loop);
    //zoom of this map
    let zoomed_char_map = get_zoomed_char_map(char_map);
    let start_of_filling_big_map = zoomed_char_map
        .iter()
        .find(|(_, val)| **val == 'S')
        .unwrap()
        .0;

    Some(get_area(&zoomed_char_map, start_of_filling_big_map) as f32 / 9 as f32)
}

fn get_area(
    zoomed_char_map: &HashMap<(i32, i32), char>,
    start_of_filling_big_map: &(i32, i32),
) -> usize {
    let mut result: usize = 0;
    let mut seen: HashSet<coord> = HashSet::new();
    let mut q: Vec<(i32, i32)> = vec![*start_of_filling_big_map];

    while !q.is_empty() {
        let curr_coord = q.remove(0);
        if seen.contains(&curr_coord) {
            continue;
        }
        seen.insert(curr_coord);
        if zoomed_char_map.get(&curr_coord).unwrap() == &'.' {
            result += 1;
        }

        for adj_coord in get_all_positive_adjacent_coords(&curr_coord) {
            let curr_char = zoomed_char_map.get(&adj_coord).unwrap();
            if curr_char == &'.' || curr_char == &'i' {
                q.push(adj_coord.clone());
            }
        }
    }

    return result;
}

//only the char from the main loop
fn get_filtered_char_map(input: &str, all_pos_from_loop: Vec<coord>) -> HashMap<coord, char> {
    let mut map: HashMap<coord, char> = HashMap::new();
    for (y, line) in input.split("\n").enumerate() {
        for (x, curr_char) in line.chars().enumerate() {
            let curr_coord: coord = (x as i32, y as i32);
            if all_pos_from_loop.contains(&curr_coord) {
                map.insert(curr_coord, curr_char);
            } else {
                map.insert(curr_coord, '.');
            }
        }
    }
    return map.clone();
}

fn get_zoomed_char_map(char_map: HashMap<coord, char>) -> HashMap<coord, char> {
    let mut zoomed_char_map: HashMap<coord, char> = HashMap::new();

    for (coord, char) in char_map {
        for (sub_coord, sub_char) in get_zoomed_char(char) {
            zoomed_char_map.insert(
                (coord.0 * 3 + sub_coord.0, coord.1 * 3 + sub_coord.1),
                sub_char,
            );
        }
    }
    return zoomed_char_map.clone();
}

fn get_zoomed_char(curr_char: char) -> Vec<(coord, char)> {
    match curr_char {
        '|' => {
            return vec![
                ((0, 0), 'i'),
                ((1, 0), '|'),
                ((2, 0), 'i'),
                ((0, 1), 'i'),
                ((1, 1), '|'),
                ((2, 1), 'i'),
                ((0, 2), 'i'),
                ((1, 2), '|'),
                ((2, 2), 'i'),
            ]
        }

        '-' => {
            return vec![
                ((0, 0), 'i'),
                ((1, 0), 'i'),
                ((2, 0), 'i'),
                ((0, 1), '-'),
                ((1, 1), '-'),
                ((2, 1), '-'),
                ((0, 2), 'i'),
                ((1, 2), 'i'),
                ((2, 2), 'i'),
            ]
        }

        'L' => {
            return vec![
                ((0, 0), 'i'),
                ((1, 0), '|'),
                ((2, 0), 'i'),
                ((0, 1), 'i'),
                ((1, 1), 'L'),
                ((2, 1), '-'),
                ((0, 2), 'i'),
                ((1, 2), 'i'),
                ((2, 2), 'i'),
            ]
        }

        'J' => {
            return vec![
                ((0, 0), 'i'),
                ((1, 0), '|'),
                ((2, 0), 'i'),
                ((0, 1), '-'),
                ((1, 1), 'J'),
                ((2, 1), 'i'),
                ((0, 2), 'i'),
                ((1, 2), 'i'),
                ((2, 2), 'i'),
            ]
        }

        '7' => {
            return vec![
                ((0, 0), 'i'),
                ((1, 0), 'i'),
                ((2, 0), 'i'),
                ((0, 1), '-'),
                ((1, 1), '7'),
                ((2, 1), 'i'),
                ((0, 2), 'i'),
                ((1, 2), '|'),
                ((2, 2), 'i'),
            ]
        }

        'F' => {
            return vec![
                ((0, 0), 'i'),
                ((1, 0), 'i'),
                ((2, 0), 'i'),
                ((0, 1), 'i'),
                ((1, 1), 'F'),
                ((2, 1), '-'),
                ((0, 2), 'i'),
                ((1, 2), '|'),
                ((2, 2), 'i'),
            ]
        }

        '.' => {
            return vec![
                ((0, 0), '.'),
                ((1, 0), '.'),
                ((2, 0), '.'),
                ((0, 1), '.'),
                ((1, 1), '.'),
                ((2, 1), '.'),
                ((0, 2), '.'),
                ((1, 2), '.'),
                ((2, 2), '.'),
            ]
        }
        //cas particulier a faire a la main flemme de coder
        'S' => {
            return vec![
                ((0, 0), 'i'),
                ((1, 0), 'i'),
                ((2, 0), 'i'),
                ((0, 1), 'i'),
                ((1, 1), 'F'),
                ((2, 1), '-'),
                ((0, 2), 'i'),
                ((1, 2), '|'),
                ((2, 2), 'S'),
            ]
        }
        _ => panic!(),
    }
}

fn navigate(origin: &coord, first_move: &coord, map: &HashMap<coord, Vec<coord>>) -> Vec<coord> {
    let mut all_positions_of_the_loop: Vec<coord> = vec![*origin];
    let mut previous_coord = *origin;
    let mut current_coord = *first_move;

    while !map.get(&current_coord).unwrap().is_empty() {
        all_positions_of_the_loop.push(current_coord);
        let linked_moves = map.get(&current_coord).unwrap();
        let next_coord = *linked_moves
            .iter()
            .find(|sub_coord| sub_coord != &&previous_coord)
            .unwrap();

        previous_coord = current_coord;
        current_coord = next_coord;
    }
    return all_positions_of_the_loop.clone();
}

fn parse_input(input: &str) -> (HashMap<coord, Vec<coord>>, coord) {
    let mut start = (0, 0);
    let mut map: HashMap<coord, Vec<coord>> = HashMap::new();
    for (y, line) in input.split("\n").enumerate() {
        for (x, curr_char) in line.chars().enumerate() {
            let curr_coord: coord = (x as i32, y as i32);

            if curr_char == 'S' {
                start = curr_coord;
            }
            let linked_coord = get_linked_coord(curr_coord, curr_char)
                .iter()
                .filter(|coords| coords.0 >= 0 && coords.1 >= 0)
                .map(|a| *a)
                .collect_vec();

            map.insert(curr_coord, linked_coord);
        }
    }
    return (map.clone(), start.clone());
}

fn get_linked_coord(curr_coord: coord, curr_char: char) -> Vec<coord> {
    return match curr_char {
        '|' => vec![
            (curr_coord.0, curr_coord.1 + 1),
            (curr_coord.0, curr_coord.1 - 1),
        ],
        '-' => vec![
            (curr_coord.0 - 1, curr_coord.1),
            (curr_coord.0 + 1, curr_coord.1),
        ],
        'L' => vec![
            (curr_coord.0, curr_coord.1 - 1),
            (curr_coord.0 + 1, curr_coord.1),
        ],
        'J' => vec![
            (curr_coord.0, curr_coord.1 - 1),
            (curr_coord.0 - 1, curr_coord.1),
        ],
        '7' => vec![
            (curr_coord.0, curr_coord.1 + 1),
            (curr_coord.0 - 1, curr_coord.1),
        ],
        'F' => vec![
            (curr_coord.0, curr_coord.1 + 1),
            (curr_coord.0 + 1, curr_coord.1),
        ],
        '.' => vec![],
        'S' => vec![],
        _ => panic!(),
    };
}

fn get_all_positive_adjacent_coords(key: &coord) -> Vec<coord> {
    return vec![
        (key.0 - 1, key.1),
        (key.0 + 1, key.1),
        (key.0, key.1 - 1),
        (key.0, key.1 + 1),
    ]
    .iter()
    .filter(|(a, b)| a >= &0 && b >= &0)
    .map(|(a, b)| (*a, *b))
    .collect_vec();
}