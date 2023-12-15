use std::io::{stdin, Read};

fn main() {
    println!("2023 AoC - Day 15");

    let mut buff = String::new();
    stdin()
        .read_to_string(&mut buff)
        .expect("Could not read stdin.");

    let chunks: Vec<&str> = buff.trim().split(",").collect();

    dbg!(&chunks);

    // Part 1 ----------------------------------------------------------------
    let mut part1 = 0;
    for chunk in chunks {
        part1 += compute_hash(chunk);
    }

    println!("PART 1: {part1}");
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
