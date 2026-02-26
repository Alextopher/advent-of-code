use aoc::input_str;

// A number is 'repeating' if it is form like 55, 123123, etc
fn part1_is_repeating(n: i64) -> bool {
    // log base 10 tells us the number of digits
    let digits = (n as f64).log10().floor() as u32 + 1;

    // get half the digits
    let top = n / 10_i64.pow(digits / 2);
    let bottom = n % 10_i64.pow(digits / 2);

    top == bottom
}

fn part1(input: &str) -> i64 {
    input
        .trim()
        .split(',')
        .flat_map(|s| {
            let parts = s.split('-').collect::<Vec<&str>>();
            let start = parts[0].parse().unwrap();
            let end = parts[1].parse().unwrap();
            start..=end
        })
        .filter(|&n| part1_is_repeating(n))
        .sum()
}

// now numbers with multiple repeats count (12341234, 123123123, 1212121212)
fn part2_is_repeating(digits: &mut Vec<i64>, mut n: i64) -> bool {
    digits.clear();

    // convert the number to a vec of digits
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    // check for sequences of length 1, 2, 3 .. n/2
    for chunk_size in 1..=digits.len() / 2 {
        // verify the chunk size factors into len
        if digits.len().is_multiple_of(chunk_size) {
            let mut chunks = digits.chunks(chunk_size);
            let first = chunks.next().unwrap();
            if chunks.all(|w| w == first) {
                return true;
            }
        }
    }
    false
}

fn part2(input: &str) -> i64 {
    let mut buffer = Vec::new();

    input
        .trim()
        .split(',')
        .flat_map(|s| {
            let parts = s.split('-').collect::<Vec<&str>>();
            let start = parts[0].parse().unwrap();
            let end = parts[1].parse().unwrap();
            start..=end
        })
        .filter(|&n| part2_is_repeating(&mut buffer, n))
        .sum()
}

fn main() {
    let input = input_str!(2025, 2);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_repeating() {
        assert!(part1_is_repeating(55));
        assert!(part1_is_repeating(123123));
        assert!(!part1_is_repeating(123456));

        let mut buffer = Vec::new();
        assert!(part2_is_repeating(&mut buffer, 12341234));
        assert!(part2_is_repeating(&mut buffer, 123123123));
        assert!(part2_is_repeating(&mut buffer, 1212121212));
    }

    #[test]
    fn test_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part1(input), 1227775554);
    }
}
