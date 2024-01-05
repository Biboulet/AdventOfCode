use std::{collections::HashMap, vec};

use itertools::Itertools;

advent_of_code::solution!(20);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    Low,
    High,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Module<'a> {
    is_flip_flop: bool,
    name: &'a str,
    output: Vec<&'a str>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let modules: HashMap<&str, Module> = parse_input(input);
    Some(press_button(&modules, 1000, false))
}
pub fn part_two(input: &str) -> Option<u32> {
    let modules: HashMap<&str, Module> = parse_input(input);
    Some(press_button(&modules, 100000000, true))
}

fn press_button(modules: &HashMap<&str, Module>, num_of_press: i32, is_part2: bool) -> u32 {
    let mut all_signal_count: Vec<(u32, u32)> = vec![];
    let mut flip_flop_state: HashMap<&str, bool> =
        modules
            .iter()
            .map(|(_, module)| module)
            .fold(HashMap::new(), |mut map, curr_module| {
                if curr_module.is_flip_flop {
                    map.insert(curr_module.name, false);
                }
                return map;
            });

    let mut conjonction_states: HashMap<&str, HashMap<&str, Pulse>> = modules
        .iter()
        .map(|(_, module)| module)
        .fold(HashMap::new(), |mut map, curr_module| {
            if !curr_module.is_flip_flop && curr_module.name != "broadcaster" {
                map.insert(
                    curr_module.name,
                    modules
                        .iter()
                        .filter(|(_, sub_module)| sub_module.output.contains(&curr_module.name))
                        .map(|(module_name, _)| (*module_name, Pulse::Low))
                        .collect(),
                );
            }
            return map;
        });
    
    // for part 2 -->
    // let module_linked_to_rx = *modules.iter().find(|(_, sub_mod)| sub_mod.output.contains(&"rx")).unwrap().0;
    // let condition_to_get_a_low_pulse_to_rx = conjonction_states.get(module_linked_to_rx).unwrap().keys().collect_vec();
    // let condition_to_get_a_low_pulse_to_rx_appearence_memo = vec![0;condition_to_get_a_low_pulse_to_rx.len()];
    // <-- for part 2
    //partie 2 faite a la main a finir
    
    let mut i = 1;
    while i <= num_of_press && !is_part2{
        // (LowPulse, HighPulse)
        let mut signal_count: (u32, u32) = (1, 0);
        // (sender, pulse, receiver)
        let mut q: Vec<(&str, Pulse, &str)> = vec![];
        q.push(("Button", Pulse::Low, "broadcaster"));
        while !q.is_empty() {
            let current_task = q.remove(0);
            // dbg!(&current_task);
            if current_task.2 == "rx" && current_task.1 == Pulse::Low{
                return i as u32;
            }
            if modules.get(current_task.2).is_none(){
                continue;
            }
            let receiver_module = modules.get(current_task.2).unwrap();
            let mut next_pulse_value = Pulse::Low;
            if receiver_module.is_flip_flop && current_task.1 == Pulse::High {
                continue;
            } else if receiver_module.name == "broadcaster" {
                next_pulse_value = Pulse::Low;
            } else if receiver_module.is_flip_flop {
                let flip_flop_is_on = *flip_flop_state.get(receiver_module.name).unwrap();
                if flip_flop_is_on == false {
                    next_pulse_value = Pulse::High;
                } else {
                    next_pulse_value = Pulse::Low;
                }
                flip_flop_state.insert(receiver_module.name, !flip_flop_is_on);
            } else if !receiver_module.is_flip_flop {
                let mut connected_inputs_memo = conjonction_states
                    .get(receiver_module.name)
                    .unwrap()
                    .to_owned();

                connected_inputs_memo.insert(current_task.0, current_task.1);
                conjonction_states.insert(receiver_module.name, connected_inputs_memo.clone());
                // dbg!(&connected_inputs_memo);
                if connected_inputs_memo
                    .iter()
                    .all(|(_, pulse_value)| *pulse_value == Pulse::High)
                {
                    next_pulse_value = Pulse::Low;
                } else {
                    next_pulse_value = Pulse::High
                }
            }

            let next_tasks = receiver_module
                .output
                .iter()
                .map(|curr_output| (receiver_module.name, next_pulse_value.clone(), *curr_output))
                .collect_vec();
            signal_count =
            update_signal_pressed_count(signal_count, next_tasks.len(), &next_pulse_value);
            q.extend(next_tasks);
            // dbg!(&q);
        }
        all_signal_count.push(signal_count.clone());
        i+=1;
    }

    return all_signal_count.iter().map(|(_, b)| b).sum::<u32>()
        * all_signal_count.iter().map(|(a, _)| a).sum::<u32>();
}

fn update_signal_pressed_count(
    signal_count: (u32, u32),
    len: usize,
    next_pulse_value: &Pulse,
) -> (u32, u32) {
    return (
        signal_count.0 + len as u32 * (*next_pulse_value == Pulse::Low) as u32,
        signal_count.1 + len as u32 * (*next_pulse_value == Pulse::High) as u32,
    );
}

fn parse_input(input: &str) -> HashMap<&str, Module<'_>> {
    return input
        .split("\r\n")
        .map(|curr_line| {
            let curr_module = parse_module(curr_line);
            return (curr_module.name, curr_module.clone());
        })
        .collect::<HashMap<&str, Module>>();
}

fn parse_module(line: &str) -> Module<'_> {
    return Module {
        name: line
            .trim_start_matches(['&', '%'])
            .split(" -> ")
            .next()
            .unwrap(),
        is_flip_flop: line.chars().next().unwrap() == '%',
        output: line
            .split(" -> ")
            .nth(1)
            .unwrap()
            .trim()
            .split(", ")
            .collect_vec(),
    };
}