use std::ops::RangeInclusive;

advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .split(",")
        .map(|range| {
            let (left, right) = range.split_once("-").unwrap();
            left.parse().unwrap()..=right.parse().unwrap()
        })
        .collect()
}

#[inline]
fn num_digits(n: u64) -> u32 {
    n.ilog10() + 1
}

#[inline]
fn left_digits(value: u64, x: u32) -> u64 {
    value / 10_u64.pow(num_digits(value) - x)
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse(input);

    let mut val = 0;

    for range in ranges {
        for x in range {
            let num_length = num_digits(x);

            if !num_length.is_multiple_of(2) {
                continue;
            }

            let half_length = 10_u64.pow(num_length / 2);

            let left_half = x / half_length;
            let right_half = x % half_length;

            if left_half ^ right_half == 0 {
                val += x;
            }
        }
    }

    Some(val)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse(input);
    let mut val = 0;
    for range in ranges {
        'range: for x in range {
            let num_length = num_digits(x);

            for sub_length in 1..=(num_length / 2) {
                if !num_length.is_multiple_of(sub_length) {
                    continue;
                }

                let pattern = left_digits(x, sub_length);
                let divisor = 10_u64.pow(sub_length) - 1;
                let multiplier = 10_u64.pow(num_length) - 1;

                if x * divisor == pattern * multiplier {
                    val += x;
                    continue 'range;
                }
            }
        }
    }
    Some(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
