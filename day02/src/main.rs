use std::io;

fn process_game(line: String) -> u32 {
    // Assuming all lines starts with Game ID: Always present
    let e = line.trim().find(":").unwrap();
    let id: u32 = line[5..e].parse().expect("Could not parse Game ID!");

    // Parse each round and extract the values.
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    // Process each round -> Gather the maximum recorded Red, Green and Blue values.
    for round in line[e + 1..].split(';') {
        for val in round.split(',') {
            if val.trim().ends_with("red") {
                red = std::cmp::max(
                    red,
                    val.trim()
                        .split(' ')
                        .nth(0)
                        .unwrap()
                        .parse()
                        .expect("Could not parse red value"),
                );
            } else if val.trim().ends_with("green") {
                green = std::cmp::max(
                    green,
                    val.trim()
                        .split(' ')
                        .nth(0)
                        .unwrap()
                        .parse()
                        .expect("Could not parse green value"),
                );
            } else {
                blue = std::cmp::max(
                    blue,
                    val.trim()
                        .split(' ')
                        .nth(0)
                        .unwrap()
                        .parse()
                        .expect("Could not parse blue value"),
                );
            }
        }
    }

    // Check if the game is possible and return the ID, otherwise, return 0.
    if red > 12 || green > 13 || blue > 14 {
        return 0;
    } else {
        return id;
    }
}

fn main() {
    println!("2023 AoC - Day 2");

    let mut sum: u32 = 0;

    loop {
        let mut str = String::new();
        let bytes: usize = io::stdin().read_line(&mut str).expect("Cant read?!");

        // Check for EoF.
        if bytes == 0 {
            break;
        }

        sum += process_game(str);
    }

    println!("\nThe final sum is: \n{sum}");
}
