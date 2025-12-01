advent_of_code::solution!(1);

use std::ops::Neg;

use atoi::atoi;

fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(|line| {
            let direction = &line[0..1];
            let num = &line[1..];
            let num: i32 = atoi(num.as_bytes()).expect("Always a number.");
            match direction {
                "L" => num.neg(),
                "R" => num,
                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut position = 50;
    let nums = parse(input);
    Some(nums.iter().fold(0, |acc, rotation| {
        position += rotation;

        if position % 100 == 0 {
            return acc + 1;
        }
        acc
    }))
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut position = 50;
    let nums = parse(input);
    Some(nums.into_iter().fold(0, |acc, rotation| {
        let mut spins = 0;

        if rotation >= 0 {
            spins += (position + rotation) / 100;
        } else {
            spins += (100 - position - rotation) / 100 - (100 - position) / 100;
        }

        position = (position + rotation).rem_euclid(100);
        acc + spins
    }))
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
