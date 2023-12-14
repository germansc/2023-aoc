use std::io::{stdin, Read};

#[derive(Debug, Clone)]
struct Pattern {
    width: usize,
    height: usize,
    buff: Vec<char>,
}

impl Pattern {
    fn to_index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let idx = y * self.width + x;

        return Some(idx);
    }

    fn print(&self) {
        println!("Map {}x{}", self.width, self.height);
        for i in 0..self.buff.len() {
            if i != 0 && i % self.width as usize == 0 {
                println!();
            }
            print!("{}", self.buff[i]);
        }

        println!();
    }
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

    for pat in &patterns {
        let va = find_vertical_axis(&pat, 0);
        if va > 0 {
            vert_idx.push(va);
            continue;
        }

        let ha = find_horizontal_axis(&pat, 0);
        hori_idx.push(ha);
        continue;
    }

    let part1: u64 = vert_idx.iter().sum::<u64>() + 100 * hori_idx.iter().sum::<u64>();
    println!("PART 1: {part1}");

    // --------------------------------------------------------------- Part 2
    //
    let mut part2: u64 = 0;
    for pat in patterns {
        let temp = fix_smudges(&pat);
        part2 += temp.0 + 100 * temp.1;
    }

    println!("PART 2: {part2}");
}

fn fix_smudges(pattern: &Pattern) -> (u64, u64) {
    // Get reference values of mirroring to avoid.
    let h_ref = find_horizontal_axis(pattern, 0);
    let v_ref = find_vertical_axis(pattern, 0);

    // Look for rows that differ in one character.
    for i in 0..pattern.height - 1 {
        let row1: Vec<char> = pattern.buff[i * pattern.width..(i + 1) * pattern.width].to_vec();

        for j in i + 1..pattern.height {
            let row2: Vec<char> = pattern.buff[j * pattern.width..(j + 1) * pattern.width].to_vec();

            let diff: Vec<u64> = row1
                .clone()
                .into_iter()
                .zip(&row2)
                .map(|(a, b)| if a == *b { 0 } else { 1 })
                .collect();

            if diff.iter().sum::<u64>() == 1 {
                // println!(
                //     "Found a row smudge! [{i}] {} vs [{j}] {}",
                //     &row1.iter().collect::<String>(),
                //     &row2.iter().collect::<String>()
                // );

                let x = diff.iter().position(|&u| u == 1).unwrap();
                let from = pattern.to_index(x, i).unwrap();
                let to = pattern.to_index(x, j).unwrap();

                // Swap one row with the other, and try to get a hor axis.
                // Skip rows up to this change to avoid previous solution.
                let mut test = pattern.clone();
                test.buff[to] = pattern.buff[from];
                let ha = find_horizontal_axis(&test, i);
                if ha > 0 && ha != h_ref {
                    println!("Found haxis at {ha}");
                    return (0, ha);
                }
            }
        }
    }

    // Look for columns that differ in one character.
    for i in 0..pattern.width - 1 {
        let col1: Vec<char> = pattern
            .buff
            .iter()
            .enumerate()
            .filter(|&(idx, _)| idx % pattern.width == i)
            .map(|(_, c)| *c)
            .collect();

        for j in i + 1..pattern.width {
            let col2: Vec<char> = pattern
                .buff
                .iter()
                .enumerate()
                .filter(|&(idx, _)| idx % pattern.width == j)
                .map(|(_, c)| *c)
                .collect();

            let diff: Vec<u64> = col1
                .clone()
                .into_iter()
                .zip(&col2)
                .map(|(a, b)| if a == *b { 0 } else { 1 })
                .collect();

            if diff.iter().sum::<u64>() == 1 {
                // println!(
                //     "Found a col smudge! [{i}] {} vs [{j}] {}",
                //     &col1.iter().collect::<String>(),
                //     &col2.iter().collect::<String>()
                // );

                let y = diff.iter().position(|&u| u == 1).unwrap();
                let from = pattern.to_index(i, y).unwrap();
                let to = pattern.to_index(j, y).unwrap();

                // Swap one row with the other, and try to get a hor axis.
                // Skip columns up to this change to avoid previous solution.
                let mut test = pattern.clone();
                test.buff[to] = pattern.buff[from];
                let va = find_vertical_axis(&test, i);
                if va > 0 && va != v_ref {
                    println!("Found vaxis at {va}");
                    return (va, 0);
                }
            }
        }
    }

    println!("> No change found? h:{h_ref} | v:{v_ref}");
    pattern.print();
    return (0, 0);
}

fn find_vertical_axis(pattern: &Pattern, skip: usize) -> u64 {
    if skip >= pattern.width - 1 {
        return 0;
    }

    // Find two contiguous identical columns.
    for i in skip + 1..pattern.width {
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

fn find_horizontal_axis(pattern: &Pattern, skip: usize) -> u64 {
    if skip >= pattern.height - 1 {
        return 0;
    }
    // Find two contiguous identical rows.
    for i in skip + 1..pattern.height {
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
