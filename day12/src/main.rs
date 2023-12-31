use std::{collections::HashMap, io::stdin};

fn block_arrangements(slice: &str, groups: &Vec<u64>, cache: &mut HashMap<String, u64>) -> u64 {
    // Early return.
    // If the slice is empty, the arrengement is valid if there are no more expected groups.
    if slice.is_empty() {
        if groups.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    // Early return.
    // If no more groups are expected, the line cannot contain '#'.
    if groups.is_empty() {
        if slice.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }

    let mut trimmed_slice = slice.to_string();
    // Slice any heading '.', they can be ignore to determine the iterations of a block.
    match slice.chars().position(|c| c != '.') {
        Some(idx) => {
            trimmed_slice = slice[idx..].to_string();
        }
        _ => {}
    }

    // Check if this combination of slice and groups was already calculated:
    let key = vec![slice.to_string(), group_to_string(&groups)].join("-");
    match cache.get(&key) {
        Some(num) => return *num,
        None => {}
    }

    let mut count = 0;

    // At this point, trimmed_slice begins with either a # or a ?. In the latter case, append the
    // subresult in case it was a '.'
    if trimmed_slice.chars().nth(0).unwrap() == '?' {
        count += block_arrangements(&trimmed_slice[1..], groups, cache);
    }

    // We still have some condition where block requirements cannot be fulfilled.
    if groups[0] as usize > trimmed_slice.len()
        || trimmed_slice[0..groups[0] as usize].contains('.')
        || (trimmed_slice.len() > groups[0] as usize
            && trimmed_slice.chars().nth(groups[0] as usize).unwrap() == '#')
    {
        return count;
    }

    // Recursively analyze next block arrangement.
    let subgroup: Vec<u64>;
    if groups.len() > 1 {
        subgroup = groups[1..].iter().map(|&u| u).collect();
    } else {
        subgroup = vec![];
    }

    let skip = 1 + groups[0] as usize;
    if skip > trimmed_slice.len() {
        trimmed_slice = "".to_string();
    } else {
        trimmed_slice = trimmed_slice[skip..].to_string();
    }

    count += block_arrangements(&trimmed_slice, &subgroup, cache);

    // Store the result for this combination of slice and groups in a cache.
    cache.insert(key, count);

    return count;
}

fn group_to_string(groups: &Vec<u64>) -> String {
    return groups
        .iter()
        .map(|n| format!(" {}", n.to_string()))
        .collect();
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

    let mut precomputed: HashMap<String, u64> = HashMap::new();
    let mut part1 = 0;

    for item in &input {
        let temp = block_arrangements(&item.0, &item.1, &mut precomputed);
        // println!(
        //     "{} [{}] -> {} arrangements.",
        //     item.0,
        //     group_to_string(&item.1),
        //     temp,
        // );

        part1 += temp;
    }

    println!("PART 1: {part1}");

    // PART 2 ----------------------------------------------------------------
    let mut part2 = 0;

    for item in &input {
        let exp_line = vec![item.0.to_owned(); 5];
        let exp_line = exp_line.join("?");

        let exp_groups = vec![item.1.to_owned(); 5];
        let exp_groups: Vec<u64> = exp_groups.into_iter().flatten().collect();

        let temp = block_arrangements(&exp_line, &exp_groups, &mut precomputed);
        // println!(
        //     "{} [{}] -> {} arrangements.",
        //     exp_line,
        //     group_to_string(&exp_groups),
        //     temp,
        // );

        part2 += temp;
    }

    println!("PART 2: {part2}");
}
