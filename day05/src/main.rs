use std::io::{self, Read};

#[derive(Debug)]
struct Mapping {
    src_start: u64,
    dst_start: u64,
    range: u64,
}

#[derive(Debug)]
struct Conversion {
    from: String,
    _to: String,
    mappings: Vec<Mapping>,
}

fn convert_to(convs: &Vec<Conversion>, mut values: Vec<u64>, until: String) -> Vec<u64> {
    for conv in convs {
        // Check end of conversion...
        if conv.from == until {
            break;
        }

        // For each value, check if it fits in any mapping.
        for i in 0..values.len() {
            for map in &conv.mappings {
                let index = values[i].wrapping_sub(map.src_start);
                if index < map.range {
                    values[i] = map.dst_start + index;
                    break;
                }
            }
        }
    }

    return values;
}

fn main() {
    println!("2023 Aoc - Day 05");

    // Get seeds from the first line
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Could not read stdin");

    let seeds: Vec<u64> = line
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    // Generate the mappings.
    let mut conversions: Vec<Conversion> = vec![];

    // Read the whole input until EOF
    let mut buff = String::new();
    io::stdin()
        .read_to_string(&mut buff)
        .expect("Can't read input");

    //Skip first line, as it should be empty.
    let buff: Vec<&str> = buff.split('\n').skip(1).collect();

    let mut i = 0;
    while i < buff.len() {
        let line = buff[i];
        let mut maps: Vec<Mapping> = vec![];
        let from_to: Vec<&str> = line.split(" ").nth(0).unwrap().split("-").collect();
        i += 1;
        while buff[i] != "" {
            let vals: Vec<u64> = buff[i]
                .trim()
                .split(" ")
                .map(|s| s.parse().unwrap())
                .collect();

            maps.push(Mapping {
                src_start: vals[1],
                dst_start: vals[0],
                range: vals[2],
            });

            i += 1;
        }

        // Create the new conversion!
        conversions.push(Conversion {
            from: from_to[0].to_string(),
            _to: from_to[2].to_string(),
            mappings: maps,
        });

        // Skip the current empty line.
        i += 1;
    }

    // Part 1: Get the smaller location.
    let mut part1 = convert_to(&conversions, seeds.clone(), "location".to_string());
    part1.sort();

    println!("PART 1: {}", part1[0]);
}
