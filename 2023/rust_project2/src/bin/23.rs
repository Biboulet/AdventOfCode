use std::{
    collections::{HashMap, HashSet},
    vec,
};

use num::Complex;

advent_of_code::solution!(23);

type Coord = Complex<i32>;
#[derive(Debug, Clone)]
struct Path {
    length: u32,
    position: Coord,
    visited_position: HashSet<Coord>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: HashMap<Coord, char> = parse_input(input);

    Some(get_longest_valid_path(map, false))
}
pub fn part_two(input: &str) -> Option<u32> {
    let map: HashMap<Coord, char> = parse_input(input);
    Some(get_longest_valid_path(map, true))
}

fn get_longest_valid_path(map: HashMap<Coord, char>, is_p2: bool) -> u32 {
    let mut valid_paths: Vec<Path> = Vec::new();
    let mut q = vec![Path {
        length: 1,
        position: Complex::new(1, 1),
        visited_position: HashSet::from_iter([Complex::new(1, 0)]),
    }];
    while let Some(current_path) = q.pop() {
        
        if is_arrival(&map, current_path.position) {
            valid_paths.push(current_path.clone());
            continue;
        }
        let mut new_visited_position = current_path.visited_position.clone();
        new_visited_position.insert(current_path.position);
        for adjacent_valid_pos in get_valid_adjacent_pos(
            &map,
            current_path.position,
            current_path.visited_position,
            is_p2,
        ) {
            q.push(Path {
                length: current_path.length + 1,
                position: adjacent_valid_pos,
                visited_position: new_visited_position.clone(),
            })
        }
    }
    return valid_paths.iter().map(|path| path.length).max().unwrap();
}

fn get_valid_adjacent_pos(
    map: &HashMap<Coord, char>,
    position: Coord,
    visited_position: HashSet<Coord>,
    is_p2: bool,
) -> Vec<Coord> {
    let adjacent_coord: Vec<Coord> = get_adjacent_position(position);
    return if is_p2 {
        adjacent_coord
    } else {
        match map.get(&position).unwrap() {
            '^' => vec![adjacent_coord[0]],
            'v' => vec![adjacent_coord[1]],
            '<' => vec![adjacent_coord[2]],
            '>' => vec![adjacent_coord[3]],
            '.' => adjacent_coord,
            a => panic!("unexpected char: {}", a),
        }
    }
    .iter()
    .filter(|adj_coord| {
        *map.get(*adj_coord).unwrap() != '#' && !visited_position.contains(adj_coord)
    }).copied()
    .collect();
}
// Up ; Down ; Left ; Right
fn get_adjacent_position(curr_coord: Coord) -> Vec<Coord> {
    vec![
        curr_coord + Complex::<i32>::new(0, -1),
        curr_coord + Complex::<i32>::new(0, 1),
        curr_coord + Complex::<i32>::new(-1, 0),
        curr_coord + Complex::<i32>::new(1, 0),
    ]
}

fn is_arrival(map: &HashMap<Coord, char>, position: Coord) -> bool {
    return position.re == map.iter().map(|(pos, _)| pos.re).max().unwrap() - 1
        && position.im == map.iter().map(|(pos, _)| pos.im).max().unwrap();
}

fn parse_input(input: &str) -> HashMap<Complex<i32>, char> {
    let mut map = HashMap::new();
    for (y, line) in input.split("\r\n").enumerate() {
        for (x, char) in line.chars().enumerate() {
            map.insert(Complex::new(x as i32, y as i32), char);
        }
    }
    map.clone()
}
