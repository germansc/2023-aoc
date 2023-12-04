use std::io::{self, Read};

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

    println!("\nThe final sum is: \n{part1}");
}
