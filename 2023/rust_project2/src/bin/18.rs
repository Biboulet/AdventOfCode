use itertools::Itertools;
use num::Complex;
use std::vec;

advent_of_code::solution!(18);
type Coord = Complex<i32>;

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
struct Instruction<'a> {
    direction: Direction,
    length: u32,
    color: &'a str,
}

impl Instruction<'_> {
    pub fn as_part2(&self) -> Self {
        let true_length =
            u32::from_str_radix(self.color.chars().skip(2).take(5).join("").as_str(), 16).unwrap();
        let true_direction = match self.color.trim().chars().rev().nth(1).unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            a => panic!("{}", a),
        };
        return Instruction {
            color: self.color,
            length: true_length,
            direction: true_direction,
        };
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
fn get_direction_from_str(char: &str) -> Direction {
    return match char {
        "U" => Direction::Up,
        "L" => Direction::Left,
        "R" => Direction::Right,
        "D" => Direction::Down,
        _ => panic!("unexpected_char"),
    };
}

pub fn part_one(input: &str) -> Option<usize> {
    let instructions = parse_input(input);
    let corners: Vec<Coord> = get_corners(&instructions);
    Some(get_area(corners))
}

pub fn part_two(input: &str) -> Option<usize> {
    let instructions = parse_input(input)
        .iter()
        .map(|instr| instr.as_part2())
        .collect_vec();
    let corners: Vec<Coord> = get_corners(&instructions);
    Some(get_area(corners))
}

fn get_area(corners: Vec<Complex<i32>>) -> usize {
    //shoelace + pick's theorem
    let mut area: i64 = 0;
    let mut perimiter: u64 = 0;
    for i in 0..corners.len() {
        let corner_a = corners[i];
        let corner_b = corners[(i + 1) % corners.len()];
        area += corner_a.im as i64 * corner_b.re as i64 - corner_a.re as i64 * corner_b.im as i64;
        perimiter += (corner_a.re.abs_diff(corner_b.re) + corner_a.im.abs_diff(corner_b.im)) as u64;
    }
    return area.abs() as usize / 2 + perimiter as usize / 2 + 1;
}

fn get_corners(instructions: &[Instruction<'_>]) -> Vec<Complex<i32>> {
    let mut corners = vec![];
    let mut curr_pos: Coord = Complex::new(0, 0);
    for curr_instruction in instructions {
        corners.push(curr_pos.clone());

        curr_pos += get_next_position(curr_instruction.direction) * curr_instruction.length as i32;
    }
    return corners.clone();
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .split("\n")
        .map(|line| {
            let mut args = line.split(" ");
            return Instruction {
                direction: get_direction_from_str(args.next().unwrap()),
                length: args.next().unwrap().parse().unwrap(),
                color: args.next().unwrap(),
            };
        })
        .collect_vec()
}

fn get_next_position(direction: Direction) -> Coord {
    return match direction {
        Direction::Up => Complex::<i32>::new(0, 1),
        Direction::Down => Complex::<i32>::new(0, -1),
        Direction::Left => Complex::<i32>::new(-1, 0),
        Direction::Right => Complex::<i32>::new(1, 0),
    };
}
