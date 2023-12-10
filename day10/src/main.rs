use std::io::{self, Read};

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Map {
    width: u64,
    height: u64,
    chars: Vec<char>,
}

fn point_to_index(x: i64, y: i64, map: &Map) -> Option<usize> {
    if x < 0 || y < 0 || x >= map.width as i64 || y >= map.height as i64 {
        return None;
    }

    let index = y * map.width as i64 + x;

    return Some(index as usize);
}

fn index_to_point(idx: usize, map: &Map) -> Option<(i64, i64)> {
    if idx >= map.chars.len() {
        return None;
    }

    let x = idx as u64 % map.width;
    let y = idx as u64 / map.width;

    return Some((x as i64, y as i64));
}

fn main() {
    println!("2023 AoC - Day 10");
    let mut buff = String::new();

    io::stdin()
        .read_to_string(&mut buff)
        .expect("Could not read input!");

    let buff: Vec<&str> = buff.split("\n").filter(|line| !line.is_empty()).collect();
    let height = buff.len() as u64;
    let width = buff[0].len() as u64;

    let mut chars: Vec<char> = vec![];
    for line in buff {
        for char in line.chars() {
            chars.push(char);
        }
    }

    let map = Map {
        width,
        height,
        chars,
    };

    dbg!(map.height, map.width);

    // Part 1 ----------------------------------------------------------------
    // Find the S in the buffer.
    let start = index_to_point(
        map.chars
            .iter()
            .position(|i| *i == 'S')
            .expect("Cant find S"),
        &map,
    )
    .unwrap();

    // Nodes are made of the next cell index, and the direction we arrive from
    let mut nodes: Vec<(usize, Direction)> = vec![];

    // Gather all posible next positions from this point.
    // North
    match point_to_index(start.0, start.1 - 1, &map) {
        Some(idx) => nodes.push((idx, Direction::South)),
        None => { /* Do nothing */ }
    }

    // South
    match point_to_index(start.0, start.1 + 1, &map) {
        Some(idx) => nodes.push((idx, Direction::North)),
        None => { /* Do nothing */ }
    }

    // East
    match point_to_index(start.0 + 1, start.1, &map) {
        Some(idx) => nodes.push((idx, Direction::West)),
        None => { /* Do nothing */ }
    }

    // West
    match point_to_index(start.0 - 1, start.1, &map) {
        Some(idx) => nodes.push((idx, Direction::East)),
        None => { /* Do nothing */ }
    }

    let mut steps: u64 = 1;
    let mut converged_paths = false;
    while !converged_paths {
        let mut next_nodes: Vec<(usize, Direction)> = vec![];

        // Walk each path until two reach the same cell.
        while nodes.len() != 0 {
            let (idx, dir) = nodes.pop().unwrap();
            let (x, y) = index_to_point(idx, &map).unwrap();
            let new_idx: usize;
            match dir {
                Direction::North => match map.chars[idx] {
                    'L' => {
                        new_idx = point_to_index(x + 1, y, &map).unwrap();
                        next_nodes.push((new_idx, Direction::West));
                    }
                    'J' => {
                        new_idx = point_to_index(x - 1, y, &map).unwrap();
                        next_nodes.push((new_idx, Direction::East));
                    }
                    '|' => {
                        new_idx = point_to_index(x, y + 1, &map).unwrap();
                        next_nodes.push((new_idx, Direction::North));
                    }
                    _ => { /*Invalid path, ends here.*/ }
                },
                Direction::South => match map.chars[idx] {
                    'F' => {
                        new_idx = point_to_index(x + 1, y, &map).unwrap();
                        next_nodes.push((new_idx, Direction::West));
                    }
                    '7' => {
                        new_idx = point_to_index(x - 1, y, &map).unwrap();
                        next_nodes.push((new_idx, Direction::East));
                    }
                    '|' => {
                        new_idx = point_to_index(x, y - 1, &map).unwrap();
                        next_nodes.push((new_idx, Direction::South));
                    }
                    _ => { /*Invalid path, ends here.*/ }
                },
                Direction::East => match map.chars[idx] {
                    'F' => {
                        new_idx = point_to_index(x, y + 1, &map).unwrap();
                        next_nodes.push((new_idx, Direction::North));
                    }
                    'L' => {
                        new_idx = point_to_index(x, y - 1, &map).unwrap();
                        next_nodes.push((new_idx, Direction::South));
                    }
                    '-' => {
                        new_idx = point_to_index(x - 1, y, &map).unwrap();
                        next_nodes.push((new_idx, Direction::East));
                    }
                    _ => { /*Invalid path, ends here.*/ }
                },
                Direction::West => match map.chars[idx] {
                    '7' => {
                        new_idx = point_to_index(x, y + 1, &map).unwrap();
                        next_nodes.push((new_idx, Direction::North));
                    }
                    'J' => {
                        new_idx = point_to_index(x, y - 1, &map).unwrap();
                        next_nodes.push((new_idx, Direction::South));
                    }
                    '-' => {
                        new_idx = point_to_index(x + 1, y, &map).unwrap();
                        next_nodes.push((new_idx, Direction::West));
                    }
                    _ => { /*Invalid path, ends here.*/ }
                },
            }
        }

        steps += 1;

        // If any index is repeated in next_nodes, I can end the search.
        let indexes: Vec<usize> = next_nodes.iter().map(|s| s.0).collect();
        for (i, index) in indexes.iter().enumerate() {
            if indexes[i + 1..indexes.len()].contains(index) {
                dbg!(index_to_point(*index, &map).unwrap());
                converged_paths = true;
                break;
            }
        }

        nodes = next_nodes;
    }

    println!("PART1: {steps}");
}
