use std::{fs::read, collections::HashMap};

use itertools::Itertools;

pub fn compute_results(input: Vec<String>) -> (u32, u32) {
    let instructions = parse_input(input);
    return (solve(&instructions, 10), 0);
}

fn solve(instructions: &Vec<((i32, i32), (i32, i32))>, max_sec: u32) -> u32 {
    let mut all_stars = instructions.clone();
    for _ in 0..max_sec {
        Move_stars(&mut all_stars);
        if looks_observable(&all_stars){
            print_stars(&all_stars);
        }
    }

    return 0;
}

fn print_stars(all_stars: &[((i32, i32), (i32, i32))]){
    // let mut map: HashMap<(u32, u32), char> = HashMap::new();
    // for x in -100..100{
    //     for y in -100..100{
    //         map.insert((x,y), '.');
    //     }
    // }
}

fn looks_observable(all_stars: &[((i32, i32), (i32, i32))]) -> bool {
    return true;
}



fn Move_stars(all_stars: &mut Vec<((i32, i32), (i32, i32))>) {
    *all_stars = all_stars
        .iter()
        .map(|curr_star| {
            return (
                (
                    (curr_star.0 .0 + curr_star.1 .0),
                    (curr_star.0 .1 + curr_star.1 .1),
                ),
                (curr_star.1 .0, curr_star.1 .1),
            );
        })
        .collect_vec();
}

fn parse_input(input: Vec<String>) -> Vec<((i32, i32), (i32, i32))> {
    return input
        .iter()
        .map(|line| {
            return (
                (
                    line[10..16].replace(" ", "").parse().unwrap(),
                    line[18..24].replace(" ", "").parse().unwrap(),
                ),
                (
                    line[36..38].replace(" ", "").parse().unwrap(),
                    line[40..42].replace(" ", "").parse().unwrap(),
                ),
            );
        })
        .collect_vec();
}
