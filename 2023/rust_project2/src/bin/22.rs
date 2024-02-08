use std::{usize, vec};

use itertools::Itertools;

advent_of_code::solution!(22);

type Coord = (usize, usize, usize);
type Brick = Vec<Coord>;
#[derive(Debug, Clone)]
struct Node {
    supported_by: Vec<usize>,
    supporting: Vec<usize>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let bricks = bricks_fall(&parse_input(input));
    dbg!("test");
    let nodes = generate_nodes(&bricks);
    dbg!("test2");

    Some(
        nodes
            .iter()
            .enumerate()
            .map(|(node_id, _)| (get_length_of_chain_reaction(&nodes, node_id) == 0) as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let bricks = bricks_fall(&parse_input(input));
    let nodes = generate_nodes(&bricks);
    Some(
        nodes
            .iter()
            .enumerate()
            .map(|(node_id, _)| get_length_of_chain_reaction(&nodes, node_id))
            .sum(),
    )
}
fn generate_nodes(bricks: &Vec<Brick>) -> Vec<Node> {
    let mut nodes: Vec<Node> = vec![
        Node {
            supported_by: Vec::new(),
            supporting: Vec::new()
        };
        bricks.len()
    ];
    for (index, curr_brick) in bricks.iter().enumerate() {
        let index_of_all_brick_which_support_current_brick: Vec<usize> = bricks
            .iter()
            .enumerate()
            .filter(|(_, sub_brick)| is_brick_a_under_brick_b(sub_brick, curr_brick))
            .map(|(index, _)| index)
            .collect();
        let index_of_all_brick_supported_by_current_brick: Vec<usize> = bricks
            .iter()
            .enumerate()
            .filter(|(_, sub_brick)| is_brick_a_under_brick_b(curr_brick, *sub_brick))
            .map(|(index, _)| index)
            .collect();

        nodes[index] = Node {
            supported_by: index_of_all_brick_which_support_current_brick.clone(),
            supporting: index_of_all_brick_supported_by_current_brick.clone(),
        }
    }

    return nodes.clone();
}

fn is_brick_a_under_brick_b(brick_a: &Brick, brick_b: &Brick) -> bool {
    let bricks_under_bottom_bricks = get_all_pos_under_brick(&brick_b);
    return bricks_under_bottom_bricks
        .iter()
        .any(|pos_under| brick_a.contains(pos_under));
}

fn get_length_of_chain_reaction(nodes: &[Node], node_id: usize) -> u32 {
    let mut destroyed_brick: Vec<usize> = vec![node_id];
    let mut q: Vec<usize> = nodes[node_id].supporting.clone();
    while !q.is_empty() {
        let curr_node_index = q.pop().unwrap();
        let curr_node = nodes[curr_node_index].clone();
        if curr_node
            .supported_by
            .iter()
            .all(|sub_node| destroyed_brick.contains(sub_node))
        {
            destroyed_brick.push(curr_node_index);
            q.append(&mut curr_node.supporting.clone());
        }
    }

    return destroyed_brick.len() as u32 - 1;
}

fn bricks_fall(bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut new_bricks: Vec<Brick> = Vec::new();
    let mut sorted_bricks = bricks.clone();
    sorted_bricks.sort_by(|curr_brick, other| {
        curr_brick
            .iter()
            .map(|(_, _, z)| *z)
            .min()
            .unwrap()
            .cmp(&other.iter().map(|(_, _, z)| *z).min().unwrap())
    });
    //lent de con
    for curr_brick in sorted_bricks {
        let mut studied_brick = curr_brick.clone();
        while !is_brick_blocked(&studied_brick, &new_bricks) {
            studied_brick = studied_brick
                .iter()
                .map(|(x, y, z)| (*x, *y, (*z as i32 - 1) as usize))
                .collect_vec();
        }
        new_bricks.push(studied_brick.clone());
    }

    return new_bricks;
}

fn is_brick_blocked(curr_brick: &Brick, all_bricks: &Vec<Brick>) -> bool {
    if get_all_pos_under_brick(curr_brick)
        .iter()
        .any(|(_, _, z)| *z == 0)
    {
        return true;
    }
    return all_bricks
        .iter()
        .any(|sub_brick| is_brick_a_under_brick_b(sub_brick, curr_brick));
}

fn get_all_pos_under_brick(curr_brick: &Vec<(usize, usize, usize)>) -> Vec<(usize, usize, usize)> {
    let bottow_pos = curr_brick
        .iter()
        .fold(
            (Vec::new(), usize::MAX),
            |(mut bottom_pos, lowest_z), curr_pos| {
                if curr_pos.2 == lowest_z {
                    bottom_pos.push(curr_pos);
                    return (bottom_pos, lowest_z);
                } else if curr_pos.2 < lowest_z {
                    return (vec![curr_pos], curr_pos.2);
                }
                return (bottom_pos, lowest_z);
            },
        )
        .0;

    let pos_under_bottom_pos = bottow_pos
        .iter()
        .map(|(x, y, z)| (*x, *y, (*z as i32 - 1) as usize))
        .collect_vec();
    pos_under_bottom_pos
}

fn parse_input(input: &str) -> Vec<Brick> {
    let mut bricks = vec![];
    for line in input.split("\r\n") {
        let left_args = line
            .split("~")
            .next()
            .unwrap()
            .split(",")
            .map(|num| num.parse::<usize>().unwrap())
            .collect_vec();
        let right_args = line
            .split("~")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|num| num.parse::<usize>().unwrap())
            .collect_vec();

        let new_brick = (left_args[0]..=right_args[0])
            .into_iter()
            .map(|x| {
                let test = (left_args[1]..=right_args[1])
                    .into_iter()
                    .map(|y| {
                        (left_args[2]..=right_args[2])
                            .into_iter()
                            .map(|z| (x, y, z))
                            .collect_vec()
                    })
                    .concat();
                return test;
            })
            .concat();
        bricks.push(new_brick.clone());
    }
    return bricks;
}
