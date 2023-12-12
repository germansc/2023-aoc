use std::io::stdin;

fn valid_arrangement(line: &str, groups: &Vec<u64>) -> bool {
    let mut count = 0;
    let mut group_idx = 0;

    for c in line.chars() {
        match c {
            '.' => {
                if count > 0 {
                    /* Compare current count with the groups, might return early */
                    if count != groups[group_idx] {
                        return false;
                    } else {
                        count = 0;
                        group_idx += 1;
                    }
                }
            }
            '#' => {
                count += 1;
                // If there are no more groups to compare, we know this is invalid.
                if group_idx == groups.len() {
                    return false;
                }
            }
            _ => {
                println!("Unknown char!");
                return false;
            }
        }
    }

    if group_idx < groups.len() - 1 {
        return false;
    }

    // A last comparison might be needed.
    if group_idx == groups.len() - 1 {
        if count != groups[group_idx] {
            return false;
        } else {
            return true;
        }
    }

    // Else, all groups matched the counts.
    return true;
}

fn line_arrangements(line: &str, groups: &Vec<u64>, up_to: usize) -> u64 {
    if up_to == line.len() {
        return if valid_arrangement(line, groups) {
            1
        } else {
            0
        };
    }

    // Recursive calls to generate all posible cases.
    return match line.chars().nth(up_to).unwrap() {
        '?' => {
            // Generate the two posible strings from this point, and return the sum.
            let string1 = format!("{}{}{}", &line[0..up_to], &".", &line[up_to + 1..]);
            let string2 = format!("{}{}{}", &line[0..up_to], &"#", &line[up_to + 1..]);

            line_arrangements(&string1, groups, up_to + 1)
                + line_arrangements(&string2, groups, up_to + 1)
        }
        _ => line_arrangements(line, groups, up_to + 1),
    };
}

fn group_to_string(groups: &Vec<u64>) -> String {
    return groups.iter().map(|n| n.to_string()).collect::<String>();
}

fn main() {
    println!("2023 AoC - Day 12");

    let mut input: Vec<(String, Vec<u64>)> = vec![];
    let mut buff = String::new();
    while stdin().read_line(&mut buff).expect("Could not read stdin") != 0 {
        let line = buff.trim().split(" ").nth(0).unwrap();
        let groups: Vec<u64> = buff
            .trim()
            .split(" ")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|c| c.parse().expect("Could not parse group!"))
            .collect();

        input.push((line.to_string(), groups));
        buff.clear();
    }

    let mut part1 = 0;

    for item in input {
        let temp = line_arrangements(&item.0, &item.1, 0);
        // println!(
        //     "{} {} -> {temp} arrangements",
        //     &item.0,
        //     group_to_string(&item.1)
        // );
        part1 += temp;
    }

    println!("PART 1: {part1}");
}
