use std::{
    collections::HashMap,
    io::{stdin, Read},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map {
    width: usize,
    height: usize,
    chars: Vec<char>,
}

impl Map {
    fn _print(&self) {
        println!("Map {}x{}", self.width, self.height);
        for i in 0..self.chars.len() {
            if i != 0 && i % self.width as usize == 0 {
                println!();
            }
            print!("{}", self.chars[i]);
        }

        println!();
    }

    fn to_index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = y * self.width + x;

        return Some(index);
    }

    fn _to_point(&self, idx: usize) -> Option<(usize, usize)> {
        if idx >= self.chars.len() {
            return None;
        }

        let x = idx % self.width;
        let y = idx / self.width;

        return Some((x, y));
    }

    fn get_column(&self, i: usize) -> Vec<char> {
        if i >= self.width {
            return vec![];
        }

        let col: Vec<char> = self
            .chars
            .iter()
            .enumerate()
            .filter(|&(idx, _)| idx % self.width == i)
            .map(|(_, c)| *c)
            .collect();

        return col;
    }

    fn set_column(mut self, i: usize, new_col: &Vec<char>) -> Map {
        if i >= self.width {
            return self;
        }

        for y in 0..self.height {
            let idx: usize = self.to_index(i, y).unwrap();
            self.chars[idx] = new_col[y];
        }

        return self;
    }

    fn roll_clockwise(self) -> Map {
        let mut new_buff: Vec<char> = vec![];

        for x in 0..self.width {
            let mut col = self.get_column(x);
            col.reverse();
            new_buff.append(&mut col);
        }

        return Map {
            width: self.height,
            height: self.width,
            chars: new_buff,
        };
    }
}

fn main() {
    println!("2023 AoC - Day 14");

    // Gather the input map.
    let mut buff = String::new();
    stdin()
        .read_to_string(&mut buff)
        .expect("Can't read stdin!");

    let buff: Vec<&str> = buff.split("\n").filter(|line| !line.is_empty()).collect();
    let height = buff.len();
    let width = buff[0].len();

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

    // Part 1 ----------------------------------------------------------------
    let mut tilted_map = map.clone();
    let mut part1 = 0;

    // map.print();
    for col in 0..map.width {
        let rolled_col = tilt_column(map.get_column(col));
        tilted_map = tilted_map.set_column(col, &rolled_col);

        part1 += score_column(&rolled_col);
    }

    println!("PART 1: {part1}");

    // Part 2 .---------------------------------------------------------------
    //
    let mut cycle_cache: HashMap<Map, u64> = HashMap::new();
    let mut scores: Vec<u64> = vec![];
    let mut cycles = 0;
    let period;
    let offset;

    // Perform cycles and store the results unitl a repeated condition is found.
    let mut rollable_map = map.clone();
    loop {
        // Tilt North, West, South and East.
        for _ in 0..4 {
            // Tilt North, West, South and East.
            for col in 0..rollable_map.width {
                let tilted_column = tilt_column(rollable_map.get_column(col));
                rollable_map = rollable_map.set_column(col, &tilted_column);
            }

            rollable_map = rollable_map.roll_clockwise();
        }

        cycles += 1;
        let mut cycle_score = 0;
        for col in 0..rollable_map.width {
            cycle_score += score_column(&rollable_map.get_column(col));
        }

        // println!("Cycle {cycles}: {cycle_score}");
        // rollable_map._print();
        // println!("");

        // Check if this condition was already seen, or store its load if new.
        let key_map = rollable_map.clone();
        match cycle_cache.get(&key_map) {
            Some(val) => {
                period = cycles - val;
                offset = val;
                println!(
                    "Detected a Cycle! This board is the same as {}, so it repeats every {} rotations!",
                    val,
                    period
                );

                break;
            }
            None => {
                cycle_cache.insert(key_map, cycles);
                scores.push(cycle_score);
            }
        }
    }

    println!("Period: {period} | Offset: {offset}");

    let idx = (1000000000 - offset) % period;
    let idx = (idx + offset) as usize;
    println!("PART 2: {idx} {}", scores[idx - 1]);
}

fn score_column(column: &Vec<char>) -> u64 {
    let mut sum = 0;

    for y in 0..column.len() {
        if column[y] == 'O' {
            sum += column.len() - y;
        }
    }

    return sum as u64;
}

fn tilt_column(column: Vec<char>) -> Vec<char> {
    let mut empty_spot = 0;
    let mut new_col = column.clone();

    for i in 0..column.len() {
        match column[i] {
            '#' => {
                empty_spot = i + 1;
            }
            'O' => {
                new_col[i] = '.';
                new_col[empty_spot] = 'O';
                empty_spot += 1;
            }
            _ => {}
        }
    }

    return new_col;
}
