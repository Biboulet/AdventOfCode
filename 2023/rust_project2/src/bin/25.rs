use std::{
    cmp::min, collections::{HashMap, HashSet}, ops::Deref, vec
};
use rand::Rng;
use itertools::Itertools;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let nodes: HashMap<&str, Vec<&str>> = parse_input(input);
    let wires_to_be_cut = get_wires_to_be_cut(&nodes);
    dbg!(&wires_to_be_cut);
    let new_nodes = cut_wires(&nodes, wires_to_be_cut);
    let first_group_len: u32 = new_nodes
        .keys()
        .map(|ele| {
            naviguate(
                ele,
                new_nodes.keys().collect_vec()[0],
                HashSet::new(),
                &new_nodes,
            )
            .is_empty() as u32
        })
        .sum();
    Some(first_group_len * (new_nodes.len() as u32 - first_group_len))
}
pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn cut_wires<'a>(
    nodes: &'a HashMap<&str, Vec<&str>>,
    wires_to_be_cut: Vec<(&'a str, &'a str)>,
) -> HashMap<&'a str, Vec<&'a str>> {
    let mut new_map = nodes.clone();
    for wire_to_cut in wires_to_be_cut {
        new_map.insert(
            wire_to_cut.0,
            nodes
                .get(wire_to_cut.0)
                .unwrap()
                .iter()
                .filter(|ele| **ele != wire_to_cut.1)
                .map(|a| *a)
                .collect_vec()
                .clone(),
        );
    }
    return new_map.clone();
}

fn get_wires_to_be_cut<'a>(nodes: &'a HashMap<&str, Vec<&str>>) -> Vec<(&'a str, &'a str)> {
    let mut all_paths: Vec<HashSet<&str>> = vec![];

    for (curr_nodes) in nodes.keys().combinations(2) {
        let rand_num = rand::thread_rng().gen_range(0..=10);
        if 10 == rand_num {
            all_paths.push(naviguate(curr_nodes[0], curr_nodes[1], HashSet::new(), nodes))
        }
    }
    let most_used_components = all_paths.iter().map(|set| set.iter().collect_vec()).concat();

    return vec![most_used_components];
}

//returns the path
fn naviguate<'a>(
    current: &str,
    destination: &str,
    path: HashSet<&str>,
    nodes: &HashMap<&str, Vec<&str>>,
) -> HashSet<&'a str> {
    let mut new_path = path.clone();
    new_path.insert(current);
    if current == destination {
        return new_path.clone();
    }
    nodes.insert("0", nodes.get("0").unwrap().iter().map(|a| *a).chain(vec!["ele"]).collect_vec());
    

    for neighbour in nodes
        .get(current)
        .unwrap()
        .iter()
        .filter(|a| !path.contains(**a))
    {
        let curr_path = naviguate(&neighbour, destination, new_path.clone(), nodes);
        if !curr_path.is_empty(){
            return curr_path;
        }
    }
    return HashSet::new();
}

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.split("\r\n") {
        let args = line.split(": ").collect_vec();
        let component_a = args[0];
        for component_b in args[1].split(" ") {
            let mut new_component_a_value: Vec<&str> =
                map.get(component_a).unwrap_or(&Vec::new()).clone();
            new_component_a_value.push(component_b);
            let mut new_component_b_value: Vec<&str> =
                map.get(component_b).unwrap_or(&Vec::new()).clone();
            new_component_b_value.push(component_a);

            map.insert(component_a, new_component_a_value);
            map.insert(component_b, new_component_b_value);
        }
    }
    return map.clone();
}
