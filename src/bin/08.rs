advent_of_code::solution!(8);

use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    id: usize,
}

impl Point {
    #[inline]
    fn distance_squared(&self, other: &Point) -> u64 {
        let x = self.x.abs_diff(other.x);
        let y = self.y.abs_diff(other.y);
        let z = self.z.abs_diff(other.z);

        x * x + y * y + z * z
    }
}

#[derive(Debug)]
struct Distance {
    p1: Point,
    p2: Point,
    distance: u64,
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl Eq for Distance {}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl PartialEq for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.p1 == other.p1 && self.p2 == other.p2 && self.distance == other.distance
    }
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let mut vals = [0, 0, 0];
            line.split(',').enumerate().for_each(|(index, num)| {
                vals[index] = num.parse().unwrap();
            });
            Point {
                x: vals[0],
                y: vals[1],
                z: vals[2],
                id,
            }
        })
        .collect()
}

fn calculate_distances(points: &Vec<Point>) -> BinaryHeap<Reverse<Distance>> {
    let mut distance_mapping = BinaryHeap::new();

    for (i, point1) in points.iter().enumerate() {
        for point2 in points.iter().skip(i + 1) {
            distance_mapping.push(Reverse(Distance {
                p1: *point1,
                p2: *point2,
                distance: point1.distance_squared(point2),
            }));
        }
    }

    distance_mapping
}

fn link_boxes(
    mut distance_map: BinaryHeap<Reverse<Distance>>,
    points: Vec<Point>,
    link_times: usize,
) -> Vec<Vec<Point>> {
    let mut link_actions = 0;
    let mut circuits: Vec<Vec<Point>> = Vec::new();

    for point in points {
        circuits.push(vec![point]);
    }

    while link_actions < link_times {
        let smallest_distance = distance_map.pop().unwrap().0;
        link_actions += 1;
        let (mut c1, mut c2) = (None, None);
        for (index, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&smallest_distance.p1) {
                c1 = Some(index);
            }
            if circuit.contains(&smallest_distance.p2) {
                c2 = Some(index);
            }
        }

        match (c1, c2) {
            (None, Some(d2_index)) => circuits[d2_index].push(smallest_distance.p1),
            (Some(d1_index), None) => circuits[d1_index].push(smallest_distance.p2),

            (Some(d1_index), Some(d2_index)) => {
                if d1_index != d2_index {
                    let to_move = std::mem::take(&mut circuits[d2_index]); // Sometimes you get tired of asking for memory nicely
                    circuits[d1_index].extend(to_move);

                    circuits.remove(d2_index);
                }
            }
            (None, None) => circuits.push(vec![smallest_distance.p1, smallest_distance.p2]),
        }
    }

    circuits
}

fn link_boxes_of_dumb(
    mut distance_map: BinaryHeap<Reverse<Distance>>,
    points: Vec<Point>,
) -> (i64, i64) {
    let mut circuits: Vec<Vec<Point>> = Vec::new();

    for point in &points {
        circuits.push(vec![*point]);
    }

    loop {
        let smallest_distance = distance_map.pop().unwrap().0;
        let (mut c1, mut c2) = (None, None);
        for (index, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&smallest_distance.p1) {
                c1 = Some(index);
            }
            if circuit.contains(&smallest_distance.p2) {
                c2 = Some(index);
            }
        }

        match (c1, c2) {
            (None, Some(d2_index)) => circuits[d2_index].push(smallest_distance.p1),
            (Some(d1_index), None) => circuits[d1_index].push(smallest_distance.p2),

            (Some(d1_index), Some(d2_index)) => {
                if d1_index != d2_index {
                    let to_move = std::mem::take(&mut circuits[d2_index]); // Sometimes you get tired of asking for memory nicely
                    circuits[d1_index].extend(to_move);

                    if circuits[d1_index].len() == points.len() {
                        return (smallest_distance.p1.x, smallest_distance.p2.x);
                    }

                    circuits.remove(d2_index);
                }
            }
            (None, None) => circuits.push(vec![smallest_distance.p1, smallest_distance.p2]),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse(input);

    let distance_map = calculate_distances(&points);

    #[cfg(test)]
    let link_times = 10;
    #[cfg(not(test))]
    let link_times = 1000;

    let mut circuits = link_boxes(distance_map, points, link_times);

    circuits.sort_by_key(|a| a.len());

    Some(
        circuits
            .iter()
            .rev()
            .take(3)
            .map(|circuit| circuit.len() as u64)
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let points = parse(input);

    let distance_map = calculate_distances(&points);

    let (x1, x2) = link_boxes_of_dumb(distance_map, points);
    Some(x1 * x2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
