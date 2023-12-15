use std::{
    collections::HashMap,
    io::{stdin, Read},
};

fn main() {
    println!("2023 AoC - Day 15");

    let mut buff = String::new();
    stdin()
        .read_to_string(&mut buff)
        .expect("Could not read stdin.");

    let chunks: Vec<&str> = buff.trim().split(",").collect();

    // Part 1 ----------------------------------------------------------------
    let mut part1 = 0;
    for chunk in &chunks {
        part1 += compute_hash(chunk);
    }

    println!("PART 1: {part1}");

    // Part 2 ----------------------------------------------------------------

    // Each Hashmap holds the slot and focal lenght of a lens in each box. Since I don't know how
    // to sort and manipulate the order of keys, the lenses vector contains the focal lenght in
    // order of each box.
    let mut boxes: Vec<HashMap<String, (usize, u64)>> = vec![HashMap::new(); 256];
    let mut lenses: Vec<Vec<u64>> = vec![vec![]; 256];

    for chunk in &chunks {
        // Get the label and boxID
        let idx = chunk.chars().position(|c| c == '=' || c == '-').unwrap();
        let label = chunk[0..idx].to_string();
        let box_id = compute_hash(&label) as usize;

        // Get the operation
        let op = chunk.chars().nth(idx).unwrap();
        let fl: u64 = if op == '=' {
            chunk[idx + 1..].parse().unwrap()
        } else {
            0
        };

        // Operate on the box
        if op == '=' {
            // Insert lens at the end of the map. If label was already present, update focal
            // lenght.
            match boxes[box_id].get(&label) {
                Some(val) => {
                    lenses[box_id][val.0] = fl;
                    *boxes[box_id].get_mut(&label).unwrap() = (val.0, fl);
                }
                None => {
                    lenses[box_id].push(fl);
                    boxes[box_id].insert(label, (lenses[box_id].len() - 1, fl));
                }
            }
        } else {
            // Remove lens
            match boxes[box_id].get(&label) {
                Some(val) => {
                    lenses[box_id][val.0] = 0;
                    boxes[box_id].remove(&label);
                }
                None => {}
            }
        }
    }

    // Compute focal lenght.
    let mut part2 = 0;
    for (i, lens_box) in lenses.iter().enumerate() {
        if !lens_box.is_empty() {
            // Remove 0 from each box to compress the lenses array and get correct slot numbers.
            let mut ordered_box = lens_box.clone();
            ordered_box.retain(|&n| n != 0);

            if !ordered_box.is_empty() {
                let mut focal_power = 0;
                for (j, fl) in ordered_box.iter().enumerate() {
                    focal_power += (i + 1) * (j + 1) * *fl as usize;
                }

                println!("Box {i} - Focal power: {focal_power}");
                part2 += focal_power;
            }
        }
    }

    println!("PART 2: {part2}");
}

fn compute_hash(chunk: &str) -> u64 {
    let mut hash = 0;
    for c in chunk.bytes() {
        hash += c as u64;
        hash *= 17;
        hash %= 256;
    }

    return hash;
}
