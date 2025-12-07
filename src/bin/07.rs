advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Start,
    Empty,
    Split,
}

#[derive(Debug)]
struct Grid {
    start: (usize, usize),
    grid: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    #[inline]
    fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        self.grid.get(y * self.width + x).copied()
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut start = (0, 0);
        let width = value.find('\n').unwrap();
        let grid: Vec<Cell> = value
            .bytes()
            .filter(|b| *b != b'\n')
            .enumerate()
            .map(|(i, b)| match b {
                b'S' => {
                    start = (i % width, i / width);
                    Cell::Start
                }
                b'^' => Cell::Split,
                _ => Cell::Empty
            })
            .collect();

        let height = grid.len() / width;

        Self {
            start,
            grid,
            width,
            height,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut split_count = 0;

    let grid: Grid = input.into();

    let mut current_window = vec![0; grid.width];
    let mut next = vec![0; grid.width];

    current_window[grid.start.0] = 1;

    for y in 0..grid.height {
        for (x, &val) in current_window.iter().enumerate() {
            let present = val;
            if present != 0 {
                if let Some(Cell::Split) = grid.get_cell(x, y) {
                    split_count += 1;

                    if x > 0 {
                        next[x - 1] = val;
                    }

                    if x < grid.width - 1 {
                        next[x + 1] = val;
                    }
                } else {
                    next[x] = val;
                }
            }
        }

        (current_window, next) = (next, current_window);
        next.fill(0);
    }

    Some(split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Grid = input.into();

    let mut current_window = vec![0; grid.width];
    let mut next = vec![0; grid.width];

    current_window[grid.start.0] = 1;

    for y in 0..grid.height {
        for (x, &val) in current_window.iter().enumerate() {
            let present = val;
            if present > 0 {
                if let Some(Cell::Split) = grid.get_cell(x, y) {

                    if x > 0 {
                        next[x - 1] += val;
                    }

                    if x < grid.width - 1 {
                        next[x + 1] += val;
                    }
                } else {
                    next[x] += val;
                }
            }
        }

        (current_window, next) = (next, current_window);
        next.fill(0);
    }

    Some(current_window.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
