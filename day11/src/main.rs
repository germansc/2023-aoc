use std::io::{stdin, Read};

#[derive(Debug)]
struct Map {
    width: u64,
    height: u64,
    chars: Vec<char>,
    erows: Vec<u64>,
    ecols: Vec<u64>,
}

impl Map {
    fn print(&self) {
        println!("Map {}x{}", self.width, self.height);
        for i in 0..self.chars.len() {
            if i != 0 && i % self.width as usize == 0 {
                println!();
            }
            print!("{}", self.chars[i]);
        }

        println!();
    }

    fn to_point(&self, idx: usize) -> Option<(i64, i64)> {
        if idx >= self.chars.len() {
            return None;
        }

        let x = idx as u64 % self.width;
        let y = idx as u64 / self.width;

        return Some((x as i64, y as i64));
    }

    fn distance(&self, idx1: usize, idx2: usize, expansion: u64) -> u64 {
        let p1 = self.to_point(idx1).expect("Invalid index {idx1}!");
        let p2 = self.to_point(idx2).expect("Invalid index {idx1}!");

        // Get the base distance:
        let distance = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();

        // Add <expansion> for each empty row and column crossed.
        let cs = std::cmp::min(p1.0, p2.0) as u64;
        let ce = std::cmp::max(p1.0, p2.0) as u64;
        let mut crosses: u64 = 0;
        for col in &self.ecols {
            if *col > cs && *col < ce {
                crosses += 1;
            }
        }

        let rs = std::cmp::min(p1.1, p2.1) as u64;
        let re = std::cmp::max(p1.1, p2.1) as u64;
        for row in &self.erows {
            if *row > rs && *row < re {
                crosses += 1;
            }
        }

        return distance as u64 + crosses * expansion;
    }
}

fn main() {
    println!("2023 AoC - Day 11");

    // Gather the input map.
    let mut buff = String::new();
    stdin()
        .read_to_string(&mut buff)
        .expect("Can't read stdin!");

    let buff: Vec<&str> = buff.split("\n").filter(|line| !line.is_empty()).collect();
    let height = buff.len() as u64;
    let width = buff[0].len() as u64;

    let mut chars: Vec<char> = vec![];
    for line in buff {
        for char in line.chars() {
            chars.push(char);
        }
    }

    // Mark expansion rows and columns.
    let mut erows: Vec<u64> = vec![];
    for i in 0..height {
        let row_start = i * width;
        let row_end = i * width + width;
        let row = &chars[row_start as usize..row_end as usize];

        if !row.contains(&'#') {
            erows.push(i);
        }
    }

    println!("Found {} empty rows.", erows.len());

    // Mark expansion rows and columns.
    let mut ecols: Vec<u64> = vec![];
    for i in 0..width {
        let col: Vec<char> = chars
            .iter()
            .enumerate()
            .filter(|&(idx, _)| idx as u64 % width == i)
            .map(|(_, c)| *c)
            .collect();

        if !col.contains(&'#') {
            ecols.push(i);
        }
    }

    println!("Found {} empty columns.", ecols.len());

    let map = Map {
        width,
        height,
        chars,
        erows,
        ecols,
    };

    map.print();

    // For each galaxy, calculate the distance to the remaining ones.
    let galaxies: Vec<usize> = map
        .chars
        .iter()
        .collect::<String>()
        .match_indices("#")
        .map(|(i, _)| i)
        .collect();

    println!(
        "Found {} galaxies -> {} pairs.",
        galaxies.len(),
        galaxies.len() * (galaxies.len() - 1) / 2
    );

    // Part 1 ----------------------------------------------------------------
    // Calculate each pair distance.
    let mut distances: Vec<u64> = vec![];
    let mut temp = galaxies.clone();
    while temp.len() != 0 {
        let p1 = temp.pop().unwrap();
        for p2 in &temp {
            distances.push(map.distance(p1, *p2, 1));
        }
    }

    let part1: u64 = distances.iter().sum();
    println!("PART 1: {part1}");

    // Part 2 ----------------------------------------------------------------
    // Calculate each pair distance with an increased expansion!.
    let mut distances: Vec<u64> = vec![];
    let mut temp = galaxies.clone();
    while temp.len() != 0 {
        let p1 = temp.pop().unwrap();
        for p2 in &temp {
            distances.push(map.distance(p1, *p2, 1000000 - 1));
        }
    }

    let part2: u64 = distances.iter().sum();
    println!("PART 2: {part2}");
}
