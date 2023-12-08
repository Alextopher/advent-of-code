use crate::HandType;

//  A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
fn rank(c: u8) -> u8 {
    match c {
        b'2'..=b'9' => c - b'2',
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => unsafe { std::hint::unreachable_unchecked() },
    }
}

// Ignores the jokers
fn decision_tree(cards: &[u8]) -> HandType {
    // Builds a 4 level decision tree that is capable of determining the hand type
    // by making a single pass through the cards hands
    debug_assert_eq!(cards.len(), 5);

    // 0
    let zero = cards[0];
    if cards[1] == zero {
        // 00
        if cards[2] == zero {
            // 000
            if cards[3] == zero {
                // 0000
                if cards[4] == zero {
                    // 00000
                    HandType::FiveOfAKind
                } else {
                    // 00001
                    HandType::FourOfAKind
                }
            } else {
                // 0001
                let one = cards[3];
                if cards[4] == zero {
                    // 00010
                    HandType::FourOfAKind
                } else if cards[4] == one {
                    // 00011
                    HandType::FullHouse
                } else {
                    // 00012
                    HandType::ThreeOfAKind
                }
            }
        } else {
            // 001
            let one = cards[2];
            if cards[3] == zero {
                // 0010
                if cards[4] == zero {
                    // 00100
                    HandType::FourOfAKind
                } else if cards[4] == one {
                    // 00101
                    HandType::FullHouse
                } else {
                    // 00102
                    HandType::ThreeOfAKind
                }
            } else if cards[3] == one {
                // 0011
                if cards[4] == zero || cards[4] == one {
                    // 00110 | 00111
                    HandType::FullHouse
                } else {
                    // 00112
                    HandType::TwoPair
                }
            } else {
                // 0012
                let two = cards[3];
                if cards[4] == zero {
                    // 00120
                    HandType::ThreeOfAKind
                } else if cards[4] == one || cards[4] == two {
                    // 00121 | 00122
                    HandType::TwoPair
                } else {
                    // 00123
                    HandType::OnePair
                }
            }
        }
    } else {
        // 01
        let one = cards[1];
        if cards[2] == zero {
            // 010
            if cards[3] == zero {
                // 0100
                if cards[4] == zero {
                    // 01000
                    HandType::FourOfAKind
                } else if cards[4] == one {
                    // 01001
                    HandType::FullHouse
                } else {
                    // 01002
                    HandType::ThreeOfAKind
                }
            } else if cards[3] == one {
                // 0101
                if cards[4] == zero || cards[4] == one {
                    // 01010 | 01011
                    HandType::FullHouse
                } else {
                    // 01012
                    HandType::TwoPair
                }
            } else {
                // 0102
                let two = cards[3];
                if cards[4] == zero {
                    // 01020
                    HandType::ThreeOfAKind
                } else if cards[4] == one || cards[4] == two {
                    // 01021 | 01022
                    HandType::TwoPair
                } else {
                    // 01023
                    HandType::OnePair
                }
            }
        } else if cards[2] == one {
            // 011
            if cards[3] == zero {
                // 0110
                if cards[4] == zero || cards[4] == one {
                    // 01100 | 01101
                    HandType::FullHouse
                } else {
                    // 01102
                    HandType::TwoPair
                }
            } else if cards[3] == one {
                // 0111
                if cards[4] == zero {
                    // 01110
                    HandType::FullHouse
                } else if cards[4] == one {
                    // 01111
                    HandType::FourOfAKind
                } else {
                    // 01112
                    HandType::ThreeOfAKind
                }
            } else {
                // 0112
                let two = cards[3];
                if cards[4] == zero {
                    // 01120
                    HandType::TwoPair
                } else if cards[4] == one {
                    // 01121
                    HandType::ThreeOfAKind
                } else if cards[4] == two {
                    // 01122
                    HandType::TwoPair
                } else {
                    // 01123
                    HandType::OnePair
                }
            }
        } else {
            // 012
            let two = cards[2];
            if cards[3] == zero {
                // 0120
                if cards[4] == zero {
                    // 01200
                    HandType::ThreeOfAKind
                } else if cards[4] == one || cards[4] == two {
                    // 01201 | 01202
                    HandType::TwoPair
                } else {
                    // 01203
                    HandType::OnePair
                }
            } else if cards[3] == one {
                // 0121
                if cards[4] == zero {
                    // 01210
                    HandType::TwoPair
                } else if cards[4] == one {
                    // 01211
                    HandType::ThreeOfAKind
                } else if cards[4] == two {
                    // 01212
                    HandType::TwoPair
                } else {
                    // 01213
                    HandType::OnePair
                }
            } else if cards[3] == two {
                // 0122
                if cards[4] == zero || cards[4] == one {
                    // 01220 | 01221
                    HandType::TwoPair
                } else if cards[4] == two {
                    // 01222
                    HandType::ThreeOfAKind
                } else {
                    // 01223
                    HandType::OnePair
                }
            } else {
                // 0123
                let three = cards[3];
                if cards[4] == zero || cards[4] == one || cards[4] == two || cards[4] == three {
                    // 01230 | 01231 | 01232 | 01233
                    HandType::OnePair
                } else {
                    // 01234
                    HandType::HighCard
                }
            }
        }
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

pub fn part1(input: &str) -> u32 {
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
