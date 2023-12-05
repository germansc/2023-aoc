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
    to: String,
    mappings: Vec<Mapping>,
}

#[derive(Debug, Clone)]
struct Range {
    start: u64,
    end: u64,
}

// ------------------------------------------------------- PART 1 FUNCTIONS ---
//
fn convert_to(convs: &Vec<Conversion>, mut values: Vec<u64>, until: &str) -> Vec<u64> {
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

// ------------------------------------------------------- PART 2 FUNCTIONS ---
//

fn convert_ranges(
    convs: &Vec<Conversion>,
    mut ranges: Vec<Range>,
    source: &str,
    until: &str,
) -> Vec<Range> {
    // Nothing to do in this case...
    if source == until {
        return ranges;
    }

    // Search for the requirements of the conversion.
    let conv = match convs.iter().find(|a| a.to == until) {
        Some(val) => val,
        None => {
            println!("Could not find conversion!");
            return vec![];
        }
    };

    // Lets get recursive, why not?!
    if source != conv.from {
        ranges = convert_ranges(convs, ranges, source, &conv.from);
    }

    print!(
        "Converting {} to {} -> {} mappings {} ranges",
        conv.from,
        conv.to,
        conv.mappings.len(),
        ranges.len()
    );

    let mut converted: Vec<Range> = vec![];

    // For each range, check for mappings interceptions to create new subranges.
    while ranges.len() != 0 {
        let range = ranges.remove(0);
        let mut mapped = false;

        for map in &conv.mappings {
            let map_start = map.src_start;
            let map_end = map_start + map.range;
            let map_offset = map.dst_start.wrapping_sub(map.src_start) as i64;

            if map_end >= range.start && map_start <= range.end {
                // Found some overlap! -> Shift the overlapped range:
                mapped = true;
                let new_start = std::cmp::max(range.start, map_start) as i64 + map_offset;
                let new_end = std::cmp::min(range.end, map_end) as i64 + map_offset;
                converted.push(Range {
                    start: new_start as u64,
                    end: new_end as u64,
                });

                // Make new ranges out of values not mapped.
                if range.start < map_start {
                    ranges.push(Range {
                        start: range.start,
                        end: map_start - 1,
                    });
                }
                // Make new ranges out of values not mapped.
                if range.end > map_end {
                    ranges.push(Range {
                        start: map_end + 1,
                        end: range.end,
                    });
                }
            }
        }

        // No mappings for this range... it is converted as is.
        if !mapped {
            converted.push(range);
        }
    }

    println!(" -> {} resulting ranges", converted.len());

    return converted;
}

//
fn main() {
    println!("2023 Aoc - Day 05");

    // --------------------------------------------------- INPUT COLLECTION ---
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
            to: from_to[2].to_string(),
            mappings: maps,
        });

        // Skip the current empty line.
        i += 1;
    }

    // ------------------------------------------------------------- PART 1 ---
    //
    let mut part1 = convert_to(&conversions, seeds.clone(), "location");
    part1.sort();

    println!("PART 1: {}", part1[0]);

    // ------------------------------------------------------------- PART 2 ---
    // Part 2 - May God have mercy on my ranges...

    // Working with every seed is insane, I'll work with ranges instead.
    let mut inital_ranges: Vec<Range> = vec![];
    for i in 0..seeds.len() / 2 {
        let start: u64 = seeds[2 * i];
        let end: u64 = start + seeds[2 * i + 1];

        inital_ranges.push(Range { start, end });
    }

    // The tricky thing is that every conversion might output more ranges than went in.
    let part2_ranges = convert_ranges(&conversions, inital_ranges.clone(), "seed", "location");

    // Look for the range with the smaller start.
    let mut part2_min = u64::MAX;
    for range in part2_ranges {
        part2_min = std::cmp::min(part2_min, range.start);
    }

    println!("PART 2: {}", part2_min);
}
