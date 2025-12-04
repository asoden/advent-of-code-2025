advent_of_code::solution!(4);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    Empty,
    Removed,
    Roll,
}

fn parse(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|c| match c {
                    b'@' => Cell::Roll,
                    _ => Cell::Empty,
                })
                .collect()
        })
        .collect()
}

fn get_adjacent(grid: &[Vec<Cell>], x: i32, y: i32) -> impl Iterator<Item = Cell> + '_ {
    let width = grid.len() as i32;
    let height = grid.len() as i32;
    [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]
    .into_iter()
    .filter(move |(delta_x, delta_y)| {
        x + delta_x >= 0 && x + delta_x < width && y + delta_y >= 0 && y + delta_y < height
    })
    .map(move |(delta_x, delta_y)| {
        let new_x = x + delta_x;
        let new_y = y + delta_y;
        grid[new_y as usize][new_x as usize]
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse(input);

    let mut safe_rolls = 0;
    for (j, row) in grid.iter().enumerate() {
        for (i, cell) in row.iter().enumerate() {
            if *cell == Cell::Empty {
                continue;
            }

            if get_adjacent(&grid, i as i32, j as i32)
                .filter(|cell| *cell == Cell::Roll)
                .count()
                < 4
            {
                safe_rolls += 1;
            }
        }
    }
    Some(safe_rolls)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse(input);

    let mut total = 0;
    loop {
        let mut safe_rolls = 0;
        for y in 0..grid.len() {
            for x in 0..grid.len() {
                if grid[y][x] == Cell::Empty {
                    continue;
                }

                if get_adjacent(&grid, x as i32, y as i32)
                    .filter(|cell| *cell == Cell::Roll)
                    .count()
                    < 4
                {
                    safe_rolls += 1;
                    grid[y][x] = Cell::Removed;
                }
            }
        }

        // kludgy I hate this. Why is the safe_rolls detecting all safe items total rather than just the newly safe ones for this outer iteration??????
        if safe_rolls == total {
            break;
        }
        total = safe_rolls;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
