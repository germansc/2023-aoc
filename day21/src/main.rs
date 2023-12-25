#[allow(dead_code)]
use std::{
    collections::HashMap,
    io::{stdin, Read},
};

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    chars: Vec<char>,
}

impl Map {
    fn to_index(&self, x: i64, y: i64) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
            return None;
        }

        let index = y * self.width as i64 + x;

        return Some(index as usize);
    }

    fn to_point(&self, idx: usize) -> Option<(i64, i64)> {
        if idx >= self.chars.len() {
            return None;
        }

        let x = idx % self.width;
        let y = idx / self.width;

        return Some((x as i64, y as i64));
    }

    fn can_walk(&self, x: i64, y: i64) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
            return None;
        }

        let index = y * self.width as i64 + x;
        if self.chars[index as usize] == '#' {
            return None;
        }

        return Some(index as usize);
    }

    fn can_walk_part2(&self, x: i64, y: i64) -> Option<usize> {
        let x = (self.width as i64 + x) % self.width as i64;
        let y = (self.height as i64 + y) % self.height as i64;

        let index = y * self.width as i64 + x;
        if self.chars[index as usize] == '#' {
            return None;
        }

        return Some(index as usize);
    }

    fn print(&self) {
        println!("Map {}x{}", self.width, self.height);
        for i in 0..self.height {
            let line: String = self.chars[i * self.width..(i + 1) * self.width]
                .iter()
                .collect();
            print!("{}", line);
        }
        println!("\n");
    }
}

fn process_part1(map: &Map) -> u64 {
    // Walk the cells figuring out which are acessible at each iteration.
    let mut nodes: Vec<usize> = vec![];
    let mut record: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut steps = 0;

    // Add the initial spot.
    nodes.push(
        map.chars
            .iter()
            .position(|c| *c == 'S')
            .expect("Start not found?"),
    );

    while steps < 64 {
        let mut next_nodes: Vec<usize> = vec![];
        while nodes.len() != 0 {
            let idx = nodes.pop().unwrap();

            // Check if this cell was already evaulated.
            match record.get(&idx) {
                Some(cells) => next_nodes.extend(cells),
                None => {
                    let (x, y) = map.to_point(idx).unwrap();
                    let mut cell_nodes: Vec<usize> = vec![];
                    for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        if let Some(index) = map.can_walk(x + dir.0, y + dir.1) {
                            cell_nodes.push(index);
                        }
                    }

                    record.insert(idx, cell_nodes.clone());
                    next_nodes.extend(cell_nodes);
                }
            }
        }

        // Remove duplicates.
        next_nodes.sort();
        next_nodes.dedup();

        steps += 1;
        // println!(
        //     "After {steps} steps, the elf can reach {} cells.",
        //     next_nodes.len()
        // );

        nodes = next_nodes;
    }

    // Return the value.
    return nodes.len() as u64;
}

fn process_part2(map: &Map) -> u64 {
    // Walk the cells figuring out which are acessible at each iteration.
    let mut nodes: Vec<usize> = vec![];
    let mut record: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut steps = 0;

    // Add the initial spot.
    nodes.push(
        map.chars
            .iter()
            .position(|c| *c == 'S')
            .expect("Start not found?"),
    );

    while steps < 26501365 {
        let mut next_nodes: Vec<usize> = vec![];
        while nodes.len() != 0 {
            let idx = nodes.pop().unwrap();

            // Check if this cell was already evaulated.
            match record.get(&idx) {
                Some(cells) => next_nodes.extend(cells),
                None => {
                    let (x, y) = map.to_point(idx).unwrap();
                    let mut cell_nodes: Vec<usize> = vec![];
                    for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        if let Some(index) = map.can_walk_part2(x + dir.0, y + dir.1) {
                            cell_nodes.push(index);
                        }
                    }

                    record.insert(idx, cell_nodes.clone());
                    next_nodes.extend(cell_nodes);
                }
            }
        }

        // Remove duplicates.
        next_nodes.sort();
        next_nodes.dedup();

        steps += 1;
        println!(
            "After {steps} steps, the elf can reach {} cells.",
            next_nodes.len()
        );

        nodes = next_nodes;
    }

    // Return the value.
    return nodes.len() as u64;
}

fn main() {
    println!("2023 AoC - Day 21");

    let mut buff = String::new();
    stdin()
        .read_to_string(&mut buff)
        .expect("Could not read stdin");

    let buff: Vec<&str> = buff.split("\n").filter(|line| !line.is_empty()).collect();
    let height = buff.len();
    let width = buff[0].len();

    let mut chars: Vec<char> = vec![];
    for line in buff {
        chars.extend(line.chars());
    }

    let map = Map {
        width,
        height,
        chars,
    };

    // Part 1 ---------------------------------------------------------------
    //

    let part1 = process_part1(&map);
    println!("PART 1: {part1}");

    // Part 2 ---------------------------------------------------------------
    //

    let part2 = process_part2(&map);
    println!("PART 2: {part2}");
}

