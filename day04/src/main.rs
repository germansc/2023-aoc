#[derive(Debug)]
struct Card {
    _id: u32,
    _w_num: Vec<u32>,
    _num: Vec<u32>,
    points: u32,
    _subset: Vec<u32>,
    copies: u32,
}

fn parse_line(line: &str) -> Card {
    let s = line.trim().find(|c: char| c.is_digit(10)).unwrap();
    let e = line.trim().find(':').unwrap();
    let card_num = line[s..e].parse().expect("Could not parse ID");

    // Gather each set of numbers.
    let winners: Vec<u32> = line[e + 1..]
        .split('|')
        .nth(0 as usize)
        .unwrap()
        .trim()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();

    let numbers: Vec<u32> = line[e + 1..]
        .split('|')
        .nth(1 as usize)
        .unwrap()
        .trim()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();

    let temp: Vec<u32> = winners
        .clone()
        .into_iter()
        .filter(|s| numbers.contains(s))
        .collect();

    let mut score: u32 = 0;
    if !temp.is_empty() {
        score = 1 << temp.len() - 1;
    }

    return Card {
        _id: card_num,
        _w_num: winners,
        _num: numbers,
        points: score,
        _subset: temp,
        copies: 1,
    };
}

fn main() {
    println!("2023 AoC - Day 4");

    let mut line = String::new();
    let mut cards: Vec<Card> = Vec::new();

    while std::io::stdin()
        .read_line(&mut line)
        .expect("Could not read input!")
        != 0
    {
        cards.push(parse_line(&line));
        line.clear();
    }

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    for i in 0..cards.len() {
        part1 += cards[i].points;

        for n in 1..=cards[i]._subset.len() {
            cards[i + n].copies += cards[i].copies;
        }

        part2 += cards[i].copies;
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
