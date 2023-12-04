use std::io::{self, Read};

// ------------------------------------------------------- PART 1 Functions ---
struct EnginePart {
    valid: bool,
    number: u32,
}

fn contains_symbol(string: &str) -> bool {
    for c in string.chars() {
        if !"0123456789.".contains(c) {
            return true;
        }
    }

    return false;
}

fn validate_part(sch: &Vec<&str>, x: usize, y: usize, l: usize) -> EnginePart {
    let mut valid: bool = false;
    let current: &str = sch[y];
    let prev: &str = if y > 0 { sch[y - 1] } else { "" };
    let next: &str = if y < sch.len() - 1 { sch[y + 1] } else { "" };
    let leftmost: usize = x.saturating_sub(1);
    let rightmost: usize = std::cmp::min(current.len() - 1, x + l);

    let part: u32 = current[x..x + l]
        .parse()
        .expect("Could not parse part number");

    // Check to the left and right of the number:
    if contains_symbol(&current[leftmost..=rightmost]) {
        valid = true;
    } else if !prev.is_empty() && contains_symbol(&prev[leftmost..=rightmost]) {
        valid = true;
    } else if !next.is_empty() && contains_symbol(&next[leftmost..=rightmost]) {
        valid = true;
    }

    return EnginePart {
        valid,
        number: part,
    };
}

fn analyze_schematic(sch: &Vec<&str>) -> u32 {
    let mut parts: Vec<EnginePart> = Vec::new();

    for (y, line) in sch.iter().enumerate() {
        let mut x: usize = 0;
        while x < line.len() {
            if line.chars().nth(x).unwrap().is_digit(10) {
                let mut e: usize = x + 1;
                while e < line.len() {
                    if line.chars().nth(e).unwrap().is_digit(10) {
                        e += 1
                    } else {
                        break;
                    }
                }

                // Found digit of length e-x at y,x, validate it:
                parts.push(validate_part(sch, x, y, e - x));

                // Keep looking
                x = e + 1;
            } else {
                x += 1;
            }
        }
    }

    // Directly return the sum of valid parts numbers.
    let mut sum: u32 = 0;
    for part in parts {
        if part.valid {
            sum += part.number;
        }
    }

    return sum;
}

// ------------------------------------------------------- PART 2 Functions ---
struct Gear {
    valid: bool,
    ratio: u32,
}

fn get_part_num(s: &Vec<&str>, x: usize, y: usize) -> u32 {
    let mut start: isize = x as isize;
    let mut end = x;
    let line = s[y];

    while start >= 0 && line.chars().nth(start as usize).unwrap().is_digit(10) {
        start -= 1;
    }

    while end < line.len() && line.chars().nth(end).unwrap().is_digit(10) {
        end += 1;
    }

    let start = (start + 1) as usize;
    return line[start..end].parse().expect("Could not parse number");
}

fn validate_gear(s: &Vec<&str>, x: usize, y: usize) -> Gear {
    let mut valid: bool = false;
    let mut parts: Vec<u32> = Vec::new();
    let mut ratio: u32 = 0;

    let current: &str = s[y];
    let prev: &str = if y > 0 { s[y - 1] } else { "" };
    let next: &str = if y < s.len() - 1 { s[y + 1] } else { "" };

    if x != 0 && current.chars().nth(x - 1).unwrap().is_digit(10) {
        parts.push(get_part_num(s, x - 1, y));
    }

    if x != 0 && !prev.is_empty() && prev.chars().nth(x - 1).unwrap().is_digit(10) {
        parts.push(get_part_num(s, x - 1, y - 1));
    }

    if x != 0 && !next.is_empty() && next.chars().nth(x - 1).unwrap().is_digit(10) {
        parts.push(get_part_num(s, x - 1, y + 1));
    }

    if x != current.len() - 1 && current.chars().nth(x + 1).unwrap().is_digit(10) {
        parts.push(get_part_num(s, x + 1, y));
    }

    if x != prev.len() - 1 && !prev.is_empty() && prev.chars().nth(x + 1).unwrap().is_digit(10) {
        parts.push(get_part_num(s, x + 1, y - 1));
    }

    if x != next.len() - 1 && !next.is_empty() && next.chars().nth(x + 1).unwrap().is_digit(10) {
        parts.push(get_part_num(s, x + 1, y + 1));
    }

    if !prev.is_empty() && prev.chars().nth(x).unwrap().is_digit(10) {
        parts.push(get_part_num(s, x, y - 1));
    }

    if !next.is_empty() && next.chars().nth(x).unwrap().is_digit(10) {
        parts.push(get_part_num(s, x, y + 1));
    }

    // Assuming unique parts numbers, I can remove duplicates from parts to know
    // how many parts are adjacent to this gear.
    parts.sort();
    parts.dedup();

    if parts.len() == 2 {
        valid = true;
        ratio = parts[0] * parts[1];
    }

    return Gear { valid, ratio };
}

fn analyze_gears(sch: &Vec<&str>) -> u32 {
    let mut gears: Vec<Gear> = Vec::new();

    for (y, line) in sch.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                // Found a gear.
                gears.push(validate_gear(sch, x, y));
            }
        }
    }

    let mut sum: u32 = 0;
    for gear in gears {
        if gear.valid {
            sum += gear.ratio;
        }
    }

    return sum;
}

fn main() {
    println!("2023 AoC - Day 3");

    let mut schematic: Vec<&str> = Vec::new();
    let mut buff = String::new();

    // Read the whole input until EOF
    io::stdin()
        .read_to_string(&mut buff)
        .expect("Can't read input");

    for line in buff.split('\n') {
        if line.len() != 0 {
            schematic.push(line);
        }
    }

    // Find parts and their validity.
    let part1 = analyze_schematic(&schematic);
    let part2 = analyze_gears(&schematic);

    println!("Part 1: \n{part1}");
    println!("Part 2: \n{part2}");
}
