use std::{
    collections::HashMap,
    io::{stdin, Read},
};

#[derive(Debug)]
struct Part {
    //           x m a s
    attribures: [u64; 4],
}

#[derive(Debug)]
struct Rule {
    attribute: usize,
    operation: char,
    threshold: u64,
    target: String,
}

#[derive(Debug)]
struct Workflow {
    ruleset: Vec<Rule>,
    default: String,
}

fn parse_input() -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut workflows = HashMap::new();
    let mut part_list: Vec<Part> = vec![];

    let mut buff = String::new();
    stdin()
        .read_to_string(&mut buff)
        .expect("Could not read stdin!");

    // Split on the empty line to separate rules from parts description.
    let workf_buff = buff.split("\n\n").nth(0).unwrap();
    let parts_buff = buff.split("\n\n").nth(1).unwrap();

    // Parse workflows:
    for workflow in workf_buff.split("\n") {
        let workflow = workflow.trim();
        if workflow.is_empty() {
            continue;
        }

        let name = workflow.split("{").nth(0).unwrap().to_string();
        let rules = workflow.split("{").nth(1).unwrap();
        let rules = &rules[0..rules.len() - 1];

        let mut ruleset: Vec<Rule> = vec![];
        let mut default: String = "R".to_string();
        for rule in rules.split(",") {
            if rule.contains(":") {
                let operation = rule.chars().nth(1).unwrap();
                let attribute = rule.split(['<', '>', ':']).nth(0).unwrap();
                let threshold = rule.split(['<', '>', ':']).nth(1).unwrap();
                let target = rule.split(['<', '>', ':']).nth(2).unwrap();

                // Parse each element of the Rule
                let attribute = match attribute {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    _ => 9,
                };

                let threshold = threshold.parse::<u64>().unwrap();
                let target = target.to_string();

                ruleset.push(Rule {
                    attribute,
                    operation,
                    threshold,
                    target,
                });
            } else {
                default = rule.to_string();
            }
        }

        workflows.insert(name, Workflow { ruleset, default });
    }

    // Parse parts:
    for part in parts_buff.split("\n") {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        // Trim the {} at start and end
        let part = &part[1..part.len() - 1];

        // Get attributes by index: x = 0, m = 1, a = 2, s = 3.
        // Assuming all attributes are given in order, and all parts have all attributes
        let x = part.split(",").nth(0).unwrap().split("=").nth(1).unwrap();
        let m = part.split(",").nth(1).unwrap().split("=").nth(1).unwrap();
        let a = part.split(",").nth(2).unwrap().split("=").nth(1).unwrap();
        let s = part.split(",").nth(3).unwrap().split("=").nth(1).unwrap();
        let x = x.parse::<u64>().unwrap();
        let m = m.parse::<u64>().unwrap();
        let a = a.parse::<u64>().unwrap();
        let s = s.parse::<u64>().unwrap();

        part_list.push(Part {
            attribures: [x, m, a, s],
        })
    }

    return (workflows, part_list);
}

fn process_part1(workflows: HashMap<String, Workflow>, parts: Vec<Part>) -> u64 {
    let mut approved: Vec<Part> = vec![];
    'next_part: for part in parts {
        // Initial point.
        let mut wf = workflows.get("in").unwrap();

        'next_workflow: loop {
            let mut next_workflow = wf.default.as_str();
            for rule in &wf.ruleset {
                // Evaluate the rule:
                let passed = match rule.operation {
                    '>' => part.attribures[rule.attribute] > rule.threshold,
                    '<' => part.attribures[rule.attribute] < rule.threshold,
                    _ => false,
                };

                if passed {
                    next_workflow = rule.target.as_str();
                    break;
                }
            }
            // No more rules or passed a test, go to the next_workflow
            match next_workflow {
                "A" => {
                    approved.push(part);
                    continue 'next_part;
                }
                "R" => continue 'next_part,
                val => {
                    wf = workflows.get(val).unwrap();
                    continue 'next_workflow;
                }
            }
        }
    }

    // Return the sum of all approved attributes.
    return approved
        .iter()
        .map(|part| part.attribures.iter().sum::<u64>())
        .sum::<u64>();
}

fn main() {
    println!("2023 AoC - Day 19");

    // Input gathering
    let (workflows, parts) = parse_input();

    // Part 1 ---------------------------------------------------------------

    let part1 = process_part1(workflows, parts);

    println!("PART 1: {part1}");
}
