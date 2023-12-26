use std::{
    collections::{HashMap, HashSet},
    vec,
};

use itertools::Itertools;
use num::Complex;
advent_of_code::solution!(16);
type Coord = (Complex<i32>);
#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: HashMap<Coord, char> = parse_input(input);
    Some(spread_beam(&map, &(Complex::new(0, 0), Direction::Right)))
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: HashMap<Coord, char> = parse_input(input);
    let starts: Vec<(Coord, Direction)> = get_all_starts(&map);
    Some(
        starts
            .iter()
            .map(|curr_start| spread_beam(&map, curr_start))
            .max()
            .unwrap(),
    )
}

fn get_all_starts(map: &HashMap<Complex<i32>, char>) -> Vec<(Complex<i32>, Direction)> {
    let max_x = map.keys().map(|a| a.re).max().unwrap();
    let max_y = map.keys().map(|a| a.im).max().unwrap();

    let mut all_starts: Vec<(Complex<i32>, Direction)> = vec![];
    //top
    all_starts.extend((0..max_x).map(|x| (Complex::new(x,0), Direction::Down)));
    //left
    all_starts.extend((0..max_y).map(|y| (Complex::new(0,y), Direction::Right)));
    //right
    all_starts.extend((0..max_y).map(|y| (Complex::new(max_x, y), Direction::Left)));
    //bottom
    all_starts.extend((0..max_x).map(|x| (Complex::new(x,max_y), Direction::Up)));

    return all_starts.clone(); 
}

fn spread_beam(map: &HashMap<Coord, char>, start: &(Complex<i32>, Direction)) -> usize {
    let max_x = map.keys().map(|a| a.re).max().unwrap();
    let max_y = map.keys().map(|a| a.im).max().unwrap();

    let mut energized_pos: HashSet<Coord> = HashSet::new();
    let mut seen: HashSet<(Coord, Direction)> = HashSet::new();
    let mut q: Vec<(Coord, Direction)> = vec![*start];
    while !q.is_empty() {
        let curr_beam = q.pop().unwrap();
        if pos_is_invalid(&curr_beam.0, max_x, max_y) || seen.contains(&curr_beam) {
            continue;
        }
        seen.insert(curr_beam);
        energized_pos.insert(curr_beam.0);
        let curr_char = map.get(&curr_beam.0).unwrap();
        q.extend(get_next_beams(curr_char, &curr_beam));
    }
    return energized_pos.len();
}

fn get_next_beams(curr_char: &char, curr_beam: &(Coord, Direction)) -> Vec<(Coord, Direction)> {
    let next_directions: Vec<Direction> = get_next_direction(curr_char, &curr_beam.1);
    return next_directions
        .into_iter()
        .map(|curr_next_dir| {
            (
                curr_beam.0 + get_next_position(curr_next_dir),
                curr_next_dir,
            )
        })
        .collect_vec();
}

fn get_next_direction(curr_char: &char, direction: &Direction) -> Vec<Direction> {
    return match curr_char {
        '.' => vec![*direction],
        '/' => vec![match direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
        }],
        '\\' => vec![match direction {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
        }],
        '|' => {
            if *direction == Direction::Up || *direction == Direction::Down {
                vec![*direction]
            } else {
                vec![Direction::Down, Direction::Up]
            }
        }
        '-' => {
            if *direction == Direction::Left || *direction == Direction::Right {
                vec![*direction]
            } else {
                vec![Direction::Right, Direction::Left]
            }
        }

        a => panic!("found an unexpected char : {}", a),
    };
}

fn get_next_position(direction: Direction) -> Coord {
    return match direction {
        Direction::Up => Complex::<i32>::new(0, -1),
        Direction::Down => Complex::<i32>::new(0, 1),
        Direction::Left => Complex::<i32>::new(-1, 0),
        Direction::Right => Complex::<i32>::new(1, 0),
    };
}

fn pos_is_invalid(position: &Coord, max_x: i32, max_y: i32) -> bool {
    position.re < 0 || position.im < 0 || position.re > max_x || position.im > max_y
}

fn parse_input(input: &str) -> HashMap<Coord, char> {
    assert!(input.contains("\r\n"));
    let mut map = HashMap::new();
    for (y, line) in input.split("\r\n").enumerate() {
        for (x, curr_char) in line.chars().enumerate() {
            let a = Complex::new(x as i32, y as i32);
            map.insert(a, curr_char);
        }
    }
    return map.clone();
}
