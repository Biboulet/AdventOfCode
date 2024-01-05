use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(19);
#[derive(Debug)]
struct Instruction {
    is_final_instr: bool,
    var_name: char,
    var_is_greater: bool,
    const_num: u32,
    output: String,
}
impl Instruction {
    pub fn instr_is_valid(&self, var: u32) -> bool {
        if self.is_final_instr {
            return true;
        }
        // return (var < self.const_num) ^ self.var_is_greater;
        if self.var_is_greater {
            return var > self.const_num;
        } else {
            return var < self.const_num;
        }
    }

    fn new(curr_instruction_input: &str) -> Self {
        if !curr_instruction_input.contains(":") {
            return Instruction {
                is_final_instr: true,
                const_num: 0,
                var_name: ' ',
                var_is_greater: false,
                output: curr_instruction_input.to_string(),
            };
        }
        return Instruction {
            is_final_instr: false,
            var_name: curr_instruction_input.chars().next().unwrap(),
            var_is_greater: curr_instruction_input.chars().nth(1).unwrap() == '>',
            const_num: curr_instruction_input.split(":").next().unwrap()[2..]
                .parse()
                .unwrap(),
            output: curr_instruction_input
                .split(":")
                .nth(1)
                .unwrap()
                .to_string(),
        };
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (workflows, inputs) = parse_input(input);

    Some(
        inputs
            .iter()
            .map(|curr_input| {
                if input_is_accepted(curr_input, &workflows) {
                    return curr_input.values().sum::<u32>();
                } else {
                    return 0;
                }
            })
            .sum(),
    )
}
pub fn part_two(input: &str) -> Option<u32> {
    let (workflows, _) = parse_input(input);
    
}

fn input_is_accepted(
    curr_input: &HashMap<char, u32>,
    workflows: &HashMap<&str, Vec<Instruction>>,
) -> bool {
    let mut curr_workflow = "in";
    loop {
        for curr_instr in workflows.get(curr_workflow).unwrap() {
            if curr_instr.instr_is_valid(*curr_input.get(&curr_instr.var_name).unwrap_or(&0)) {
                if curr_instr.output.len() == 1 {
                    return curr_instr.output == "A";
                }
                curr_workflow = curr_instr.output.as_str();
                break;
            }
        }
    }
}

fn parse_input(input: &str) -> (HashMap<&str, Vec<Instruction>>, Vec<HashMap<char, u32>>) {
    assert!(input.contains("\r\n\r\n"));
    let workflows = input.split("\r\n\r\n").next().unwrap().split("\r\n").fold(
        HashMap::new(),
        |mut map, curr_line| {
            let mut args = curr_line.split("{");
            map.insert(
                args.next().unwrap(),
                parse_instruction(args.next().unwrap().trim_end_matches("}")),
            );
            return map;
        },
    );

    let inputs = input
        .split("\r\n\r\n")
        .nth(1)
        .unwrap()
        .split("\r\n")
        .map(|curr_line| {
            let args = &curr_line[1..curr_line.len() - 1];

            return args.split(",").fold(HashMap::new(), |mut map, curr_var| {
                map.insert(
                    curr_var.chars().next().unwrap(),
                    curr_var.split("=").nth(1).unwrap().parse::<u32>().unwrap(),
                );
                return map;
            });
        })
        .collect_vec();
    return (workflows, inputs);
}

fn parse_instruction(all_instructions_input: &str) -> Vec<Instruction> {
    return all_instructions_input
        .split(",")
        .map(|curr_instruction_input| Instruction::new(curr_instruction_input))
        .collect_vec();
}
