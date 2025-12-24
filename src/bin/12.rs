advent_of_code::solution!(12);

#[derive(Debug, Clone)]
struct Region {
    width: u64,
    height: u64,
    presents: Vec<u64>,
}

fn parse(input: &str) -> Vec<Region> {
    let line_split = input.split("\n\n");
    let regions = line_split.last().unwrap();

    regions
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            let (width, height) = left.split_once('x').unwrap();
            let presents = right
                .split_ascii_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();

            Region {
                width: width.parse().unwrap(),
                height: height.parse().unwrap(),
                presents,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let regions = parse(input);

    let total = regions
        .iter()
        .map(|region| {
            let mut sub_value = 0;
            let area = region.width * region.height;
            for present_num in region.presents.iter() {
                sub_value += present_num;
            }

            if sub_value * 9 <= area { 1 } else { 0 }
        })
        .sum();
    Some(total)
}

pub fn part_two(_: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
