advent_of_code::solution!(11);

use std::collections::VecDeque;

use ahash::{HashMap, HashMapExt};

fn path_count<'a>(
    cache: &mut HashMap<(&'a str, bool, bool), u64>,
    network: &'a HashMap<&str, Vec<&str>>,
    node: &'a str,
    mut dac: bool,
    mut fft: bool,
) -> u64 {
    if node == "out" {
        return if dac && fft { 1 } else { 0 };
    }
    dac |= node == "dac";
    fft |= node == "fft";
    let key = (node, dac, fft);
    if !cache.contains_key(&key) {
        let sub_counts = network[node]
            .iter()
            .map(|x| path_count(cache, network, x, dac, fft))
            .sum();
        cache.insert(key, sub_counts);
    }

    cache[&key]
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut mapping = HashMap::new();

    input.lines().for_each(|line| {
        let (left, right) = line.split_once(": ").unwrap();
        let nodes = right
            .split_ascii_whitespace()
            .collect::<Vec<&str>>();
        mapping.insert(left, nodes);
    });

    mapping
}

pub fn part_one(input: &str) -> Option<u64> {
    let network = parse(input);

    let mut queue = VecDeque::new();

    let you = network.get("you").unwrap();

    queue.extend(you);

    let mut path_count = 0;

    while let Some(&node) = queue.pop_front() {
        if node == "out" {
            path_count += 1;
        } else {
            let nodes = network.get(node).unwrap();
            for next in nodes {
                queue.push_front(next);
            }
        }
    }

    Some(path_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let network = parse(input);

    let mut cache = HashMap::new();

    let total = path_count(&mut cache, &network, "svr", false, false);
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
