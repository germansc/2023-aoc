use std::{cmp::Ordering, io::stdin};

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u64,
    kind: String,
}

fn main() {
    println!("2023 AoC - Day 07");

    // Input collection.
    let mut hands: Vec<Hand> = vec![];

    let mut line: String = String::new();
    while stdin().read_line(&mut line).expect("Could not read stdin") != 0 {
        let cards: String = line.split(" ").nth(0).unwrap().to_string();
        let bid: u64 = line
            .split(" ")
            .nth(1)
            .unwrap()
            .trim()
            .parse()
            .expect("Could not parse bid");

        let kind: String = get_kind_from_hand(&cards);
        hands.push(Hand { cards, bid, kind });

        line.clear();
    }

    // dbg!(&hands);

    // Hands of the same kind must be sorted by comparing card values in order.
    hands.sort_by(|s1, s2| match s2.kind.cmp(&s1.kind) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => compare_hands(&s2.cards, &s1.cards),
        Ordering::Greater => Ordering::Greater,
    });

    // dbg!(&hands);

    let mut part1: u64 = 0;
    let hand_count: u64 = hands.len() as u64;
    for (i, hand) in hands.into_iter().enumerate() {
        part1 += hand.bid * (hand_count - i as u64);
    }

    println!("PART 1: {part1}");
}

fn get_kind_from_hand(cards: &str) -> String {
    let mut kind: Vec<usize> = vec![];

    for (i, char) in cards.chars().enumerate() {
        if !cards[0..i].contains(char) {
            kind.push(cards.matches(char).count());
        }
    }

    kind.sort_by(|a, b| b.cmp(a));
    let kind = kind.iter().map(|u| u.to_string()).collect::<String>();

    return kind;
}

fn compare_hands(lhs: &str, rhs: &str) -> Ordering {
    let values = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    for i in 0..lhs.len() {
        let val1 = values
            .iter()
            .position(|&c| c == lhs.chars().nth(i).unwrap())
            .unwrap();

        let val2 = values
            .iter()
            .position(|&c| c == rhs.chars().nth(i).unwrap())
            .unwrap();

        let result = match val1.cmp(&val2) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => continue,
            Ordering::Greater => Ordering::Greater,
        };

        return result;
    }

    return Ordering::Equal;
}
