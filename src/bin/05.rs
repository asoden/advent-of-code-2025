advent_of_code::solution!(5);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    #[inline]
    fn contains(&self, other: &Range) -> bool {
        self.start > other.end || self.end < other.start
    }
}

fn parse(input: &str) -> (Vec<Range>, Vec<u64>) {
    let (ranges, values) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            Range {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            }
        })
        .collect();

    let values = values.lines().map(|line| line.parse().unwrap()).collect();

    (ranges, values)
}

fn find_overlap(ranges: &[Range]) -> Option<(usize, usize)> {
    for (i, range1) in ranges.iter().enumerate().rev() {
        for (j, range2) in ranges.iter().enumerate().skip(i + 1) {
            if !range1.contains(range2) {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, mut values) = parse(input);

    values.sort();

    let mut total = 0;
    'ingredient: for value in values {
        for range in &ranges {
            if (range.start..=range.end).contains(&value) {
                total += 1;
                continue 'ingredient;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut ranges, _) = parse(input);

    ranges.sort();

    while let Some((i, j)) = find_overlap(&ranges) {
        let range1 = ranges.remove(j);
        let range2 = ranges.remove(i);

        let start = range1.start.min(range2.start);
        let end = range1.end.max(range2.end);

        let combined = Range { start, end };
        ranges.push(combined);
    }

    let count = ranges
        .into_iter()
        .map(|range| range.end - range.start + 1)
        .sum();
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
