use std::io::{stdin, Read};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    chars: Vec<char>,
    visited: Vec<Vec<Direction>>,
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

    fn next(&self, idx: usize, direction: Direction) -> Option<usize> {
        let (x, y) = self.to_point(idx).unwrap();
        let next_point: (i64, i64);
        match direction {
            Direction::North => next_point = (x, y - 1),
            Direction::South => next_point = (x, y + 1),
            Direction::East => next_point = (x + 1, y),
            Direction::West => next_point = (x - 1, y),
        };

        // Check if this cell was already visited in this direction.
        match self.to_index(next_point.0, next_point.1) {
            Some(idx) => {
                if !self.visited[idx].contains(&direction) {
                    return Some(idx);
                }
            }
            None => {}
        }

        return None;
    }

    fn _print(&self) {
        println!("Map {}x{}", self.width, self.height);
        for i in 0..self.chars.len() {
            if i != 0 && i % self.width as usize == 0 {
                println!();
            }

            let mut c: char = self.chars[i];
            if c == '.' {
                match self.visited[i].len() {
                    0 => {}
                    1 => match self.visited[i][0] {
                        Direction::North => c = '^',
                        Direction::South => c = 'v',
                        Direction::East => c = '>',
                        Direction::West => c = '<',
                    },
                    2 => c = '2',
                    3 => c = '3',
                    4 => c = '4',
                    _ => c = '?',
                }
            }

            print!("{}", c);
        }
        println!("\n");
    }
}

fn main() {
    println!("2023 AoC - Day 16");

    let mut buff = String::new();
    stdin()
        .read_to_string(&mut buff)
        .expect("Could not read stdin");

    let buff: Vec<&str> = buff.split("\n").filter(|line| !line.is_empty()).collect();
    let height = buff.len();
    let width = buff[0].len();

    let mut chars: Vec<char> = vec![];
    for line in buff {
        for char in line.chars() {
            chars.push(char);
        }
    }

    let visited: Vec<Vec<Direction>> = vec![vec![]; chars.len()];
    let mut map = Map {
        width,
        height,
        chars,
        visited,
    };

    // Part 1 ---------------------------------------------------------------
    //

    // map._print();
    let part1 = propagate_light(&mut map, (0, Direction::East));
    // map._print();

    println!("PART 1: {part1}");

    // Part 1 ---------------------------------------------------------------
    //

    // Bruteforce?
    let mut part2 = 0;

    // Starting at North:
    for i in 0..map.width {
        // reset visited.
        map.visited = vec![vec![]; map.chars.len()];
        let temp = propagate_light(&mut map, (i, Direction::South));

        part2 = std::cmp::max(part2, temp);
    }

    // Starting at East:
    for i in 0..map.height {
        // reset visited.
        map.visited = vec![vec![]; map.chars.len()];
        let idx = map.to_index((map.width - 1) as i64, i as i64).unwrap();
        let temp = propagate_light(&mut map, (idx, Direction::West));

        part2 = std::cmp::max(part2, temp);
    }

    // Starting at South:
    for i in 0..map.width {
        // reset visited.
        map.visited = vec![vec![]; map.chars.len()];
        let idx = map.to_index(i as i64, (map.height - 1) as i64).unwrap();
        let temp = propagate_light(&mut map, (idx, Direction::North));

        part2 = std::cmp::max(part2, temp);
    }

    // Starting at West:
    for i in 0..map.height {
        // reset visited.
        map.visited = vec![vec![]; map.chars.len()];
        let idx = map.to_index(0, i as i64).unwrap();
        let temp = propagate_light(&mut map, (idx, Direction::East));

        part2 = std::cmp::max(part2, temp);
    }

    println!("PART 2: {part2}")
}

