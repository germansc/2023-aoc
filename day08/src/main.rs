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

    // My starting position are all nodes that end with 'A'.
    let mut starts: Vec<&str> = vec![];
    for key in node_map.keys() {
        if key.chars().nth(key.len() - 1).unwrap() == 'A' {
            starts.push(key);
        }
    }

    // My ending position are all nodes that end with 'Z'.
    let mut ends: Vec<&str> = vec![];
    for key in node_map.keys() {
        if key.chars().nth(key.len() - 1).unwrap() == 'Z' {
            ends.push(key);
        }
    }

    let mut steps = vec![];

    // Get the steps from each starting point, to each ending point.
    for start in &starts {
        for end in &ends {
            steps.push(get_steps(*start, *end, &pattern, &node_map))
        }
    }

    steps.retain(|&c| c > 0);

    // Get the lower-common-multiplier.
    let mut part2: u64 = 1;
    for i in steps {
        part2 = lcm(part2, i as u64);
    }

    println!("PART 2: {part2}");
}

fn get_steps(from: &str, to: &str, pattern: &str, map: &HashMap<String, Node>) -> i64 {
    // Initial position.
    let mut current = from;
    let mut steps: i64 = 0;

    while current != to {
        // Get the current node:
        let node = match map.get(current) {
            Some(node) => node,
            None => {
                println!("Could not find starting position.");
                return -1;
            }
        };

        // Travel in the direction specified by the pattern. I can reuse step as an index counter.
        current = match pattern.chars().nth(steps as usize % pattern.len()).unwrap() {
            'L' => &node.left,
            'R' => &node.right,
            _ => {
                println!("Unknown direction... ");
                return -1;
            }
        };

        steps += 1;

        // Arbitrary number to decide that this path is unreachable.
        if steps > 25000 {
            return -1;
        }
    }

    return steps;
}

fn lcm(x: u64, y: u64) -> u64 {
    // Get the GCD
    let mut rem;
    let mut temp;
    let mut gcd;

    if x > y {
        rem = x;
        gcd = y;
    } else {
        rem = y;
        gcd = x;
    }

    // Get the GCD first
    rem = rem % gcd;
    while rem != 0 {
        temp = gcd;
        gcd = rem;
        rem = temp % gcd;
    }

    return x * y / gcd;
}
