use std::cmp::max;
use itertools::Itertools;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let parsed_input: Vec<Vec<Vec<(usize, &str)>>> = parse_input(input);
    Some(parsed_input
        .iter()
        .enumerate()
        .map(|(index, curr_line)| {
            if line_is_valid(curr_line) {
                return index + 1;
            } else {
                return 0;
            }
        })
        .sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let parsed_input: Vec<Vec<Vec<(usize, &str)>>> = parse_input(input);
    Some(parsed_input.iter().map(|game| get_game_power(game)).sum())
}

fn parse_input(input: &str) -> Vec<Vec<Vec<(usize, &str)>>> {
    return input.split("\n").map(|line| parse_line(line)).collect_vec();
}

fn parse_line(line: &str) -> Vec<Vec<(usize, &str)>> {
    return line
        .split(": ")
        .nth(1)
        .unwrap()
        .split(';')
        .map(|a| parse_packet(a))
        .collect();
}

fn parse_packet(packet: &str) -> Vec<(usize, &str)> {
    return packet
        .split(",")
        .map(|num_and_color| parse_grab(num_and_color))
        .collect_vec();
}

fn parse_grab(num_and_color: &str) -> (usize, &str) {
    let args: Vec<&str> = num_and_color.trim().split(' ').collect();
    return (args[0].parse::<usize>().expect("failed parsing"), args[1]);
}

fn get_game_power(game: &Vec<Vec<(usize, &str)>>) -> usize {
    let (min_red, min_green, min_blue) = get_min_num_of_cube(game);
    return min_red * min_green * min_blue;
}

fn get_min_num_of_cube(game: &Vec<Vec<(usize, &str)>>) -> (usize, usize, usize) {
    // (red, gree, blue)
    return game
        .iter()
        .fold((0, 0, 0), |fewest_num_of_cubes, current_packet| {
            let curr_num_of_cubes = get_num_of_cube(current_packet);
            return (
                max(curr_num_of_cubes.0, fewest_num_of_cubes.0),
                max(curr_num_of_cubes.1, fewest_num_of_cubes.1),
                max(curr_num_of_cubes.2, fewest_num_of_cubes.2),
            );
        });
}

fn get_num_of_cube(current_packet: &Vec<(usize, &str)>) -> (usize, usize, usize) {
    return current_packet
        .iter()
        .fold((0, 0, 0), |curr_num_of_cubes, curr_grab| match curr_grab.1 {
            "red" => return (curr_grab.0, curr_num_of_cubes.1, curr_num_of_cubes.2),
            "green" => return (curr_num_of_cubes.0, curr_grab.0, curr_num_of_cubes.2),
            "blue" => return (curr_num_of_cubes.0, curr_num_of_cubes.1, curr_grab.0),
            _ => panic!("couleur bizzare"),
        });
}

fn line_is_valid(curr_line: &Vec<Vec<(usize, &str)>>) -> bool {
    return curr_line.iter().all(|packet| packet_is_valid(packet));
}

fn packet_is_valid(packet: &[(usize, &str)]) -> bool {
    return packet.iter().all(|grab| grab_is_valid(*grab));
}

fn grab_is_valid(grab: (usize, &str)) -> bool {
    match grab.1 {
        "red" => return grab.0 <= 12,
        "green" => return grab.0 <= 13,
        "blue" => return grab.0 <= 14,
        _ => panic!("couleur bizzare"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
