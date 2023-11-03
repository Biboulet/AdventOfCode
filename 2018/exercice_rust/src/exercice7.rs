use std::{collections::HashMap, collections::VecDeque, vec};

use itertools::Itertools;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn compute_results(input: Vec<String>) -> (String, u32) {
    let instructions = parse_input(input);
    return ("".to_string(), solve_part2(&instructions, 5));
}

fn solve_part2(instructions: &HashMap<char, Vec<char>>, workers_num: u32) -> u32 {
    let mut completed_steps: Vec<char> = Vec::new();
    let mut workers: Vec<(char, u32)> = vec![(' ', 0); workers_num as usize];
    let mut sec = 0;

    while completed_steps.len() < 26 as usize {
        update_workers(&mut workers, &mut completed_steps);
        let mut available_steps: VecDeque<char> =
            get_sorted_available_steps(&instructions, &completed_steps, &workers).into();

        workers = workers
            .iter()
            .map(|curr_worker| {
                if can_work(&curr_worker) {
                    if let Some(i) = available_steps.pop_front() {
                        return (i, compute_working_time(&i));
                    }
                }
                return *curr_worker;
            })
            .collect_vec();
        sec += 1;
    }
    return sec - 1;
}

fn can_work(curr_worker: &(char, u32)) -> bool {
    return curr_worker.1 == 0;
}

fn compute_working_time(i: &char) -> u32 {
    return 60 + ALPHABET.chars().into_iter().position(|a| a == *i).unwrap() as u32 + 1;
}

fn update_workers(workers: &mut Vec<(char, u32)>, completed_steps: &mut Vec<char>) {
    let mut new_completed_steps = completed_steps.clone();
    *workers = workers
        .iter()
        .map(|curr_worker| {
            if curr_worker.0 != ' ' {
                let new_worker = (curr_worker.0, curr_worker.1 - 1);
                if new_worker.1 == 0 {
                    new_completed_steps.push(curr_worker.0);
                    return (' ', 0);
                } else {
                    return new_worker;
                }
            }
            else {
                return (' ', 0);
            }
        })
        .collect_vec();

    *completed_steps = new_completed_steps.clone();
}

fn solve(instructions: &HashMap<char, Vec<char>>) -> String {
    let mut completed_steps = String::new();

    while completed_steps.len() < 26 {
        completed_steps.push(
            get_sorted_available_steps(
                instructions,
                &completed_steps.chars().collect_vec(),
                &[].to_vec(),
            )[0],
        );
    }
    return completed_steps;
}

fn get_sorted_available_steps(
    instructions: &HashMap<char, Vec<char>>,
    completed_steps: &Vec<char>,
    workers: &Vec<(char, u32)>,
) -> Vec<char> {
    instructions
        .iter()
        .filter(|(key, val)| {
            return !completed_steps.contains(*key)
                && !workers.iter().map(|(task, time)| task).contains(*key)
                && val
                    .iter()
                    .all(|condition| completed_steps.contains(condition));
        })
        .map(|(key, _val)| *key)
        .sorted_by_key(|key| *key as u32)
        .collect_vec()
}

fn parse_input(input: Vec<String>) -> HashMap<char, Vec<char>> {
    let mut instructions = ALPHABET.chars().fold(
        HashMap::new(),
        |mut hashmap: HashMap<char, Vec<char>>, curr_char| {
            hashmap.insert(curr_char, vec![]);
            return hashmap;
        },
    );
    for curr_instr in input.iter() {
        let condition = curr_instr.chars().nth(5).unwrap();
        let consequence = curr_instr.chars().nth(36).unwrap();
        instructions.get_mut(&consequence).unwrap().push(condition);
    }
    instructions
}
