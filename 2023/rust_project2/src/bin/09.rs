advent_of_code::solution!(9);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {

    let all_sequence = parse_input(input);
    Some(all_sequence
        .iter()
        .map(|curr_sequence| get_next_value(curr_sequence))
        .sum())

}

pub fn part_two(input: &str) -> Option<i32> {
    let all_sequence = parse_input(input);
    Some(all_sequence
             .iter()
             .map(|curr_sequence| get_previous_value(curr_sequence))
             .sum())
}


fn parse_input(input: &str) -> Vec<Vec<i32>> {
    return input
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(|arg| arg.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
}


fn get_previous_value(curr_sequence: &Vec<i32>) -> i32 {
    let differenciated_sequence: Vec<Vec<i32>> = get_all_differenciated_sequences(&curr_sequence);
    return differenciated_sequence
        .iter()
        .rev()
        .skip(1)
        .fold(0, |previous_first_value, curr_sequence: &Vec<i32>| {
            curr_sequence.first().unwrap() - previous_first_value
        });
}

fn get_next_value(curr_sequence: &Vec<i32>) -> i32 {
    let differenciated_sequence: Vec<Vec<i32>> = get_all_differenciated_sequences(&curr_sequence);
    return differenciated_sequence
        .iter()
        .rev()
        .skip(1)
        .fold(0, |previous_last_value, curr_sequence: &Vec<i32>| {
            previous_last_value + curr_sequence.last().unwrap()
        });
}

fn get_all_differenciated_sequences(root_sequence: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut all_differenciated_sequences: Vec<Vec<i32>> = vec![root_sequence.clone()];
    while !all_differenciated_sequences
        .last()
        .unwrap()
        .iter()
        .all(|a| *a == 0)
    {
        all_differenciated_sequences.push(
            all_differenciated_sequences
                .last()
                .unwrap()
                .iter()
                .zip(all_differenciated_sequences.last().unwrap().iter().skip(1))
                .map(|(val1, val2)| val2 - val1)
                .collect_vec(),
        );
    }

    return all_differenciated_sequences;
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