fn propagate_light(map: &mut Map, start: (usize, Direction)) -> u64 {
    let mut nodes: Vec<(usize, Direction)> = vec![];

    // Add the start point, and start propagating.
    nodes.push(start);

    while nodes.len() != 0 {
        let mut next_nodes: Vec<(usize, Direction)> = vec![];

        for (idx, dir) in nodes {
            // Mark as visited, and add new points to the nodes lists.
            map.visited[idx].push(dir);
            match map.chars[idx] {
                '.' => match map.next(idx, dir) {
                    Some(new_idx) => next_nodes.push((new_idx, dir)),
                    None => {}
                },
                '/' => match dir {
                    Direction::North => match map.next(idx, Direction::East) {
                        Some(new_idx) => next_nodes.push((new_idx, Direction::East)),
                        None => {}
                    },
                    Direction::South => match map.next(idx, Direction::West) {
                        Some(new_idx) => next_nodes.push((new_idx, Direction::West)),
                        None => {}
                    },
                    Direction::East => match map.next(idx, Direction::North) {
                        Some(new_idx) => next_nodes.push((new_idx, Direction::North)),
                        None => {}
                    },
                    Direction::West => match map.next(idx, Direction::South) {
                        Some(new_idx) => next_nodes.push((new_idx, Direction::South)),
                        None => {}
                    },
                },
                '\\' => match dir {
                    Direction::North => match map.next(idx, Direction::West) {
                        Some(new_idx) => next_nodes.push((new_idx, Direction::West)),
                        None => {}
                    },
                    Direction::South => match map.next(idx, Direction::East) {
                        Some(new_idx) => next_nodes.push((new_idx, Direction::East)),
                        None => {}
                    },
                    Direction::East => match map.next(idx, Direction::South) {
                        Some(new_idx) => next_nodes.push((new_idx, Direction::South)),
                        None => {}
                    },
                    Direction::West => match map.next(idx, Direction::North) {
                        Some(new_idx) => next_nodes.push((new_idx, Direction::North)),
                        None => {}
                    },
                },
                '-' => match dir {
                    Direction::North => {
                        match map.next(idx, Direction::East) {
                            Some(new_idx) => next_nodes.push((new_idx, Direction::East)),
                            None => {}
                        }

                        match map.next(idx, Direction::West) {
                            Some(new_idx) => next_nodes.push((new_idx, Direction::West)),
                            None => {}
                        }
                    }
                    Direction::South => {
                        match map.next(idx, Direction::East) {
                            Some(new_idx) => next_nodes.push((new_idx, Direction::East)),
                            None => {}
                        }

                        match map.next(idx, Direction::West) {
                            Some(new_idx) => next_nodes.push((new_idx, Direction::West)),
                            None => {}
                        }
                    }

                    Direction::East => match map.next(idx, dir) {
                        Some(new_idx) => next_nodes.push((new_idx, dir)),
                        None => {}
                    },
                    Direction::West => match map.next(idx, dir) {
                        Some(new_idx) => next_nodes.push((new_idx, dir)),
                        None => {}
                    },
                },
                '|' => match dir {
                    Direction::East => {
                        match map.next(idx, Direction::North) {
                            Some(new_idx) => next_nodes.push((new_idx, Direction::North)),
                            None => {}
                        }

                        match map.next(idx, Direction::South) {
                            Some(new_idx) => next_nodes.push((new_idx, Direction::South)),
                            None => {}
                        }
                    }
                    Direction::West => {
                        match map.next(idx, Direction::North) {
                            Some(new_idx) => next_nodes.push((new_idx, Direction::North)),
                            None => {}
                        }

                        match map.next(idx, Direction::South) {
                            Some(new_idx) => next_nodes.push((new_idx, Direction::South)),
                            None => {}
                        }
                    }

                    Direction::North => match map.next(idx, dir) {
                        Some(new_idx) => next_nodes.push((new_idx, dir)),
                        None => {}
                    },
                    Direction::South => match map.next(idx, dir) {
                        Some(new_idx) => next_nodes.push((new_idx, dir)),
                        None => {}
                    },
                },
                _ => {}
            }
        }

        nodes = next_nodes;
    }

    return map
        .visited
        .iter()
        .map(|v| if v.len() > 0 { 1 } else { 0 })
        .sum();
}
