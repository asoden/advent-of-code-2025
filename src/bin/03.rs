advent_of_code::solution!(3);

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|value| match value {
                    x @ b'1'..=b'9' => x - b'0',
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let powerbanks = parse(input);

    let val = powerbanks
        .iter()
        .map(|powerbank| {
            let mut biggest = (0, 0);
            for (i, &value) in powerbank.iter().enumerate().take(powerbank.len() - 1) {
                if value > biggest.1 {
                    biggest = (i, value);
                }
            }

            let tens = biggest.1;
            let start = biggest.0;
            biggest = (0, 0);
            for &value in powerbank.iter().skip(start + 1) {
                if value > biggest.1 {
                    biggest.1 = value;
                }
            }
            let ones = biggest.1;

            (tens * 10 + ones) as u64
        })
        .sum();

    Some(val)
}

pub fn part_two(input: &str) -> Option<u64> {
    let powerbanks = parse(input);

    let val = powerbanks
        .iter()
        .map(|powerbank| {
            let mut buffer_end: i32 = 11;
            let mut values: Vec<u8> = Vec::with_capacity(11);
            let mut biggest = (0, 0);

            for (i, &value) in powerbank
                .iter()
                .enumerate()
                .take(powerbank.len() - buffer_end as usize)
            {
                if value > biggest.1 {
                    biggest = (i, value);
                }
            }
            values.push(biggest.1);
            buffer_end -= 1;

            while buffer_end >= 0 {
                let start = biggest.0 + 1;
                biggest = (0, 0);

                for (i, &value) in powerbank
                    .iter()
                    .enumerate()
                    .skip(start)
                    .take(powerbank.len() - start - buffer_end as usize)
                {
                    if value > biggest.1 {
                        biggest = (i, value);
                    }
                }
                values.push(biggest.1);
                buffer_end -= 1;
            }
            values.iter().fold(0_u64, |acc, x| acc * 10 + *x as u64)
        })
        .sum();
    Some(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
