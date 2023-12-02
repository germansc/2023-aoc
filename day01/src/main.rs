use std::io;

fn main() {
    println!("2023 AoC - Day 1");

    let mut sum: i32 = 0;
    let strings: Vec<_> = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    loop {
        let mut str = String::new();
        let bytes: usize = io::stdin().read_line(&mut str).expect("Cant read?!");

        // Check for EoF.
        if bytes == 0 {
            break;
        }

        print!("{}", str.trim());

        let mut first: i32 = -1;
        let mut last: i32 = -1;
        let mut index: usize = 0;

        while index < str.trim().len() {
            // Check if this char is a string.
            let c: char = str.as_bytes()[index] as char;
            if char::is_digit(c, 10) {
                if first < 0 {
                    first = c.to_digit(10).unwrap() as i32;
                    last = first;
                } else {
                    last = c.to_digit(10).unwrap() as i32
                }
            } else {
                // Try to match this substring to a number string.
                for (i, s) in strings.iter().enumerate() {
                    let e: usize = std::cmp::min(s.len(), str.len() - index);
                    if *s == &str[index..index + e] {
                        if first < 0 {
                            first = i as i32;
                            last = first;
                        } else {
                            last = i as i32;
                        }

                        // Skip most of the matched string, but not all of it. Some overlap might
                        // happen between "string numbers", since I want to find the last number, I
                        // want to consider all of them. Watching at the strings, 1 character
                        // might overlap on consecutive numbers, so, i will skip len - 2, since
                        // before the next iteration the index will be increased in one, leaving me
                        // on the last character of this match (that could be the start of a new
                        // match).
                        index += s.len() - 2;
                        break;
                    }
                }
            }

            // Next item.
            index += 1;
        }

        println!(" |-> Found: {first}{last}");
        sum += 10 * first + last;
    }

    println!("\nThe final sum is: \n{sum}");
}
