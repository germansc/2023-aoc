use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellType {
    Wall,
    Inside,
    Outside,
    Unknown,
}

#[derive(Debug)]
struct Map {
    width: u64,
    height: u64,
    chars: Vec<char>,
    cells: Vec<CellType>,
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

    let cells: Vec<CellType> = vec![CellType::Unknown; chars.len()];
    let mut map = Map {
        width,
        height,
        chars,
        cells,
    };

    dbg!(map.height, map.width);

    // Part 1 ----------------------------------------------------------------
    // Find the S in the buffer.
    let start = map
        .chars
        .iter()
        .position(|i| *i == 'S')
        .expect("Cant find S");

    map.cells[start] = CellType::Wall;
    let start = index_to_point(start, &map).unwrap();

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
            map.cells[idx] = CellType::Wall;
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
                    _ => {
                        map.cells[idx] = CellType::Unknown; /* Invalid path, ends here.*/
                    }
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
                    _ => {
                        map.cells[idx] = CellType::Unknown; /* Invalid path, ends here.*/
                    }
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
                    _ => {
                        map.cells[idx] = CellType::Unknown; /* Invalid path, ends here.*/
                    }
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
                    _ => {
                        map.cells[idx] = CellType::Unknown; /* Invalid path, ends here.*/
                    }
                },
            }
        }

        steps += 1;

        // If any index is repeated in next_nodes, I can end the search.
        let indexes: Vec<usize> = next_nodes.iter().map(|s| s.0).collect();
        for (i, index) in indexes.iter().enumerate() {
            if indexes[i + 1..indexes.len()].contains(index) {
                converged_paths = true;
                map.cells[*index] = CellType::Wall;
                break;
            }
        }

        nodes = next_nodes;
    }

    println!("PART1: {steps}");

    // Part 2 ----------------------------------------------------------------
    // At this point, walls should be marked in Cells. I can determine if a cell is inside or
    // outisde of the loop by how many walls are crossed. Since we know the loop is closed, 0,0 is
    // either a Wall or Outisde.

    let mut part2 = 0;
    let mut current_state = CellType::Outside;
    let mut flow = Direction::North;

    for i in 0..map.cells.len() {
        // For each cell that is not wall, determine if it is outside or inside.
        // When starting a new row, check if starting position is wall.
        if i & map.width as usize == 0 && map.cells[0] == CellType::Wall {
            current_state = CellType::Inside;
            if "JL".contains(map.chars[i]) {
                flow = Direction::North;
            } else if "F7".contains(map.chars[i]) {
                flow = Direction::South;
            }
        }

        // Check for domain change.
        match map.cells[i] {
            CellType::Wall => {
                if map.chars[i] == '|'
                    || flow == Direction::North && "JL".contains(map.chars[i])
                    || flow == Direction::South && "7F".contains(map.chars[i])
                {
                    current_state = if current_state == CellType::Outside {
                        CellType::Inside
                    } else {
                        CellType::Outside
                    };

                    if "JL".contains(map.chars[i]) {
                        flow = Direction::North;
                    } else if "F7".contains(map.chars[i]) {
                        flow = Direction::South;
                    }
                }
            }
            CellType::Unknown => {
                map.cells[i] = current_state;
                if current_state == CellType::Inside {
                    part2 += 1;
                }
            }
            _ => {}
        }

        if map.cells[0] == CellType::Wall {
            current_state = CellType::Inside;
        }
    }

    println!("PART2: {part2}");
}

fn _print_map(map: &Map) {
    for i in 0..map.cells.len() {
        if i != 0 && i % map.width as usize == 0 {
            println!();
        }

        print!(
            "{}",
            match map.cells[i] {
                CellType::Wall => 'W',
                CellType::Inside => 'I',
                CellType::Outside => 'O',
                CellType::Unknown => '?',
            }
        );
    }

    println!();
}
