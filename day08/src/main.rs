use std::{
    collections::HashMap,
    io::{stdin, Read},
};

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn main() {
    println!("2023 AoC - Day 08");

    // INPUT GATHERING
    let mut pattern = String::new();
    stdin()
        .read_line(&mut pattern)
        .expect("Could not read stdin!");
    pattern = pattern.trim().to_string();

    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read stdin... again");

    let mut node_map = HashMap::new();

    for line in buffer.split('\n') {
        if line.is_empty() {
            continue;
        }

        let name = line.split('=').nth(0).unwrap().trim().to_string();
        let connections = line.split('=').nth(1).unwrap().trim().to_string();
        let left = connections.split(',').nth(0).unwrap()[1..].to_string();
        let right = connections.split(',').nth(1).unwrap()[1..4].to_string();

        node_map.insert(
            name.to_string(),
            Node {
                left: left.to_string(),
                right: right.to_string(),
            },
        );
    }

    // Initial position.
    let part1 = get_steps("AAA", "ZZZ", &pattern, &node_map);

    // Part 1:
    println!("PART 1: {part1}");

    // ------------------------------------------------------------- PART 2 ---
    let mut steps = 0;


fn get_steps(from: &str, to: &str, pattern: &str, map: &HashMap<String, Node>) -> u64 {
    // Initial position.
    let mut current = from;
    let mut steps: u64 = 0;

    while current != to {
        // Get the current node:
        let node = map.get(current).unwrap();

        // Travel in the direction specified by the pattern. I can reuse step as an index counter.
        current = match pattern.chars().nth(steps as usize % pattern.len()).unwrap() {
            'L' => &node.left,
            'R' => &node.right,
            _ => {
                println!("Unknown direction... ");
                return 0;
            }
        };

        steps += 1;
    }

    return steps;
}

fn all_end_with_z(keys: &Vec<&str>) -> bool {
    for key in keys {
        if key.chars().nth(key.len() - 1).unwrap() != 'Z' {
            return false;
        }
    }

    return true;
}
