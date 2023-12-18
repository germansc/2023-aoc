use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    io::{stdin, Read},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    None,
    North,
    South,
    East,
    West,
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    idx: usize,
    direction: Direction,
    heatloss: u64,
    consecutive_cells: u64,
}

// Implementation of ordering by least heatloss, for the priority queue.
impl Ord for Node {
    fn cmp(&self, rhs: &Node) -> Ordering {
        return rhs.heatloss.cmp(&self.heatloss);
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, rhs: &Node) -> Option<Ordering> {
        return Some(self.cmp(rhs));
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    chars: Vec<u64>,

    visited: Vec<(Direction, u64)>,
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

    fn _print(&self) {
        println!("Map {}x{}", self.width, self.height);
        for i in 0..self.chars.len() {
            if i != 0 && i % self.width as usize == 0 {
                println!();
            }

            let mut c: &str = &self.chars[i].to_string();
            if self.visited[i] != (Direction::None, 0) {
                c = match self.visited[i].0 {
                    Direction::None => "o",
                    Direction::North => "^",
                    Direction::South => "v",
                    Direction::East => ">",
                    Direction::West => "<",
                }
            }
            print!("{}", c);
        }
        println!("\n");
    }

    fn _print_heatloss(&self) {
        println!("Map {}x{}", self.width, self.height);

        for i in 0..self.chars.len() {
            if i != 0 && i % self.width as usize == 0 {
                println!();
            }

            print!("{:03}|", self.visited[i].1);
        }
        println!("\n");
    }
}

fn main() {
    println!("2023 AoC - Day 17");

    let mut buff = String::new();
    stdin()
        .read_to_string(&mut buff)
        .expect("Could not read stdin");

    let buff: Vec<&str> = buff.split("\n").filter(|line| !line.is_empty()).collect();
    let height = buff.len();
    let width = buff[0].len();

    let mut chars: Vec<u64> = vec![];
    for line in buff {
        for char in line.chars() {
            chars.push(char.to_digit(10).unwrap() as u64);
        }
    }

    let visited = vec![(Direction::None, 0); chars.len()];
    let mut map = Map {
        width,
        height,
        chars,
        visited,
    };

    // Part 1 ---------------------------------------------------------------
    //

    let part1 = propagate_carts(&mut map);
    println!("PART 1: {part1}");
}

fn propagate_carts(map: &mut Map) -> u64 {
    // Add the two initial cells to the pusher.
    use Direction::{East, North, South, West};

    // Each touple consists of (index, Traveling Direction, Previous count in this direction).
    let start1 = Node {
        idx: map.to_index(1, 0).unwrap(),
        direction: East,
        heatloss: map.chars[map.to_index(1, 0).unwrap()],
        consecutive_cells: 1,
    };

    let start2 = Node {
        idx: map.to_index(0, 1).unwrap(),
        direction: East,
        heatloss: map.chars[map.to_index(0, 1).unwrap()],
        consecutive_cells: 1,
    };

    // BinaryHeap, seems to be a priority based vector. This makes it so that the system always
    // extract the least heatloss cells from the neighboring node list.
    let mut nodes = BinaryHeap::new();
    nodes.push(start1);
    nodes.push(start2);

    // Already computed Hashset -> idx, Direction, Consecutive cells in that direction.
    let mut cache: HashSet<(usize, Direction, u64)> = HashSet::new();
    let mut count = 0;

    // Get the next node -> priority lesser heatloss.
    while let Some(node) = nodes.pop() {
        let mut next_nodes: Vec<Node> = vec![];
        count += 1;

        let (x, y) = map.to_point(node.idx).unwrap();

        // println!(
        //     "{count}: Processing cell {},{}: Heat lost {} record for cell",
        //     x, y, node.heatloss
        // );

        // Log the current heatloss record and direction.
        if map.visited[node.idx].1 == 0 || map.visited[node.idx].1 > node.heatloss {
            // println!("> New record for cell {} -> {}", node.idx, node.heatloss);
            map.visited[node.idx] = (node.direction, node.heatloss);
        }

        // If the node corresponds to the exit point, by priority queue this is the least
        // possible heatloss for this cell.
        if node.idx == map.chars.len() - 1 {
            // println!("> Reached the last cell");
            return node.heatloss;
        }

        // Otherwise, get the neighboring cells and add them to the queue.

        // Add the next posible cells to the node_list.
        match node.direction {
            North => {
                // Always add 90 degree turn cells.
                match map.to_index(x - 1, y) {
                    Some(val) => next_nodes.push(Node {
                        idx: val,
                        direction: West,
                        heatloss: node.heatloss + map.chars[val],
                        consecutive_cells: 1,
                    }),
                    None => {}
                }

                // Always add 90 degree turn cells.
                match map.to_index(x + 1, y) {
                    Some(val) => next_nodes.push(Node {
                        idx: val,
                        direction: East,
                        heatloss: node.heatloss + map.chars[val],
                        consecutive_cells: 1,
                    }),
                    None => {}
                }

                // If can still go straight, add the forward cell.
                if node.consecutive_cells < 3 {
                    match map.to_index(x, y - 1) {
                        Some(val) => next_nodes.push(Node {
                            idx: val,
                            direction: North,
                            heatloss: node.heatloss + map.chars[val],
                            consecutive_cells: node.consecutive_cells + 1,
                        }),
                        None => {}
                    }
                }
            }
            South => {
                // Always add 90 degree turn cells.
                match map.to_index(x - 1, y) {
                    Some(val) => next_nodes.push(Node {
                        idx: val,
                        direction: West,
                        heatloss: node.heatloss + map.chars[val],
                        consecutive_cells: 1,
                    }),
                    None => {}
                }

                // Always add 90 degree turn cells.
                match map.to_index(x + 1, y) {
                    Some(val) => next_nodes.push(Node {
                        idx: val,
                        direction: East,
                        heatloss: node.heatloss + map.chars[val],
                        consecutive_cells: 1,
                    }),
                    None => {}
                }

                // If can still go straight, add the forward cell.
                if node.consecutive_cells < 3 {
                    match map.to_index(x, y + 1) {
                        Some(val) => next_nodes.push(Node {
                            idx: val,
                            direction: South,
                            heatloss: node.heatloss + map.chars[val],
                            consecutive_cells: node.consecutive_cells + 1,
                        }),
                        None => {}
                    }
                }
            }
            East => {
                // Always add 90 degree turn cells.
                match map.to_index(x, y - 1) {
                    Some(val) => next_nodes.push(Node {
                        idx: val,
                        direction: North,
                        heatloss: node.heatloss + map.chars[val],
                        consecutive_cells: 1,
                    }),
                    None => {}
                }

                // Always add 90 degree turn cells.
                match map.to_index(x, y + 1) {
                    Some(val) => next_nodes.push(Node {
                        idx: val,
                        direction: South,
                        heatloss: node.heatloss + map.chars[val],
                        consecutive_cells: 1,
                    }),
                    None => {}
                }

                // If can still go straight, add the forward cell.
                if node.consecutive_cells < 3 {
                    match map.to_index(x + 1, y) {
                        Some(val) => next_nodes.push(Node {
                            idx: val,
                            direction: East,
                            heatloss: node.heatloss + map.chars[val],
                            consecutive_cells: node.consecutive_cells + 1,
                        }),
                        None => {}
                    }
                }
            }
            West => {
                // Always add 90 degree turn cells.
                match map.to_index(x, y - 1) {
                    Some(val) => next_nodes.push(Node {
                        idx: val,
                        direction: North,
                        heatloss: node.heatloss + map.chars[val],
                        consecutive_cells: 1,
                    }),
                    None => {}
                }

                // Always add 90 degree turn cells.
                match map.to_index(x, y + 1) {
                    Some(val) => next_nodes.push(Node {
                        idx: val,
                        direction: South,
                        heatloss: node.heatloss + map.chars[val],
                        consecutive_cells: 1,
                    }),
                    None => {}
                }

                // If can still go straight, add the forward cell.
                if node.consecutive_cells < 3 {
                    match map.to_index(x - 1, y) {
                        Some(val) => next_nodes.push(Node {
                            idx: val,
                            direction: West,
                            heatloss: node.heatloss + map.chars[val],
                            consecutive_cells: node.consecutive_cells + 1,
                        }),
                        None => {}
                    }
                }
            }
            _ => {}
        }

        // For each Node generated, insert them to the binary heap. The if block with the cache
        // avoids inserting cells that were already computed.
        for next_node in next_nodes {
            if cache.insert((
                next_node.idx,
                next_node.direction,
                next_node.consecutive_cells,
            )) {
                // let (x, y) = map.to_point(next_node.idx).unwrap();
                // println!(
                //     "> Adding ({x},{y} | {} | {}) to the nodes lists",
                //     next_node.heatloss, next_node.consecutive_cells
                // );
                nodes.push(next_node);
            }
        }
    }

    // Return the minimum heatloss recorded at bottom right.
    println!("Could not reach the last cell? |-> Iterations: {count}");
    let i = map.chars.len() - 1;
    return map.visited[i].1;
}

