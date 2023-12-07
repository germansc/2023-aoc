use std::{cmp::Ordering, io::stdin};

#[derive(Debug, Clone)]
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

        let kind: String = get_kind_from_hand(&cards, 1);
        hands.push(Hand { cards, bid, kind });

        line.clear();
    }

    // dbg!(&hands);

    // Hands of the same kind must be sorted by comparing card values in order.
    hands.sort_by(|s1, s2| match s2.kind.cmp(&s1.kind) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => compare_hands(&s2.cards, &s1.cards, 1),
        Ordering::Greater => Ordering::Greater,
    });

    // dbg!(&hands);

    let mut part1: u64 = 0;
    let hand_count: u64 = hands.len() as u64;
    for (i, hand) in hands.clone().into_iter().enumerate() {
        part1 += hand.bid * (hand_count - i as u64);
    }

    println!("PART 1: {part1}");

    // --------------------------------------------------------------- PART 2
    //
    let mut hands_part2: Vec<Hand> = vec![];
    // Re calculate with rules of Part 2.
    for mut hand in hands {
        hand.kind = get_kind_from_hand(&hand.cards, 2);
        hands_part2.push(hand);
    }

    // Sort hands with rules of part 2.
    hands_part2.sort_by(|s1, s2| match s2.kind.cmp(&s1.kind) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => compare_hands(&s2.cards, &s1.cards, 2),
        Ordering::Greater => Ordering::Greater,
    });

    // dbg!(&hands_part2);

    let mut part2: u64 = 0;
    let hand_count: u64 = hands_part2.len() as u64;
    for (i, hand) in hands_part2.clone().into_iter().enumerate() {
        part2 += hand.bid * (hand_count - i as u64);
    }

    println!("PART 2: {part2}");
}

fn get_kind_from_hand(cards: &str, part: u64) -> String {
    let mut kind: Vec<usize> = vec![];
    let mut wildcards: u64 = 0;

    for (i, char) in cards.chars().enumerate() {
        if !cards[0..i].contains(char) {
            kind.push(cards.matches(char).count());
            if char == 'J' {
                wildcards = cards.matches(char).count() as u64;
            }
        }
    }

    kind.sort_by(|a, b| b.cmp(a));

    if part == 2 && wildcards != 0 && kind.len() != 1 {
        //Remove the fist value of wildcards.
        kind.remove(
            kind.iter()
                .position(|x: &usize| *x == wildcards as usize)
                .unwrap(),
        );

        // Add number of wildcards to the biggest count.
        kind[0] += wildcards as usize;
    }

    let kind = kind.iter().map(|u| u.to_string()).collect::<String>();

    return kind;
}

fn compare_hands(lhs: &str, rhs: &str, part: u64) -> Ordering {
    let values = if part == 1 {
        [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ]
    } else {
        [
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ]
    };

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
