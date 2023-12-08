use crate::HandType;

//  A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J
const JOKER: u8 = 0;

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

// Ignores the jokers
fn decision_tree(cards: &[u8]) -> HandType {
    // Builds a 4 level decision tree that is capable of determining the hand type
    // by making a single pass through the cards hands
    debug_assert_eq!(cards.len(), 5);

    todo!()
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
