use std::io::{self, Read};

fn main() {
    println!("2023 AoC - Day 09");

    // Input gathering
    let mut buff = String::new();
    io::stdin()
        .read_to_string(&mut buff)
        .expect("Could not read stdin.");

    let mut readings: Vec<Vec<i64>> = vec![];
    for line in buff.split('\n') {
        if line.is_empty() {
            continue;
        }

        readings.push(
            line.split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect(),
        );
    }

    // Let's remove the mutable atribute to the original input.
    let readings = readings;

    // Part 1 ---------------------------------------------------------------
    let mut part1: i64 = 0;

    for vector in &readings {
        part1 += interpolate_readings(vector);
    }

    println!("PART 1: {part1}");
}

fn interpolate_readings(values: &[i64]) -> i64 {
    let mut diff: Vec<i64> = vec![];

    for i in 1..values.len() {
        diff.push(values[i] - values[i - 1]);
    }

    // If all diffs are equal, I can directly interpolate this vector.
    let delta: i64;
    if diff.iter().filter(|&i| *i == diff[0]).count() == diff.len() {
        delta = diff[0];
    } else {
        // Interpolate the diff vector!
        delta = interpolate_readings(&diff);
    }

    return values[values.len() - 1] + delta;
}
