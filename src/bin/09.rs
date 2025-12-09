advent_of_code::solution_with_check!(9, 4777967538, 1439894345);

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

    fn overlaps(&self, line: &Line) -> bool {
        let from_inside = self.is_inside_inclusive(line.from);
        let to_inside = self.is_inside_inclusive(line.to);

        if from_inside && to_inside
            && !self.is_on_perimeter(line) {
            return true;
        }

        let a = Point {
            x: self.points[0].x,
            y: self.points[1].y
        };

        let b = Point {
            x: self.points[0].y,
            y: self.points[1].x
        };

        let self_lines = [
            Line{from: self.points[0], to: &a},
            Line{from: self.points[0], to: &b},
            Line{from: self.points[1], to: &a},
            Line{from: self.points[1], to: &b}
        ];

        self_lines.iter().any(|e| e.intersects(line))
    }

    fn is_on_perimeter(&self, line: &Line) -> bool {
        if line.is_horizontal() {
            line.from.x == self.points[0].x || line.from.x == self.points[1].x
        } else {
            line.from.y == self.points[0].y || line.from.y == self.points[1].y
        }
    }

    fn is_inside_inclusive(&self, point: &Point) -> bool {
        is_between_inclusive((&self.points[0].x, &self.points[1].x), &point.x) &&
            is_between_inclusive((&self.points[0].y, &self.points[1].y), &point.y)
    }

    fn is_inside_exclusive(&self, point: &Point) -> bool {
        is_between_exclusive((&self.points[0].x, &self.points[1].x), &point.x) &&
            is_between_exclusive((&self.points[0].y, &self.points[1].y), &point.y)
    }

}

fn is_between_inclusive(a: (&usize, &usize), b: &usize) -> bool {
    if a.0 < a.1 {
        a.0 <= b && b <= a.1
    } else {
        a.1 <= b && b <= a.0
    }
}

fn is_between_exclusive(a: (&usize, &usize), b: &usize) -> bool {
    if a.0 < a.1 {
        a.0 < b && b < a.1
    } else {
        a.1 < b && b < a.0
    }
}

#[derive(Debug)]
struct Line<'a> {
    from: &'a Point,
    to: &'a Point
}

impl Line<'_> {
    fn intersects(&self, other: &Line) -> bool {
        ((self.from.y > other.from.y && self.to.y < other.from.y) ||
            (self.from.y < other.from.y && self.to.y > other.from.y)) &&
        (self.from.x > other.from.x && self.to.x < other.from.x) ||
            (self.from.x < other.from.x && self.to.x > other.from.x)

    }

    fn is_horizontal(&self) -> bool {
        self.from.x == self.to.x
    }

    fn from<'a>(from: &'a Point, to: &'a Point) -> Line<'a> {
        Line{
            from,
            to
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
        (self.x.abs_diff(other.x) + 1)  as u64 * (self.y.abs_diff(other.y) + 1) as u64
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

pub fn part_two_old(input: &str) -> Option<u64> {
    let points = input.lines().map(|e| e.parse::<Point>().unwrap()).collect::<Vec<Point>>();
    let lines = points
        .iter()
        .zip(
            points
            .iter()
            .cycle()
            .skip(1))
        .map(|(p1, p2)|{
            Line::from(p1, p2)
        }).collect::<Vec<Line>>();
    let res = points
        .iter()
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
        .filter(|rect| lines.iter().all(|line| {!rect.overlaps(line)}))
        .max_by_key(|rect| rect.area())
        .unwrap();
    dbg!(&res);
    Some(res.area())
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = input.lines().map(|e| e.parse::<Point>().unwrap()).collect::<Vec<Point>>();
    let perimeter = points
        .iter()
        .zip(
            points
                .iter()
                .cycle()
                .skip(1))
        .map(|(p1, p2)|{
            let mut perimeter = Vec::with_capacity(p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y));
            if p1.x == p2.x {
                let y1;
                let y2;
                if p1.y < p2.y {
                    y1 = p1.y;
                    y2 = p2.y;
                } else {
                    y1 = p2.y;
                    y2 = p1.y;
                }
                for y in y1..y2 {
                    perimeter.push(
                        Point{
                            x: p1.x,
                            y
                        }
                    )
                }
            } else {
                let x1;
                let x2;
                if p1.x < p2.x {
                    x1 = p1.x;
                    x2 = p2.x;
                } else {
                    x1 = p2.x;
                    x2 = p1.x;
                }
                for x in x1..x2 {
                    perimeter.push(
                        Point{
                            x,
                            y: p1.y
                        }
                    )
                }
            }
            perimeter
        })
        .flatten()
        .collect::<Vec<Point>>();
    let res = points
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
        .filter(|rect| !perimeter.iter().any(|point| rect.is_inside_exclusive(point)))
        .max_by_key(|rect| rect.area())
        .unwrap();
    Some(res.area())
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
