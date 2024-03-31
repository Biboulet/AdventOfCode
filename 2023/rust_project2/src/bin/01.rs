advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    Some(input
        .split('\n')
        .map(|line| {
            let first_digit: usize = line
                .chars()
                .find_map(|a| a.to_string().parse().ok())
                .unwrap();
            let second_digit: usize = line
                .chars()
                .rev()
                .find_map(|a| a.to_string().parse().ok())
                .unwrap();
            10 * first_digit + second_digit
        })
        .sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let real_input = input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");
    Some(part_one(&real_input).unwrap())
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
