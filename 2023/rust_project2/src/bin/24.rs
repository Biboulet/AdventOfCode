use itertools::Itertools;
use num::Complex;
advent_of_code::solution!(24);

type Coord = (f32, f32, f32);

struct HailStone {
    position: Coord,
    velocity: Coord,
}

pub fn part_one(input: &str) -> Option<u32> {
    let hailstones = parse_input(input);
    Some(get_num_of_intersection_within_grid(&hailstones, 200000000000000, 400000000000000))
}

fn get_num_of_intersection_within_grid(
    hailstones: &Vec<HailStone>,
    min_grid: i64,
    max_grid: i64,
) -> u32 {
    let mut intersection = 0;
    for curr_hailstone_couple in hailstones.into_iter().combinations(2) {
        if !vec_are_colineaire(
            &curr_hailstone_couple[0].velocity,
            &curr_hailstone_couple[1].velocity,
        ) {
            let (is_forward_in_time, intersection_point) =
                get_intersection_point(curr_hailstone_couple[0], curr_hailstone_couple[1]);

            if is_forward_in_time && coord_is_in_bound(intersection_point, min_grid, max_grid) {
                intersection += 1;
            }
        }
    }
    return intersection;
}

fn coord_is_in_bound(intersection: Coord, min_grid: i64, max_grid: i64) -> bool {
    return min_grid <= intersection.0 as i64
        && intersection.0 as i64 <= max_grid
        && min_grid <= intersection.1 as i64
        && intersection.1 as i64 <= max_grid;
}

// hailtone 1
// --> y = a1(x - c1) + b1
// hailstone 2:
// --> y = a2(x - c2) + b2
// avec a = vely/velx ; b = posy; c = posx
// D'ou: a1x - a1c1 + b1 = a2x - a2c2 + b2
// x = (a1c1 - b1 - a2c2 +b2) / (a1-a2)
fn get_intersection_point(hailstone_1: &HailStone, hailstone_2: &HailStone) -> (bool, Coord) {
    let a1 = hailstone_1.velocity.1 / hailstone_1.velocity.0;
    let a2 = hailstone_2.velocity.1 / hailstone_2.velocity.0;
    let b1 = hailstone_1.position.1;
    let b2 = hailstone_2.position.1;
    let c1 = hailstone_1.position.0;
    let c2 = hailstone_2.position.0;

    let x_coord_of_the_intersection_point = (a1 * c1 - b1 - a2 * c2 + b2) / (a1 - a2);
    let y_coord_of_the_intersection_point = a1 * (x_coord_of_the_intersection_point - c1) + b1;

    let is_forward_in_time_for_hailstone_a = (hailstone_1.velocity.0 > 0.0
        && x_coord_of_the_intersection_point > hailstone_1.position.0)
        || (hailstone_1.velocity.0 < 0.0
            && x_coord_of_the_intersection_point < hailstone_1.position.0);
    let is_forward_in_time_for_hailstone_b = (hailstone_2.velocity.0 > 0.0
        && x_coord_of_the_intersection_point > hailstone_2.position.0)
        || (hailstone_2.velocity.0 < 0.0
            && x_coord_of_the_intersection_point < hailstone_2.position.0);
    return (
        is_forward_in_time_for_hailstone_a && is_forward_in_time_for_hailstone_b,
        (
            x_coord_of_the_intersection_point,
            y_coord_of_the_intersection_point,
            0.0,
        ),
    );
}

fn vec_are_colineaire(curr_hailstone_couple_1: &Coord, curr_hailstone_couple_2: &Coord) -> bool {
    return curr_hailstone_couple_1.0 / curr_hailstone_couple_2.0
        == curr_hailstone_couple_1.1 / curr_hailstone_couple_2.1;
}

fn parse_input(input: &str) -> Vec<HailStone> {
    return input.split("\r\n").map(|line| parse_line(line)).collect();
}

fn parse_line(line: &str) -> HailStone {
    let args = line.split(" @ ").collect_vec();
    return HailStone {
        position: args[0]
            .split(", ")
            .map(|num| num.trim().parse::<f32>().unwrap())
            .collect_tuple()
            .unwrap(),
        velocity: args[1]
            .split(", ")
            .map(|num| return num.trim().parse::<f32>().unwrap())
            .collect_tuple()
            .unwrap(),
    };
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
