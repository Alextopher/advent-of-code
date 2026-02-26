use std::collections::HashSet;
use std::sync::Mutex;

use aoc::input_str;
use rayon::prelude::*;

#[derive(Debug)]
struct EdgeSegments {
    // Vertical segments: (x, (y_min, y_max))
    vertical: Vec<(u64, (u64, u64))>,
    // Horizontal segments: (y, (x_min, x_max))
    horizontal: Vec<(u64, (u64, u64))>,
}

struct Polygon {
    points: HashSet<(u64, u64)>,
    bounds: (u64, u64, u64, u64),
    edges: EdgeSegments,
}

fn raycast(polygon: &Polygon, x: u64, y: u64) -> bool {
    let mut intersections = 0;

    // check if the point is within the bounds
    if x < polygon.bounds.0 || x > polygon.bounds.2 || y < polygon.bounds.1 || y > polygon.bounds.3
    {
        return false;
    }

    // calculate distances to each bound
    let dist_left = x - polygon.bounds.0;
    let dist_right = polygon.bounds.2 - x;
    let dist_top = y - polygon.bounds.1;
    let dist_bottom = polygon.bounds.3 - y;

    // find the minimum distance and shoot ray in that direction
    let min_dist = dist_left.min(dist_right).min(dist_top).min(dist_bottom);

    if min_dist == dist_left {
        // shoot ray left - check vertical segments with x < current x
        for &(seg_x, (y_min, y_max)) in polygon.edges.vertical.iter() {
            // Use half-open interval: include y_min but exclude y_max to handle vertices consistently
            if seg_x < x && y >= y_min && y < y_max {
                intersections += 1;
            }
        }
    } else if min_dist == dist_right {
        // shoot ray right - check vertical segments with x > current x
        for &(seg_x, (y_min, y_max)) in polygon.edges.vertical.iter() {
            // Use half-open interval: include y_min but exclude y_max to handle vertices consistently
            if seg_x > x && y >= y_min && y < y_max {
                intersections += 1;
            }
        }
    } else if min_dist == dist_top {
        // shoot ray up - check horizontal segments with y < current y
        for &(seg_y, (x_min, x_max)) in polygon.edges.horizontal.iter() {
            // Use half-open interval: include x_min but exclude x_max to handle vertices consistently
            if seg_y < y && x >= x_min && x < x_max {
                intersections += 1;
            }
        }
    } else {
        // shoot ray down - check horizontal segments with y > current y
        for &(seg_y, (x_min, x_max)) in polygon.edges.horizontal.iter() {
            // Use half-open interval: include x_min but exclude x_max to handle vertices consistently
            if seg_y > y && x >= x_min && x < x_max {
                intersections += 1;
            }
        }
    }

    intersections % 2 == 1
}

fn part2(input: &str) -> u64 {
    let points: Vec<(u64, u64)> = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    // every pair of neighboring points either share a row or column
    // the first and last points are neighbors but were not listed

    // build a 'polygon' HashSet<(u64, u64)> and then fill it in
    let mut set = HashSet::new();
    let mut vertical_segments = Vec::new();
    let mut horizontal_segments = Vec::new();

    let (mut min_x, mut min_y, mut max_x, mut max_y) = (u64::MAX, u64::MAX, 0, 0);

    // Process all adjacent pairs, including the wrap from last to first
    for i in 0..points.len() {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % points.len()];

        if x1 == x2 {
            // Vertical segment
            vertical_segments.push((x1, (y1.min(y2), y1.max(y2))));
            for y in y1.min(y2)..=y1.max(y2) {
                set.insert((x1, y));
            }
        } else {
            // Horizontal segment
            horizontal_segments.push((y1, (x1.min(x2), x1.max(x2))));
            for x in x1.min(x2)..=x1.max(x2) {
                set.insert((x, y1));
            }
        }

        // track the bounds
        min_x = min_x.min(x1);
        min_y = min_y.min(y1);
        max_x = max_x.max(x1);
        max_y = max_y.max(y1);
    }

    // Sort segments for potential optimization (though not strictly necessary for correctness)
    vertical_segments.sort_by_key(|&(x, _)| x);
    horizontal_segments.sort_by_key(|&(y, _)| y);

    let edges = EdgeSegments {
        vertical: vertical_segments,
        horizontal: horizontal_segments,
    };

    let polygon = Polygon {
        points: set,
        bounds: (min_x, min_y, max_x, max_y),
        edges,
    };

    // Generate all possible rectangles and sort by area (descending)
    let mut rectangles = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let area = (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1);
            rectangles.push((area, x1, y1, x2, y2));
        }
    }
    rectangles.sort_by(|a, b| b.0.cmp(&a.0)); // Sort by area descending

    // Track points that have failed validation to avoid checking rectangles containing them
    let failed_points = Mutex::new(HashSet::new());

    // Find the first (largest) rectangle that fits entirely within the polygon
    for (area, x1, y1, x2, y2) in rectangles {
        // First check if any failed points are inside this rectangle
        let rect_min_x = x1.min(x2);
        let rect_max_x = x1.max(x2);
        let rect_min_y = y1.min(y2);
        let rect_max_y = y1.max(y2);

        let skip = {
            let failed = failed_points.lock().unwrap();
            failed.iter().any(|&(fx, fy)| {
                fx >= rect_min_x && fx <= rect_max_x && fy >= rect_min_y && fy <= rect_max_y
            })
        };

        if skip {
            continue;
        }

        // Check perimeter points in parallel
        let points_to_check: Vec<(u64, u64)> = (x1.min(x2)..=x1.max(x2))
            .flat_map(|x| [(x, y1), (x, y2)])
            .chain((y1.min(y2)..=y1.max(y2)).flat_map(|y| [(x1, y), (x2, y)]))
            .collect();

        let invalid_points: Vec<(u64, u64)> = points_to_check
            .par_iter()
            .filter_map(|&(x, y)| {
                if !polygon.points.contains(&(x, y)) && !raycast(&polygon, x, y) {
                    Some((x, y))
                } else {
                    None
                }
            })
            .collect();

        if invalid_points.is_empty() {
            return area;
        } else {
            // Add invalid points to the failed set
            let mut failed = failed_points.lock().unwrap();
            for point in invalid_points {
                failed.insert(point);
            }
        }
    }

    0 // No valid rectangle found
}

fn main() {
    let input = input_str!(2025, 9);

    let start = std::time::Instant::now();
    println!("Part 2: {:?}", part2(input));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = input_str!(2025, 9);
        assert_eq!(part2(input), 1560299548);
    }
}
