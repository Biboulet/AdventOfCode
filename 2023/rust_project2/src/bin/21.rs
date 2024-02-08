use std::{
    collections::{HashMap, HashSet},
    vec,
};

use itertools::Itertools;
use num::{Complex, Integer};

advent_of_code::solution!(21);
type Coord = Complex<i32>;

pub fn part_one(input: &str) -> Option<u32> {
    let (starting_pos, rock_map) = parse_input(input);
    Some(simulate_walk(starting_pos, rock_map, input, 0, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (starting_pos, rock_map) = parse_input(input);
    Some(simulate_walk(starting_pos, rock_map, input, 5000, true))
}

fn simulate_walk(
    starting_pos: Complex<i32>,
    rock_map: HashSet<Complex<i32>>,
    input: &str,
    num_of_steps: u32,
    is_part2: bool,
) -> u32 {
    let X_max = input.split("\r\n").next().unwrap().len() as i32;
    let Y_max = input.split("\r\n").collect_vec().len() as i32;
    let mut final_pos: HashSet<Coord> = HashSet::new();
    //position, step remaining
    let mut seen: HashMap<Coord, i32> = HashMap::new();
    let mut q: Vec<(Coord, u32)> = vec![];
    q.push((starting_pos, num_of_steps));
    while !q.is_empty() {
        let curr_situation = q.pop().unwrap();
        //truc a faire peut etre avec le seen
        if curr_situation.1 % 2 == 0 {
            final_pos.insert(curr_situation.0);
        }
        if curr_situation.1 == 0 {
            continue;
        }
        for next_situation in get_adjacent_position(curr_situation.0)
            .iter()
            .filter(|new_pos| {
                (is_part2 || !is_out_of_bounds(*new_pos, X_max, Y_max))
                    && !rock_map.contains(&Complex::new(
                        new_pos.re.rem_euclid(X_max),
                        new_pos.im.rem_euclid(Y_max),
                    ))
            })
            .map(|new_valid_pos| (*new_valid_pos, curr_situation.1 - 1))
            .collect_vec()
        {
            if next_situation.1 as i32 > *seen.get(&next_situation.0).unwrap_or(&-1) {
                seen.insert(next_situation.0, next_situation.1 as i32);
                q.push(next_situation);
            }
        }
    }
    return final_pos.len() as u32;
}

fn is_out_of_bounds(new_pos: &Complex<i32>, x_max: i32, y_max: i32) -> bool {
    return new_pos.re < 0 || new_pos.im < 0 || new_pos.re >= x_max || new_pos.im >= y_max;
}

fn get_adjacent_position(curr_coord: Coord) -> Vec<Coord> {
    return vec![
        curr_coord + Complex::<i32>::new(0, -1),
        curr_coord + Complex::<i32>::new(0, 1),
        curr_coord + Complex::<i32>::new(-1, 0),
        curr_coord + Complex::<i32>::new(1, 0),
    ];
}

fn parse_input(input: &str) -> (Coord, HashSet<Coord>) {
    let mut starting_pos = Complex::new(0, 0);
    let mut rock_map: HashSet<Coord> = HashSet::new();
    for (y, line) in input.split("\r\n").enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                starting_pos = Complex::new(x as i32, y as i32);
            }
            if char == '#' {
                rock_map.insert(Complex::new(x as i32, y as i32));
            }
        }
    }
    return (starting_pos.clone(), rock_map.clone());
}
