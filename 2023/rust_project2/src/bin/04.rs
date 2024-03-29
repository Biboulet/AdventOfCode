use itertools::Itertools;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let parsed_input: Vec<(Vec<usize>, Vec<usize>)> = parse_input(input);
    Some(parsed_input
        .iter()
        .map(|game: &(Vec<usize>, Vec<usize>)| get_game_score(game))
        .sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let cards: Vec<(Vec<usize>, Vec<usize>)> = parse_input(input);
    let mut numbers_of_each_card: Vec<usize> = vec![1; cards.len()];
    for (index, curr_card) in cards.iter().enumerate() {
        let num_of_matching_num = get_num_of_matching_numbers(curr_card);
        let num_of_current_card = numbers_of_each_card[index];
        for next_card_to_add in index + 1..(index + 1 + num_of_matching_num as usize) {
            numbers_of_each_card[next_card_to_add] += num_of_current_card;
        }
    }
    Some(numbers_of_each_card.iter().sum())
}

fn get_game_score(game: &(Vec<usize>, Vec<usize>)) -> usize {
    let num_of_matching_numbers = get_num_of_matching_numbers(game);

    if num_of_matching_numbers == 0 {
        return 0;
    }
    return (2 as usize).pow(num_of_matching_numbers - 1);
}

fn get_num_of_matching_numbers(game: &(Vec<usize>, Vec<usize>)) -> u32 {
    let num_of_matching_numbers = game
        .1
        .iter()
        .map(|curr_possessed_number| game.0.contains(curr_possessed_number) as u32)
        .sum::<u32>();
    num_of_matching_numbers
}

fn parse_input(input: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
    return input.split("\n").map(|line| parse_line(line)).collect_vec();
}

fn parse_line(line: &str) -> (Vec<usize>, Vec<usize>) {
    let args = line
        .split(":")
        .nth(1)
        .unwrap()
        .split("|")
        .map(|side| {
            side.trim()
                .split(" ")
                .flat_map(|num| {
                    return num.parse::<usize>();
                })
                .collect_vec()
        })
        .collect_vec();

    return (args[0].clone(), args[1].clone());
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
