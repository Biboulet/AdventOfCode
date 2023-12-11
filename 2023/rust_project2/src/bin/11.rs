use std::{
    cmp::{max, min},
    collections::HashMap,
    usize, vec,
};

use itertools::Itertools;

advent_of_code::solution!(11);

type Coord = (usize, usize);

pub fn part_one(input: &str) -> Option<usize> {
    let map: HashMap<Coord, char> = parse_input(input);
    let (empty_rows, empty_lines) = get_empty_rows_and_lines(&map);
    let stars: Vec<Coord> = map
        .iter()
        .filter(|(_, char)| char == &&'#')
        .map(|(coord, _)| *coord)
        .collect_vec();

    Some(
        stars
            .iter()
            .combinations(2)
            .map(|stars| {
                get_man_dist(stars[0], stars[1])
                    + num_of_empty_rows_and_lines_crossed(
                        &empty_rows,
                        &empty_lines,
                        stars[0],
                        stars[1],
                    )
            })
            .sum(),
    )
}
pub fn part_two(input: &str) -> Option<usize> {
    let map: HashMap<Coord, char> = parse_input(input);
    let (empty_rows, empty_lines) = get_empty_rows_and_lines(&map);
    let stars: Vec<Coord> = map
        .iter()
        .filter(|(_, char)| char == &&'#')
        .map(|(coord, _)| *coord)
        .collect_vec();

    Some(
        stars
            .iter()
            .combinations(2)
            .map(|stars| {
                get_man_dist(stars[0], stars[1])
                    + num_of_empty_rows_and_lines_crossed(
                        &empty_rows,
                        &empty_lines,
                        stars[0],
                        stars[1],
                    ) * (1000000 - 1)
            })
            .sum(),
    )
}

fn num_of_empty_rows_and_lines_crossed(
    empty_rows: &Vec<usize>,
    empty_lines: &Vec<usize>,
    stars_1: &(usize, usize),
    stars_2: &(usize, usize),
) -> usize {
    let x_min = min(stars_1.0, stars_2.0);
    let y_min = min(stars_1.1, stars_2.1);
    let x_max = max(stars_1.0, stars_2.0);
    let y_max = max(stars_1.1, stars_2.1);
    return empty_lines
        .iter()
        .map(|curr_empty_line| (y_min < *curr_empty_line && *curr_empty_line < y_max) as usize)
        .sum::<usize>()
        + empty_rows
            .iter()
            .map(|curr_empty_row| (x_min < *curr_empty_row && *curr_empty_row < x_max) as usize)
            .sum::<usize>();
}

fn get_empty_rows_and_lines(map: &HashMap<(usize, usize), char>) -> (Vec<usize>, Vec<usize>) {
    let x_max = map.keys().map(|(x, _)| x).max().unwrap();
    let y_max = map.keys().map(|(_, y)| y).max().unwrap();

    let mut empty_rows = vec![];
    let mut empty_lines = vec![];
    //rows
    for x in 0..=*x_max {
        if (0..=*y_max)
            .into_iter()
            .map(|y| map[&(x, y)])
            .all(|char| char == '.')
        {
            empty_rows.push(x);
        }
    }
    for y in 0..=*y_max {
        if (0..=*x_max)
            .into_iter()
            .map(|x| map[&(x, y)])
            .all(|char| char == '.')
        {
            empty_lines.push(y);
        }
    }

    return (empty_rows, empty_lines);
}

fn get_man_dist(star1: &Coord, star2: &Coord) -> usize {
    return star1.0.abs_diff(star2.0) + star1.1.abs_diff(star2.1);
}

fn parse_input(input: &str) -> HashMap<(usize, usize), char> {
    let mut map: HashMap<Coord, char> = HashMap::new();
    for (y, line) in input.split("\n").enumerate() {
        for (x, char) in line.chars().enumerate() {
            map.insert((x, y), char);
        }
    }
    return map.clone();
}
