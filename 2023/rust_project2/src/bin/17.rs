use std::{
    any::TypeId,
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use num::{complex::ComplexFloat, Complex};

advent_of_code::solution!(17);
type Coord = Complex<i32>;
#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Path_State {
    direction: Direction,
    num_of_consecutive_direction_taken: u32,
    position: Coord,
}
impl Default for Path_State {
    fn default() -> Self {
        return Path_State {
            direction: Direction::Up,
            num_of_consecutive_direction_taken: 0,
            position: Complex::new(0, 0),
        };
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: HashMap<Coord, u32> = parse_input(input);
    Some(naviguate_dijkstra(&map))
}

fn naviguate_dijkstra(map: &HashMap<Coord, u32>) -> u32 {
    let max_x = map.keys().map(|a| a.re).max().unwrap();
    let max_y = map.keys().map(|a| a.im).max().unwrap();

    let mut q: HashMap<Path_State, u32> = HashMap::new();
    q.insert(
        Path_State {
            direction: Direction::Right,
            num_of_consecutive_direction_taken: 0,
            position: Complex::new(0, 0),
        },
        0,
    );

    while !q.is_empty() {
        let (curr_path, curr_dist) = get_closest_path(&q);
        q.remove(&curr_path);
        dbg!(&curr_dist);


        if curr_path.position.re == max_x && curr_path.position.im == max_y {
            return curr_dist;
        }

        for (neighbour_path, neighbour_new_dist) in get_all_next_paths(&curr_path, &curr_dist, map) {
            if !path_is_valid(
                &neighbour_path.position,
                &neighbour_path.num_of_consecutive_direction_taken,
                max_x,
                max_y,
            ) {
                continue;
            }
            if let Some(i) = q.get(&neighbour_path) {
                if neighbour_new_dist < *i {
                    q.insert(neighbour_path, neighbour_new_dist);
                }
            } else {
                q.insert(neighbour_path, neighbour_new_dist);
            }
        }
    }
    unreachable!();
}

fn get_closest_path(q: &HashMap<Path_State, u32>) -> (Path_State, u32) {
    q.iter()
        .fold((Path_State::default(), u32::MAX), |closest, current| {
            if current.1 < &closest.1 {
                return (current.0.clone(), *current.1);
            } else {
                closest
            }
        })
}

fn get_all_next_paths(
    curr_path: &Path_State,
    curr_dist: &u32,
    map: &HashMap<Coord, u32>,
) -> Vec<(Path_State, u32)> {
    let mut all_next_paths: Vec<(Path_State, u32)> = vec![];
    for (adjacent_pos, curr_direction) in get_all_adjacent_coord(&curr_path.position) {
        let new_num_of_consecutive_path = {
            if curr_direction == curr_path.direction {
                curr_path.num_of_consecutive_direction_taken + 1
            } else {
                1
            }
        };
        let heat_value_of_next_pos = {
            if let Some(i) = map.get(&adjacent_pos) {
                i
            } else {
                &0
            }
        };

        all_next_paths.push((
            Path_State {
                direction: curr_direction,
                num_of_consecutive_direction_taken: new_num_of_consecutive_path,
                position: adjacent_pos,
            },
            heat_value_of_next_pos + curr_dist,
        ));
    }
    return all_next_paths.clone();
}

fn get_all_adjacent_coord(curr_pos: &Complex<i32>) -> Vec<(Coord, Direction)> {
    return vec![
        (Complex::<i32>::new(0, -1) + curr_pos, Direction::Up),
        (Complex::<i32>::new(0, 1) + curr_pos, Direction::Down),
        (Complex::<i32>::new(-1, 0) + curr_pos, Direction::Left),
        (Complex::<i32>::new(1, 0) + curr_pos, Direction::Right),
    ];
}

fn path_is_valid(position: &Coord, consecutive_direction: &u32, max_x: i32, max_y: i32) -> bool {
    position.re >= 0
        && position.im >= 0
        && position.re <= max_x
        && position.im <= max_y
        && consecutive_direction <= &3
}

fn parse_input(input: &str) -> HashMap<Coord, u32> {
    let mut map = HashMap::new();
    for (y, line) in input.split("\r\n").enumerate() {
        for (x, char) in line.chars().enumerate() {
            map.insert(
                Complex::new(x as i32, y as i32),
                char.to_string().parse().unwrap(),
            );
        }
    }
    return map.clone();
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
