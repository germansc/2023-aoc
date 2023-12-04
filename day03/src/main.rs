use std::io::{self, Read};

struct EnginePart {
    valid: bool,
    number: u32,
}

fn check_validity(sch: &Vec<&str>, x: u32, y: u32) -> bool {
    return false;
}

fn analyze_schematic(sch: &Vec<&str>) {
    let mut x = 0;

    for y in 0..sch.len() {
        while x < sch[y].len() {
            let c: char = sch[y].chars().nth(x).unwrap();
            if c.is_digit(10) {
                let e: usize = match sch[y][x..].find(|c: char| !c.is_digit(10)) {
                    Some(num) => num,
                    None => sch[y].len(),
                };
                println!("Found number of len {} at {y},{x}", e - x);
                x = e;
                continue;
            }

            x += 1;
        }

        // Next line.
        x = 0;
    }
}

fn main() {
    println!("2023 AoC - Day 3");

    let mut part1: i32 = 0;
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

    analyze_schematic(&schematic);

    println!("\nThe final sum is: \n{part1}");
}
