use std::ops::Range;

use aoc::input_str;

#[derive(Debug, Clone)]
struct MapEntry {
    input: Range<u64>,
    output: Range<u64>,
}

impl MapEntry {
    fn new(output_start: u64, input_start: u64, length: u64) -> Self {
        Self {
            input: input_start..input_start + length,
            output: output_start..output_start + length,
        }
    }

    fn contains(&self, value: u64) -> bool {
        self.input.contains(&value)
    }

    fn process(&self, value: u64) -> Option<u64> {
        if self.contains(value) {
            Some(value - self.input.start + self.output.start)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    fn new() -> Self {
        Self { entries: vec![] }
    }

    // Maps a single value through the map
    fn process(&self, value: u64) -> u64 {
        self.entries
            .iter()
            .find_map(|entry| entry.process(value))
            .unwrap_or(value)
    }

    // Maps a range of values through the map
    //
    // - The output range might not be continuous
    // - The input range is way too large to iterate over
    // - Parts of the range are going to be mapped to the same value
    fn process_range(&self, range: Range<u64>) -> Vec<Range<u64>> {
        let mut ranges = vec![];

        let mut start = range.start;
        while start != range.end {
            // Find the first entry that contains the start value
            let entry = self.entries.iter().find(|entry| entry.contains(start));

            match entry {
                Some(entry) => {
                    // Find the end of the range
                    let end = entry.input.end.min(range.end);

                    // Map the range
                    let output_start = entry.output.start + (start - entry.input.start);
                    let output_end = entry.output.start + (end - entry.input.start);

                    // Add the range to the list
                    ranges.push(output_start..output_end);

                    // Move the start to the end of the range
                    start = end;
                }
                None => {
                    // No entry contains the start value, so we need to find the next closest entry
                    let next_entry = self
                        .entries
                        .iter()
                        .filter(|entry| entry.input.start > start)
                        .min_by_key(|entry| entry.input.start);

                    match next_entry {
                        Some(next) => {
                            // Map until the next entry
                            let end = next.input.start.min(range.end);
                            ranges.push(start..end);
                            start = end;
                        }
                        None => {
                            // No more entries, so map until the end of the range
                            ranges.push(start..range.end);
                            start = range.end;
                        }
                    }
                }
            }
        }

        ranges
    }
}

impl Extend<MapEntry> for Map {
    fn extend<T: IntoIterator<Item = MapEntry>>(&mut self, iter: T) {
        self.entries.extend(iter);
    }
}

impl FromIterator<MapEntry> for Map {
    fn from_iter<T: IntoIterator<Item = MapEntry>>(iter: T) -> Self {
        let mut map = Self::new();
        map.extend(iter);
        map
    }
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    // Runs the seed through all the maps
    fn part1(&self, seed: u64) -> u64 {
        let mut value = seed;
        for map in &self.maps {
            value = map.process(value);
        }
        value
    }

    // Processes ranges through all the maps
    fn part2(&self, range: Range<u64>) -> Vec<Range<u64>> {
        let mut ranges = vec![range];
        for map in &self.maps {
            ranges = ranges
                .into_iter()
                .flat_map(|range| map.process_range(range))
                .collect();
        }

        ranges
    }
}

fn parse_map<'a>(mut lines: impl Iterator<Item = &'a str>) -> Map {
    lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|parts| MapEntry::new(parts[0], parts[1], parts[2]))
        .collect::<Map>()
}

fn parse_almanac(input: &str) -> Almanac {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse().unwrap())
        .collect::<Vec<_>>();

    lines.next();

    let mut maps = vec![];
    loop {
        if lines.next().is_none() {
            break;
        }

        maps.push(parse_map(&mut lines));
    }

    Almanac { seeds, maps }
}

fn part1(almanac: &Almanac) -> u64 {
    // Find the minimum seed
    almanac
        .seeds
        .iter()
        .map(|seed| almanac.part1(*seed))
        .min()
        .unwrap()
}

fn part2(almanac: &Almanac) -> u64 {
    // Pairs of seeds are ranges
    let ranges = almanac
        .seeds
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]));

    // Process the ranges through all the maps
    let ranges = ranges.flat_map(|range| almanac.part2(range));

    // Find the minimum value in the ranges
    ranges.map(|range| range.start).min().unwrap()
}

fn main() {
    let input = input_str!(2023, 5);
    let almanac = parse_almanac(input);

    println!("Part 1: {}", part1(&almanac));
    println!("Part 2: {}", part2(&almanac));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4";
        let almanac = parse_almanac(input);

        assert_eq!(almanac.seeds, [79, 14, 55, 13]);
        assert_eq!(almanac.maps.len(), 7);

        assert_eq!(almanac.part1(79), 82);
        assert_eq!(almanac.part1(14), 43);
        assert_eq!(almanac.part1(55), 86);
        assert_eq!(almanac.part1(13), 35);

        assert_eq!(part2(&almanac), 46);
    }
}
