use aoc::input_str;

#[derive(Debug, Clone, Copy)]
struct Range(u64, u64);

impl Range {
    fn contains(&self, n: u64) -> bool {
        self.0 <= n && n <= self.1
    }

    /// if two ranges overlap merge them into 1 larger range
    fn merge(&self, other: &Range) -> Option<Range> {
        // Check if ranges overlap or are adjacent
        if self.1 >= other.0.saturating_sub(1) && other.1 >= self.0.saturating_sub(1) {
            Some(Range(self.0.min(other.0), self.1.max(other.1)))
        } else {
            None
        }
    }

    fn len(&self) -> u64 {
        self.1 - self.0 + 1
    }
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();

    let mut ranges = vec![];
    // read while line is not empty
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        // get an inclusive range a-b
        let (a, b) = line.split_once('-').unwrap();
        let range = Range(a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap());
        ranges.push(range);
    }

    // read remaining lines as ingredients
    let mut ingredients = lines
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // sort ranges by start value
    ranges.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    ingredients.sort_unstable();

    let mut fresh = 0;
    let mut range_ptr = 0;

    for ingredient in ingredients {
        // Move range_ptr forward while current range's end is less than ingredient
        while range_ptr < ranges.len() && ranges[range_ptr].1 < ingredient {
            range_ptr += 1;
        }

        // Check if ingredient is in current range
        if range_ptr < ranges.len() && ranges[range_ptr].contains(ingredient) {
            fresh += 1;
        }
    }

    fresh
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut ranges = vec![];
    // read while line is not empty
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        // get an inclusive range a-b
        let (a, b) = line.split_once('-').unwrap();
        let range = Range(a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap());
        ranges.push(range);
    }

    // sort ranges by start value
    ranges.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let first = (0, *ranges.first().unwrap());

    // merge overlapping ranges using fold
    let merged_ranges =
        ranges
            .into_iter()
            .skip(1)
            .fold(first, |(acc, current), range| match current.merge(&range) {
                Some(merged) => (acc, merged),
                None => (acc + current.len(), range),
            });

    merged_ranges.0 + merged_ranges.1.len()
}

fn main() {
    let input = input_str!(2025, 5);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {:?}", start.elapsed());
}
