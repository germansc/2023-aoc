#![allow(dead_code)]

use std::{
    collections::HashMap,
    io::{stdin, Read},
};

#[derive(Debug, Clone)]
enum ModuleType {
    None,
    FlipFlop,
    Conjuction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    None,
    High,
    Low,
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    module_type: ModuleType,
    inputs: Vec<String>,
    outputs: Vec<String>,
    current_pulse: Pulse,
    internal_data: u64,
}

fn parse_input() -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    let mut temp_vector: Vec<Module> = vec![];

    let mut buff = String::new();
    stdin()
        .read_to_string(&mut buff)
        .expect("Could not read stdin!");

    // Create each module.
    for line in buff.split("\n") {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Depending on the start, I can determine the type.
        let mut name = line.split(" -").nth(0).unwrap();
        let module_type = match name.chars().nth(0).unwrap() {
            '%' => ModuleType::FlipFlop,
            '&' => ModuleType::Conjuction,
            _ => ModuleType::None,
        };

        if name.contains(['&', '%']) {
            name = &name[1..];
        }

        let outputs: Vec<String> = line
            .split("> ")
            .nth(1)
            .unwrap()
            .trim()
            .split(", ")
            .map(|str| str.to_string())
            .collect();

        // Create the new module. Inputs will be poulated next.
        let module = Module {
            name: name.to_string(),
            module_type,
            inputs: vec![],
            outputs,
            current_pulse: Pulse::Low,
            internal_data: 0,
        };

        temp_vector.push(module);
    }

    // For each module generate its inputs.
    while let Some(mut module) = temp_vector.pop() {
        module.inputs = temp_vector
            .iter()
            .filter(|&m| m.outputs.contains(&module.name))
            .map(|m| m.name.to_string())
            .collect();

        modules.insert(module.name.to_string(), module);
    }

    return modules;
}

fn process_pulse(
    module: &mut Module,
    pulse: Pulse,
    modules: &HashMap<String, Module>,
) -> Vec<(String, Pulse)> {
    let mut next_set: Vec<(String, Pulse)> = vec![];
    match module.module_type {
        ModuleType::None => {
            // Broadcast, sends a low signal to all outputs.
            for m in &module.outputs {
                next_set.push((m.to_string(), Pulse::Low));
                println!("broadcaster -{:?}-> {m}", Pulse::Low);
            }
            module.current_pulse = Pulse::Low;
        }

        ModuleType::FlipFlop => {
            // Ignores High pulses, toggles on low and sends singals.
            match pulse {
                Pulse::Low => {
                    module.internal_data ^= 1;
                    module.current_pulse = if module.internal_data == 1 {
                        Pulse::High
                    } else {
                        Pulse::Low
                    };

                    for m in &module.outputs {
                        next_set.push((m.to_string(), module.current_pulse));
                        println!("{} -{:?}-> {m}", module.name, module.current_pulse);
                    }
                }
                _ => {}
            }
        }

        ModuleType::Conjuction => {
            // If all inputs are High, sends a Low, else sends High.
            let inputs: Vec<Pulse> = modules
                .iter()
                .filter(|(_, m)| m.outputs.contains(&module.name))
                .map(|(_, m)| m.current_pulse)
                .collect();

            module.current_pulse = if inputs.contains(&Pulse::Low) {
                Pulse::High
            } else {
                Pulse::Low
            };

            for m in &module.outputs {
                next_set.push((m.to_string(), module.current_pulse));
                println!("{} -{:?}-> {m}", module.name, module.current_pulse);
            }
        }
    }

    return next_set;
}

fn gather_state(modules: &HashMap<String, Module>) -> Vec<u64> {
    // To ensure consistent order, I'll modules by name.
    let mut temp: Vec<&Module> = modules.iter().map(|(_, m)| m).collect();
    temp.sort_by(|m1, m2| m1.name.cmp(&m2.name));
    let internals = temp.iter().map(|m| m.internal_data).collect();

    return internals;
}

fn process_part1(modules: &HashMap<String, Module>) -> u64 {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut cycles = 0;

    let mut part1_modules = modules.clone();
    let mut processing: Vec<(String, Pulse)> = vec![];

    let mut record: HashMap<Vec<u64>, (u64, u64)> = HashMap::new();

    // Get the initial state:
    let internals = gather_state(&modules);
    record.insert(internals, (low_pulses, high_pulses));

    loop {
        cycles += 1;
        println!(">> Cycle {cycles}");

        println!("button -{:?}-> broadcaster", Pulse::Low);
        processing.push(("broadcaster".to_string(), Pulse::Low));

        while processing.len() != 0 {
            let (name, pulse) = processing.remove(0);

            // Increase statistics.
            match pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1,
                _ => {}
            }

            // Skip processing some named modules that are not defined.
            if name == "output" || name == "rx" {
                continue;
            }

            // Get a mutable handle of the module, and process the signal.
            let reference = part1_modules.clone();
            let module = part1_modules
                .get_mut(name.as_str())
                .expect(format!("Should work {name}").as_str());
            processing.append(&mut process_pulse(module, pulse, &reference));
        }

        // Evaluate current state of internal status and current outputs to detect cycles.
        // dbg!(low_pulses, high_pulses);
        let new_internals = gather_state(&part1_modules);
        if record.get(&new_internals) != None {
            println!("Found a cycle, the signals will repeat after {cycles} rounds.");
            break;
        } else {
            record.insert(new_internals, (low_pulses, high_pulses));
        }

        if cycles == 1000 {
            println!("Reached 1000 cycles... ");
            break;
        }
    }

    dbg!(low_pulses, high_pulses, 1000 / cycles);
    return low_pulses * high_pulses * ((1000 / cycles) as u64).pow(2);
}

fn process_part2() -> u64 {
    return 0;
}

fn main() {
    println!("2023 AoC - Day 20");

    // Input gathering
    let modules = parse_input();

    // Part 1 ---------------------------------------------------------------

    let part1 = process_part1(&modules);

    println!("PART 1: {part1}");

    // Part 2 ---------------------------------------------------------------

    let part2 = process_part2();

    println!("PART 2: {part2}");
}
