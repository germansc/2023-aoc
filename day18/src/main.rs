use std::io::{stdin, Read};

#[derive(Debug, Clone, Copy)]
struct Vertex {
    x: i64,
    y: i64,
}

fn main() {
    println!("2023 AoC - Day 18");

    let mut buff = String::new();
    stdin()
        .read_to_string(&mut buff)
        .expect("Could not read stdin.");

    let mut last_vertex_1 = Vertex { x: 0, y: 0 };
    let mut last_vertex_2 = Vertex { x: 0, y: 0 };

    let mut vertex_list_1: Vec<Vertex> = vec![last_vertex_1.clone()];
    let mut vertex_list_2: Vec<Vertex> = vec![last_vertex_2.clone()];
    for line in buff.split("\n") {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let dir = line.split(" ").nth(0).unwrap();
        let length = line.split(" ").nth(1).unwrap().parse::<i64>().unwrap();

        // Part 1 Input
        let (x, y) = match dir {
            "R" => (last_vertex_1.x + length, last_vertex_1.y),
            "L" => (last_vertex_1.x - length, last_vertex_1.y),
            "U" => (last_vertex_1.x, last_vertex_1.y + length),
            "D" => (last_vertex_1.x, last_vertex_1.y - length),
            _ => {
                println!("Unexpected direction!");
                return;
            }
        };

        last_vertex_1 = Vertex { x, y };
        vertex_list_1.push(last_vertex_1.clone());

        // Part 2 Input
        let hex = line.split(" ").nth(2).unwrap();
        let len = i64::from_str_radix(&hex[2..hex.len() - 2], 16).unwrap();
        let dir = &hex[hex.len() - 2..hex.len() - 1];

        let (x, y) = match dir {
            "0" => (last_vertex_2.x + len, last_vertex_2.y),
            "1" => (last_vertex_2.x, last_vertex_2.y - len),
            "2" => (last_vertex_2.x - len, last_vertex_2.y),
            "3" => (last_vertex_2.x, last_vertex_2.y + len),
            _ => {
                println!("Unexpected direction!");
                return;
            }
        };

        last_vertex_2 = Vertex { x, y };
        vertex_list_2.push(last_vertex_2.clone());
    }

    // Part 1 ---------------------------------------------------------------
    // Calculate the area of a polygon described by vertices.
    let part1 = get_area(&vertex_list_1);

    println!("PART 1: {part1}");

    // Part 2 ---------------------------------------------------------------
    // Same thing with the second group of vertex.
    let part2 = get_area(&vertex_list_2);

    println!("PART 2: {part2}");
}

fn get_area(vertex_list: &[Vertex]) -> u64 {
    let mut area: i64 = 0;
    let mut peri: i64 = 0;

    for i in 0..vertex_list.len() {
        let p1 = vertex_list[i];
        let p2 = vertex_list[(i + 1) % vertex_list.len()];

        area += p1.x * p2.y - p1.y * p2.x;
        peri += (p1.x - p2.x + p1.y - p2.y).abs();
    }

    area = area.abs() / 2;

    // Use Pick's theorem to calculate the inside points.
    let inside = area - peri / 2 + 1;

    println!("Area: {area}");
    println!("Perimeter: {peri}");
    println!("Inside points: {inside}");
    return (peri + inside) as u64;
}
