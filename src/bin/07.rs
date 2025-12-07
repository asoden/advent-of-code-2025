advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Start,
    Empty,
    Split,
}

#[derive(Debug)]
struct Grid {
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
        let width = value.find('\n').unwrap();
        let grid: Vec<Cell> = value
            .bytes()
            .filter(|b| *b != b'\n')
            .map(|b| match b {
                b'S' => Cell::Start,
                b'^' => Cell::Split,
                _ => Cell::Empty,
            })
            .collect();

        let height = grid.len() / width;

        Self {
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
    let start = grid.width / 2;

    current_window[start] = 1;

    for (y, dy) in (0..grid.height).skip(2).step_by(2).enumerate() {
        for x in ((start - y)..(start + y + 1)).step_by(2) {
            let present = current_window[x];
            if present != 0
                && let Some(Cell::Split) = grid.get_cell(x, dy)
            {
                split_count += 1;

                current_window[x - 1] = 1;
                current_window[x + 1] = 1;
                current_window[x] = 0;
            }
        }
    }

    Some(split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Grid = input.into();

    let start = grid.width / 2;

    let mut current_window = vec![0; grid.width];

    current_window[start] = 1;

    for (y, dy) in (0..grid.height).skip(2).step_by(2).enumerate() {
        for x in ((start - y)..(start + y + 1)).step_by(2) {
            let present = current_window[x];
            if present != 0
                && let Some(Cell::Split) = grid.get_cell(x, dy)
            {
                current_window[x - 1] += present;
                current_window[x + 1] += present;
                current_window[x] = 0;
            }
        }
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
