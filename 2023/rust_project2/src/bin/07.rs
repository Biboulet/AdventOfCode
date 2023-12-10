advent_of_code::solution!(7);
use std::cmp::Ordering;
use std::collections::HashMap;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    Some(compute_game_score(input, false))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(compute_game_score(input, true))
}

fn parse_input(input: &str) -> Vec<(&str, usize)> {
    return input
        .split("\n")
        .map(|line| {
            (
                line.split(" ").next().unwrap(),
                line.split(" ")
                    .skip(1)
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            )
        })
        .collect_vec();
}

fn compute_game_score(input: &str, is_part2: bool) -> usize {
    let mut games = parse_input(input);
    games
        .sort_by(|a: &(&str, usize), b: &(&str, usize)| is_a_winning_againts_b(a.0, b.0, is_part2));
    return games
        .iter()
        .enumerate()
        .map(|(index, (_, value))| (index + 1) * value)
        .sum();
}

fn is_a_winning_againts_b(a: &str, b: &str, is_part2: bool) -> Ordering {
    let mut card_value: HashMap<char, usize> = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]);
    if is_part2 {
        card_value.insert('J', 0);
    }

    let hand_type_a = get_hand_type(a, is_part2);
    let hand_type_b = get_hand_type(b, is_part2);


    if hand_type_a != hand_type_b {
        return hand_type_a.cmp(&hand_type_b);
    } else {
        return a
            .chars()
            .zip(b.chars())
            .find_map(|(a, b)| {
                let result = card_value[&a].cmp(&card_value[&b]);
                if result != Ordering::Equal {
                    Some(result)
                } else {
                    None
                }
            })
            .unwrap();
    }
}

fn get_hand_type<'a>(a: &str, is_part2: bool) -> usize {
    let mut letters_map = HashMap::new();
    a.chars().for_each(|curr_char| {
        if letters_map.contains_key(&curr_char) {
            letters_map.insert(curr_char, letters_map.get(&curr_char).unwrap() + 1);
        } else {
            letters_map.insert(curr_char, 1);
        }
    });

    let mut max_num_of_a_card = *letters_map.iter().map(|(_, val)| val).max().unwrap();
    if is_part2 {
        max_num_of_a_card = letters_map
            .iter()
            .filter(|(char, _)| **char != 'J')
            .map(|(_, val)| val)
            .max()
            .unwrap_or(&0)
            + letters_map.get(&'J').unwrap_or(&0);
    }

    match max_num_of_a_card {
        5 => return 6,
        4 => return 5,
        3 => {
            if is_part2
                && letters_map
                .values()
                .map(|val| (*val == 2) as usize)
                .sum::<usize>()
                == 1
            {
                return 3;
            }
            if letters_map.values().any(|val| *val == 2) {
                return 4;
            }
            return 3;
        }
        2 => {
            if letters_map
                .values()
                .map(|val| (*val == 2) as usize)
                .sum::<usize>()
                == 2
            {
                return 2;
            }
            return 1;
        }
        1 => return 0,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joker() {
        assert_eq!(get_hand_type("JJJJJ", true), 6);
        assert_eq!(get_hand_type("JJJJA", true), 6);
        assert_eq!(get_hand_type("JJJAA", true), 6);
        assert_eq!(get_hand_type("JJAAA", true), 6);
        assert_eq!(get_hand_type("AJAAA", true), 6);
        assert_eq!(get_hand_type("AKJJJ", true), 5);
        assert_eq!(get_hand_type("AKAJJ", true), 5);
        assert_eq!(get_hand_type("AKAAJ", true), 5);
        assert_eq!(get_hand_type("AKAJK", true), 4);
        assert_eq!(get_hand_type("AKAJT", true), 3);
    }
}
