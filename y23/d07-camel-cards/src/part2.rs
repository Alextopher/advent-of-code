use crate::HandType;

//  A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J
fn rank(c: u8) -> u8 {
    match c {
        b'J' => 0,
        b'2'..=b'9' => c - b'1',
        b'T' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => unsafe { std::hint::unreachable_unchecked() },
    }
}

fn one_joker(cards: [u8; 4]) -> HandType {
    let zero = cards[0];
    // J0
    if cards[1] == zero {
        // J00
        if cards[2] == zero {
            // J000
            if cards[3] == zero {
                // J0000
                HandType::FiveOfAKind
            } else {
                // J0001
                HandType::FourOfAKind
            }
        } else {
            // J001
            let one = cards[2];
            if cards[3] == zero {
                // J0010
                HandType::FourOfAKind
            } else if cards[3] == one {
                // J0011
                HandType::FullHouse
            } else {
                // J0012
                HandType::ThreeOfAKind
            }
        }
    } else {
        // J01
        let one = cards[1];
        if cards[2] == zero {
            // J010
            if cards[3] == zero {
                // J0100
                HandType::FourOfAKind
            } else if cards[3] == one {
                // J0101
                HandType::FullHouse
            } else {
                // J0102
                HandType::ThreeOfAKind
            }
        } else if cards[2] == one {
            // J011
            if cards[3] == zero {
                // J0110
                HandType::FullHouse
            } else if cards[3] == one {
                // J0111
                HandType::FourOfAKind
            } else {
                // J0112
                HandType::ThreeOfAKind
            }
        } else {
            // J012
            let two = cards[2];
            if cards[3] == zero || cards[3] == one || cards[3] == two {
                // J0120 | J0121 | J0122
                HandType::ThreeOfAKind
            } else {
                // J0123
                HandType::OnePair
            }
        }
    }
}

fn two_jokers(cards: [u8; 3]) -> HandType {
    let zero = cards[0];

    // JJ0
    if cards[1] == zero {
        // JJ00
        if cards[2] == zero {
            // JJ000
            HandType::FiveOfAKind
        } else {
            // JJ001
            HandType::FourOfAKind
        }
    } else {
        // JJ01
        let one = cards[1];
        if cards[2] == zero || cards[2] == one {
            // JJ010 | JJ011
            HandType::FourOfAKind
        } else {
            // JJ012
            HandType::ThreeOfAKind
        }
    }
}

fn three_jokers(a: u8, b: u8) -> HandType {
    if a == b {
        // JJJ00
        HandType::FiveOfAKind
    } else {
        // JJJ01
        HandType::FourOfAKind
    }
}

fn decision_tree(cards: &[u8]) -> HandType {
    debug_assert_eq!(cards.len(), 5);

    // Count the number of Jokers
    let jokers = cards.iter().filter(|&&c| c == b'J').count();
    if jokers == 5 || jokers == 4 {
        HandType::FiveOfAKind
    } else if jokers == 3 {
        // get a, b
        let mut iter = cards.iter().filter(|&&c| c != b'J');
        let a = *iter.next().unwrap();
        let b = *iter.next().unwrap();

        three_jokers(a, b)
    } else if jokers == 2 {
        // get a, b, c
        let mut iter = cards.iter().filter(|&&c| c != b'J');
        let a = *iter.next().unwrap();
        let b = *iter.next().unwrap();
        let c = *iter.next().unwrap();
        two_jokers([a, b, c])
    } else if jokers == 1 {
        // get a, b, c, d
        let mut iter = cards.iter().filter(|&&c| c != b'J');
        let a = *iter.next().unwrap();
        let b = *iter.next().unwrap();
        let c = *iter.next().unwrap();
        let d = *iter.next().unwrap();
        one_joker([a, b, c, d])
    } else if jokers == 0 {
        crate::part1::decision_tree(cards)
    } else {
        unreachable!()
    }
}

fn base13(cards: &str) -> u32 {
    debug_assert_eq!(cards.len(), 5);

    let mut result = 0;
    for card in cards.bytes().map(rank) {
        result *= 13;
        result += card as u32;
    }
    result
}

fn parse_line(line: &str) -> (HandType, u32, u32) {
    let cards = &line[..5];

    let hand = decision_tree(cards.as_bytes());
    let base13 = base13(cards);
    let bid = line[6..].parse().unwrap();

    (hand, base13, bid)
}

fn parse(input: &str) -> Vec<(HandType, u32, u32)> {
    input.lines().map(parse_line).collect::<Vec<_>>()
}

pub fn part2(input: &str) -> u32 {
    let time = std::time::Instant::now();
    let mut data = parse(input);
    let parsed = time.elapsed();

    let time = std::time::Instant::now();
    data.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    let sorted = time.elapsed();

    let time = std::time::Instant::now();
    let answer = data
        .iter()
        .zip(1..)
        .map(|((_, _, bid), i)| bid * i)
        .sum::<u32>();
    let summed = time.elapsed();

    println!("parsed: {:?}", parsed);
    println!("sorted: {:?}", sorted);
    println!("summed: {:?}", summed);

    answer
}
