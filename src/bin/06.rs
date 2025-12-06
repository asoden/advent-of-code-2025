advent_of_code::solution!(6);

#[derive(Debug, PartialEq)]
enum Operand {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Equation {
    nums: Vec<u64>,
    operand: Operand,
}

fn parse(input: &str) -> Vec<Equation> {
    let mut iter = input.lines().rev();

    let mut equations = Vec::new();
    iter.next().iter().for_each(|line| {
        for sign in line.split_ascii_whitespace() {
            match sign {
                "+" => equations.push(Equation {
                    operand: Operand::Add,
                    nums: Vec::new(),
                }),
                _ => equations.push(Equation {
                    nums: Vec::new(),
                    operand: Operand::Multiply,
                }),
            }
        }
    });

    iter.for_each(|line| {
        for (i, val) in line.split_ascii_whitespace().enumerate() {
            equations[i].nums.push(val.parse().unwrap());
        }
    });
    equations
}

fn dumb_number_parse(input: &str) -> u64 {
    let mut bytes: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut operands = bytes.pop().unwrap();

    // delimit end of equations with special character picked at random
    operands.push('#');

    let (total, _, _) = operands
        .iter()
        .enumerate()
        .fold((0, 0, '+'), |acc, (i, &operator)| {
            let (total, partial, prev_op) = acc;
            if operator == '#' {
                (total + partial, 0, prev_op)
            } else {
                let number = bytes.iter().fold(0, |acc, row| {
                    let part = row[i];
                    if part.is_ascii_digit() {
                        10 * acc + ((part.to_digit(10).unwrap()) as u64)
                    } else {
                        acc
                    }
                });

                if number == 0 {
                    // oops all spaces skip
                    (total, partial, prev_op)
                } else {
                    match (operator, prev_op) {
                        // mid equation
                        (' ', '+') => (total, partial + number, prev_op),
                        (' ', '*') => (total, partial * number, prev_op),
                        // new equation column
                        _ => (total + partial, number, operator),
                    }
                }
            }
        });
    total
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse(input);

    let val = equations.iter().fold(0, |acc, equation| {
        acc + if equation.operand == Operand::Add {
            equation.nums.iter().copied().sum::<u64>()
        } else {
            equation.nums.iter().copied().product::<u64>()
        }
    });
    Some(val)
}

pub fn part_two(input: &str) -> Option<u64> {
    let total = dumb_number_parse(input);

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
