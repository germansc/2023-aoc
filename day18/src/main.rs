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

    let mut last_vertex = Vertex { x: 0, y: 0 };

    let mut vertex_list: Vec<Vertex> = vec![last_vertex.clone()];
    for line in buff.split("\n") {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let dir = line.split(" ").nth(0).unwrap();
        let length = line.split(" ").nth(1).unwrap().parse::<i64>().unwrap();

        let (x, y) = match dir {
            "R" => (last_vertex.x + length, last_vertex.y),
            "L" => (last_vertex.x - length, last_vertex.y),
            "U" => (last_vertex.x, last_vertex.y + length),
            "D" => (last_vertex.x, last_vertex.y - length),
            _ => {
                println!("Unexpected direction!");
                return;
            }
        };

        last_vertex = Vertex { x, y };

        vertex_list.push(last_vertex.clone());
    }

    // Part 1 ---------------------------------------------------------------
    // Calculate the area of a polygon described by vertices.
    let part1 = get_area(&vertex_list);

    println!("PART 1: {part1}");
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
