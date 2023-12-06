use std::io;

fn main() {
    println!("2023 AoC - Day 06");
    // --------------------------------------------------- INPUT COLLECTION ---
    // Get Time from the first line
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("could not read stdin");

    let time: Vec<u64> = line
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    // Get Distance from the first line
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("could not read stdin");

    let dist: Vec<u64> = line
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    // ------------------------------------------------------------- PART 1 ---
    let winners = get_winning_ranges(&time, &dist);
    let mut part1: u64 = 1;
    for range in winners {
        part1 *= (range.1 + 1) - range.0;
    }

    println!("PART 1: {part1}");

    // ------------------------------------------------------------- PART 2 ---
    let time: u64 = time
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>()
        .parse()
        .unwrap();

    let dist: u64 = dist
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>()
        .parse()
        .unwrap();

    dbg!(&time);
    dbg!(&dist);
    let time = vec![time];
    let dist = vec![dist];

    let winners = get_winning_ranges(&time, &dist);
    let part2: u64 = (winners[0].1 + 1) - winners[0].0;

    println!("PART 2: {part2}");
}

fn get_winning_ranges(time: &Vec<u64>, dist: &Vec<u64>) -> Vec<(u64, u64)> {
    let mut results: Vec<(u64, u64)> = vec![];

    // Find the first winning number and last winning numbers
    for (i, target) in dist.iter().enumerate() {
        let mut low = 0;
        let mut hig = 0;

        for held in 0..=time[i] {
            if ((time[i] - held) * held) > *target {
                low = held;
                break;
            }
        }

        for held in (0..time[i]).rev() {
            if ((time[i] - held) * held) > *target {
                hig = held;
                break;
            }
        }

        println!(
            "Winning range = {},{} [... {} {} ... {} {} ...]/{}",
            low,
            hig,
            ((time[i] - (low - 1)) * (low - 1)),
            ((time[i] - low) * low),
            ((time[i] - hig) * hig),
            ((time[i] - (hig + 1)) * (hig + 1)),
            *target
        );

        results.push((low, hig));
    }

    return results;
}
