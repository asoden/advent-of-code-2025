advent_of_code::solution!(9);

#[derive(Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    #[inline]
    fn area(&self, other: &Point) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect()
}

fn point_in_polygon(verticies: &[Point], point: Point) -> bool {
    let mut intersections = 0;
    let num_verticies = verticies.len();

    for i in 0..num_verticies {
        let point1 = &verticies[i];
        let point2 = &verticies[(i + 1) % num_verticies]; //wrap around

        if on_boundary(&point, point1, point2) {
            return true;
        }

        if ((point1.y > point.y) != (point2.y > point.y))
            && (point.x
                < (point2.x - point1.x) * (point.y - point1.y) / (point2.y - point1.y) + point1.x)
        {
            intersections += 1;
        }
    }

    // if odd it's interior
    intersections % 2 == 1
}

fn on_boundary(point: &Point, point1: &Point, point2: &Point) -> bool {
    if (point2.x - point1.x) * (point.y - point1.y) - (point2.y - point1.y) * (point.x - point1.x)
        != 0
    {
        return false;
    }

    (point.x >= point1.x.min(point2.x) && point.x <= point1.x.max(point2.x))
        && (point.y >= point1.y.min(point2.y) && point.y <= point1.y.max(point2.y))
}

#[inline]
fn orient(a: &Point, b: &Point, c: &Point) -> i64 {
    ((b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)).signum()
}

fn lines_intersect(a1: &Point, a2: &Point, b1: &Point, b2: &Point) -> bool {
    let o1 = orient(a1, a2, b1);
    let o2 = orient(a1, a2, b2);
    let o3 = orient(b1, b2, a1);
    let o4 = orient(b1, b2, a2);
    o1 * o2 < 0 && o3 * o4 < 0
}

fn lines_crossed(verticies: &[Point], point1: &Point, point2: &Point) -> bool {
    let num_verticies = verticies.len();

    let edges = [
        (
            Point {
                x: point1.x,
                y: point1.y,
            },
            Point {
                x: point2.x,
                y: point1.y,
            },
        ),
        (
            Point {
                x: point2.x,
                y: point1.y,
            },
            Point {
                x: point2.x,
                y: point2.y,
            },
        ),
        (
            Point {
                x: point2.x,
                y: point2.y,
            },
            Point {
                x: point1.x,
                y: point2.y,
            },
        ),
        (
            Point {
                x: point1.x,
                y: point2.y,
            },
            Point {
                x: point1.x,
                y: point1.y,
            },
        ),
    ];
    for i in 0..num_verticies {
        let start = &verticies[i];
        let end = &verticies[(i + 1) % num_verticies]; //wrap around

        for edge in &edges {
            if lines_intersect(start, end, &edge.0, &edge.1) {
                return true;
            }
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse(input);

    let mut max = 0;
    for (i, point1) in points.iter().enumerate() {
        for point2 in points.iter().skip(i + 1) {
            let area = point1.area(point2);
            if area > max {
                max = area;
            }
        }
    }
    Some(max)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse(input);

    let mut max = 0;
    for (i, point1) in points.iter().enumerate() {
        for point2 in points.iter().skip(i + 1) {
            let area = point1.area(point2);
            if area > max
                && point_in_polygon(
                    &points,
                    Point {
                        x: point1.x,
                        y: point2.y,
                    },
                )
                && point_in_polygon(
                    &points,
                    Point {
                        x: point2.x,
                        y: point1.y,
                    },
                )
                && !lines_crossed(&points, point1, point2) {
                    max = area;
                }
        }
    }
    Some(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
