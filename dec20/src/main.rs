use std::collections::{HashMap, VecDeque};
use ModuleType::*;
use input_curler::input_for;
use num::integer::lcm;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    Low,
    High
}

#[derive(Debug, Clone)]
struct Module<'a> {
    module_type: ModuleType,
    outputs: Vec<&'a str>,
    flip_flop_state_on: bool,
    last_inputs: HashMap<&'a str, Pulse>
}
impl<'a> Module<'a> {
    fn receive_pulse(&mut self, from: &'a str, pulse: Pulse) -> (Vec<&'a str>, Pulse) {
        match self.module_type {
            Broadcast => {
                (self.outputs.clone(), Pulse::Low)
            },
            FlipFlop => self.ff_receive_pulse(from, pulse),
            Conjunction => self.con_receive_pulse(from, pulse),
        }
    }

    fn ff_receive_pulse(&mut self, _from: &'a str, pulse: Pulse) -> (Vec<&'a str>, Pulse) {
        if pulse == Pulse::High {
            (vec![], Pulse::High)
        } else {
            self.flip_flop_state_on = !self.flip_flop_state_on;
            if self.flip_flop_state_on {
                (self.outputs.clone(), Pulse::High)
            } else {
                (self.outputs.clone(), Pulse::Low)
            }
        }
    }

    fn con_receive_pulse(&mut self, from: &'a str, pulse: Pulse) -> (Vec<&'a str>, Pulse) {
        self.last_inputs.insert(from, pulse);
        if self.last_inputs.values().all(|&v| v == Pulse::High) {
            (self.outputs.clone(), Pulse::Low)
        } else {
            (self.outputs.clone(), Pulse::High)
        }
    }
}

fn main() {
    let data = input_for(20).unwrap();

    let module_map = parse_data(&data);

    let answer_one = part_one(module_map.clone());
    println!("Part one: {}", answer_one);
    let answer_two = part_two(&module_map);
    println!("Part two: {}", answer_two);
}

fn parse_data<'a>(data: &'a str) -> HashMap<&'a str, Module<'a>> {
    let mut inputs_for = HashMap::<&str, Vec<&str>>::new();
    let mut module_map = data.lines().map(|line| {
        let (name_part, out_part) = line.split_once(' ').unwrap();
        let (_arrow, out_list) = out_part.split_once(' ').unwrap();
        let outputs = out_list.split(", ").collect::<Vec<&str>>();
        let (module_type, name) = if name_part == "broadcaster" {
            (Broadcast, name_part)
        } else {
            let (symbol, name) = name_part.split_at(1);
            match symbol {
                "%" => (FlipFlop, name),
                "&" => (Conjunction, name),
                _ => unreachable!()
            }
        };

        for output in outputs.iter() {
            inputs_for.entry(output).or_insert(vec![]).push(name);
        }

        (name, Module {
            module_type,
            outputs,
            flip_flop_state_on: false,
            last_inputs: HashMap::new()
        })
    }).collect::<HashMap<&str, Module>>();

    for (name, module) in module_map.iter_mut() {
        if let Some(inputs) = inputs_for.get(name) {
            module.last_inputs = inputs.iter().map(|&input| (input, Pulse::Low)).collect();
        }
    }

    module_map
}

fn part_one<'a>(mut module_map: HashMap<&'a str, Module<'a>>) -> u64 {
    let mut count_low = 0;
    let mut count_high = 0;

    for _ in 0..1000 {
        let mut pulse_queue = VecDeque::from([("button", "broadcaster", Pulse::Low)]);

        while let Some((src, dest, pulse)) = pulse_queue.pop_front() {
            match pulse {
                Pulse::High => count_high += 1,
                Pulse::Low => count_low += 1
            };

            if dest == "output" { continue; }

            if let Some(receiver) = module_map.get_mut(&dest) {
                let outputs = receiver.receive_pulse(src, pulse);
                for output in outputs.0 {
                    pulse_queue.push_back((dest, output, outputs.1));
                }
            }
        }
    }

    count_high * count_low
}

fn part_two<'a>(module_map: &HashMap<&'a str, Module<'a>>) -> u64 {
    let (&feed_label, feed_module) = module_map.iter().find(|(_, module)| {
        module.outputs.contains(&"rx")
    }).unwrap();

    let must_be_high = feed_module.last_inputs.keys();
    println!("{:?}", must_be_high.clone().collect::<Vec<&&str>>());

    let cycle_lengths = must_be_high.map(|&node_name| {
        let mut working_map = module_map.clone();

        let mut count = 0;
        'outer: loop {
            count += 1;
            let mut pulse_queue = VecDeque::from([("button", "broadcaster", Pulse::Low)]);

            while let Some((src, dest, pulse)) = pulse_queue.pop_front() {
                if dest == "rx" && pulse == Pulse::Low { return count; }
                if dest == "output" { continue; }

                if let Some(receiver) = working_map.get_mut(&dest) {
                    if dest == feed_label && src == node_name && pulse == Pulse::High {
                        break 'outer;
                    }
                    let outputs = receiver.receive_pulse(src, pulse);
                    for output in outputs.0 {
                        pulse_queue.push_back((dest, output, outputs.1));
                    }
                }
            }

            // if working_map.get(feed_label).unwrap().last_inputs.get(node_name).unwrap() == &Pulse::High {
            //     break;
            // }
        }
        println!("{} in {}", node_name, count);
        count
    }).collect::<Vec<u64>>();

    println!("{:?}", cycle_lengths);
    cycle_lengths.into_iter().reduce(lcm).unwrap()
}