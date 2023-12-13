use std::io::{stdin, Read};

#[derive(Debug)]
struct Pattern {
    width: usize,
    height: usize,
    buff: Vec<char>,
}

fn main() {
    println!("2023 AoC - Day 13");

    // Input gathering.
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Could not read stdin!");

    let mut patterns: Vec<Pattern> = vec![];

    // Split by pattern cutting at double newlines.
    for line in input.split("\n\n") {
        let buff = line.trim().split("\n").collect::<Vec<&str>>();
        let height = buff.len();
        let width = buff[0].len();

        let buff = buff.join("").to_string().chars().collect::<Vec<char>>();
        patterns.push(Pattern {
            width,
            height,
            buff,
        });
    }

    // Remove mutability.
    let patterns = patterns;

    // --------------------------------------------------------------- Part 1
    let mut vert_idx: Vec<u64> = vec![];
    let mut hori_idx: Vec<u64> = vec![];

    for pat in patterns {
        let va = find_vertical_axis(&pat);
        if va > 0 {
            vert_idx.push(va);
            continue;
        }

        let ha = find_horizontal_axis(&pat);
        hori_idx.push(ha);
        continue;
    }

    let part1: u64 = vert_idx.iter().sum::<u64>() + 100 * hori_idx.iter().sum::<u64>();
    println!("PART 1: {part1}");
}

fn find_vertical_axis(pattern: &Pattern) -> u64 {
    // Find two contiguous identical columns.
    for i in 1..pattern.width {
        let mut j = 1;
        let mut col1: Vec<char> = pattern
            .buff
            .iter()
            .enumerate()
            .filter(|&(idx, _)| idx % pattern.width == i)
            .map(|(_, c)| *c)
            .collect();

        let mut col2: Vec<char> = pattern
            .buff
            .iter()
            .enumerate()
            .filter(|&(idx, _)| idx % pattern.width == i - 1)
            .map(|(_, c)| *c)
            .collect();

        while col1 == col2 {
            // check backwards
            let up = i + j;
            let down = i as isize - 1 - j as isize;

            if up == pattern.width || down < 0 {
                return i as u64;
            }

            col1 = pattern
                .buff
                .iter()
                .enumerate()
                .filter(|&(idx, _)| idx % pattern.width == down as usize)
                .map(|(_, c)| *c)
                .collect();

            col2 = pattern
                .buff
                .iter()
                .enumerate()
                .filter(|&(idx, _)| idx % pattern.width == up as usize)
                .map(|(_, c)| *c)
                .collect();
            j += 1;
        }
    }

    return 0;
}

fn find_horizontal_axis(pattern: &Pattern) -> u64 {
    // Find two contiguous identical rows.
    for i in 1..pattern.height {
        let mut j = 1;
        let mut row1: Vec<char> = pattern.buff[i * pattern.width..(i + 1) * pattern.width].to_vec();
        let mut row2: Vec<char> = pattern.buff[(i - 1) * pattern.width..i * pattern.width].to_vec();

        while row1 == row2 {
            // check backwards
            let up = i + j;
            let down = i as isize - 1 - j as isize;

            if up == pattern.height || down < 0 {
                return i as u64;
            }

            let down = down as usize;
            row1 = pattern.buff[down * pattern.width..(down + 1) * pattern.width].to_vec();
            row2 = pattern.buff[up * pattern.width..(up + 1) * pattern.width].to_vec();
            j += 1;
        }
    }

    return 0;
}
