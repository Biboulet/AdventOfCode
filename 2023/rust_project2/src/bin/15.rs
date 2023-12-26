use std::{ops::Rem, vec};

use itertools::Itertools;

advent_of_code::solution!(15);
//(label, focal length)
type Lens<'a> = (&'a str, u32);
pub fn part_one(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    Some(
        instructions
            .iter()
            .map(|curr_instr| hash_instr(curr_instr))
            .sum(),
    )
}
pub fn part_two(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    let boxes = get_boxes(instructions);

    Some(focusing_power(boxes))
}

fn focusing_power(boxes: Vec<Vec<(&str, u32)>>) -> u32 {
    let mut res = 0;
    for (i, curr_box) in boxes.iter().enumerate() {
        for (i2, (_, focal_length)) in curr_box.iter().enumerate() {
            res += (i as u32 + 1) * (i2 as u32 + 1) * *focal_length;
        }
    }
    return res;
}

fn get_boxes(instructions: Vec<&str>) -> Vec<Vec<Lens>> {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    for curr_instruction in instructions {
        let mut args = curr_instruction.split(['=', '-']);
        let label = args.next().unwrap();
        let curr_box = hash_instr(label) as usize;

        let lens_with_same_label = boxes[curr_box]
            .iter()
            .find_position(|(sub_label, _)| *sub_label == label);
        //remove lens
        if curr_instruction.contains("-") {
            if lens_with_same_label.is_some() {
                let index_of_lens_with_same_label = lens_with_same_label.unwrap().0;
                boxes[curr_box].remove(index_of_lens_with_same_label);
            }
        }
        //add or replace lens
        else if curr_instruction.contains("=") {
            let focal_length = args.next().unwrap().parse::<u32>().unwrap();
            //replace lens
            if lens_with_same_label.is_some() {
                let index_of_lens_with_same_label = lens_with_same_label.unwrap().0;
                boxes[curr_box][index_of_lens_with_same_label].1 = focal_length;
            } else {
                boxes[curr_box].push((label, focal_length));
            }
        }
    }
    return boxes.clone();
}

fn hash_instr(instruction: &str) -> u32 {
    let mut curr_val: u32 = 0;
    for char in instruction.chars() {
        curr_val = ((curr_val + (char as u32)) * 17).rem(256);
    }
    return curr_val;
}

fn parse_input(input: &str) -> Vec<&str> {
    return input.split(",").collect_vec();
}
