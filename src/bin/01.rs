advent_of_code::solution!(1);

use std::ops::Neg;

use atoi::atoi;

fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(|line| {
            let num = &line[1..];
            let num: i32 = atoi(num.as_bytes()).expect("Always a number.");
            if line.starts_with("L") {
                num.neg()
            } else {
                num
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut position = 50;
    let nums = parse(input);
    let mut zeros = 0;

    for rotation in nums {
        position += rotation;

        if position % 100 == 0 {
            zeros += 1;
        }
    }
    Some(zeros)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut position = 50;
    let nums = parse(input);
    let mut spins = 0;

    for rotation in nums {
        if rotation >= 0 {
            spins += (position + rotation) / 100;
        } else {
            spins += (100 - position - rotation) / 100 - (100 - position) / 100;
        }

        position = (position + rotation).rem_euclid(100);
    }

    Some(spins)
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
        assert_eq!(result, Some(6));
    }
}
