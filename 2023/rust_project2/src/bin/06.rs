advent_of_code::solution!(6);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let races = parse_input(input);
    Some(races
        .iter()

        .map(|race| get_num_of_way_to_win(race))
        .product())
}

pub fn part_two(input: &str) -> Option<usize> {
    let races = parse_input(input);
    let full_race: Race = (
        races
            .iter()
            .map(|a| a.0.to_string())
            .join("")
            .parse()
            .unwrap(),
        races
            .iter()
            .map(|a| a.1.to_string())
            .join("")
            .parse()
            .unwrap(),
    );

    Some(get_num_of_way_to_win(&full_race))
}


type Race = (usize, usize);

fn parse_input(input: &str) -> Vec<Race> {
    return input
        .split("\n")
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .flat_map(|a| a.parse::<usize>())
        .zip(
            input
                .split("\n")
                .nth(1)
                .unwrap()
                .split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split(" ")
                .flat_map(|a| a.parse::<usize>()),
        )
        .collect_vec();
}

fn get_num_of_way_to_win(race: &Race) -> usize {
    let sum = (1..race.0)
        .into_iter()
        .map(|curr_push_duration| is_winning_race(race, curr_push_duration))
        .sum();
    return sum;
}

fn is_winning_race(race: &(usize, usize), curr_push_duration: usize) -> usize {
    (curr_push_duration * (race.0 - curr_push_duration) > race.1) as usize
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
