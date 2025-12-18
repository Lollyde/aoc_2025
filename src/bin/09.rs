advent_of_code::solution_with_check!(9, 4777967538, 1439894345);

use std::cmp::{max, min};
use rayon::prelude::*;

#[derive(Debug)]
struct Rect<'a> {
    points: [&'a Point; 2]
}

impl Rect<'_> {
    fn from<'a>(from: &'a Point, to: &'a Point) -> Rect<'a> {
        Rect {
            points: [from, to]
        }
    }

    fn area(&self) -> u64 {
        self.points[0].area_with(self.points[1])
    }

    fn overlaps_with(&self, line: &Line) -> bool {

        let bottom = min(self.points[0].y, self.points[1].y);
        let top = max(self.points[0].y, self.points[1].y);
        let left = min(self.points[0].x, self.points[1].x);
        let right = max(self.points[0].x, self.points[1].x);

        let axis_min;
        let axis_max;
        let bound_min;
        let bound_max;

        if line.horizontal {
            axis_min = bottom;
            axis_max = top;
            bound_min = left;
            bound_max = right;
        } else {
            axis_min = left;
            axis_max = right;
            bound_min = bottom;
            bound_max = top;
        }


        // axis goes THROUGH the square, not counting overlapping with edges
        (line.axis > axis_min && axis_max > line.axis) &&
            (
                // AND the points arent outside of the bounds of the square
                !(line.from >= bound_max && line.to >= bound_max) &&
                !(line.from <= bound_min && line.to <= bound_min)
            )
    }
}

#[derive(Debug)]
struct Line {
    from: usize,
    to: usize,
    axis: usize,
    horizontal: bool
}

impl Line {
    fn new(from: &Point, to: &Point) -> Line {
        let horizontal = from.y == to.y;
        Line {
            from: min(if horizontal {from.x} else {from.y}, if horizontal {to.x} else {to.y}),
            to: max(if horizontal {from.x} else {from.y}, if horizontal {to.x} else {to.y}),
            axis: if horizontal {from.y} else {from.x},
            horizontal
        }
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn area_with(&self, other: &Point) -> u64 {
        (self.x.abs_diff(other.x) + 1) as u64 * (self.y.abs_diff(other.y) + 1) as u64
    }
}

impl std::str::FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(',')
                .into_iter()
                .map(|e|
                    e.parse::<usize>().ok()
                )
                .collect::<Option<Vec<usize>>>() {
            Some(result) => Ok(Point{
                x: result[0],
                y: result[1]
            }),
            None => Err(())
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let points: Vec<Point> = input.lines().map(|line| line.parse::<Point>().unwrap()).collect();
    points
        .iter()
        .map(|e| {
            points
                .iter()
                .map(|p| p.area_with(e))
                .collect::<Vec<u64>>()
            })
        .flatten()
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = input.lines().map(|e| e.parse::<Point>().unwrap()).collect::<Vec<Point>>();
    let lines: Vec<Line> = points
        .iter()
        .zip(
            points
                .iter()
                .skip(1))
        .map(|(p1, p2)| {
            Line::new(p1, p2)
        })
        .collect();

    points
        .par_iter()
        .enumerate()
        .map(|(index1, e)| {
            points
                .iter()
                .enumerate()
                .filter(|(index2, _)| index1 < *index2)
                .map(|(_, p)| Rect::from(p, e))
                .collect::<Vec<Rect>>()
        })
        .flatten()
        .fold(|| {0}, |max, rect| {
            let area = rect.area();
            if area > max {
                if lines.iter().any(|e| rect.overlaps_with(e)) {
                    max
                } else {
                    area
                }
            } else {
                max
            }
        }).max()
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

    #[test]
    fn test_part_two_2(){
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(40));
    }
    #[test]
    fn test_part_two_3(){
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(35));
    }
    #[test]
    fn test_part_two_4(){
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(66));
    }
}
