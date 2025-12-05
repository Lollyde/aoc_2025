advent_of_code::solution_with_check!(5, 874, 348548952146313);

struct Point {
    start: bool,
    value: u64
}

impl Point {
    fn from(input: FreshRange) -> (Point, Point) {
        (
            Point {
                start: true,
                value: input.start
            },
            Point {
                start: false,
                value: input.end + 1
            }
        )
    }
}

#[derive(Debug, Copy, Clone)]
struct FreshRange {
    start: u64,
    end: u64,
}

impl FreshRange {
    fn is_in_range(&self, input: &u64) -> bool {
        input <= &self.end && input >= &self.start
    }

    fn from(input: &str) -> Option<FreshRange> {
        let start = input
            .split('-')
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .parse::<u64>()
            .ok();
        let end = input
            .split('-')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<u64>()
            .ok();
        if start.is_some() && end.is_some() {
            Some(FreshRange {
                start: start.unwrap(),
                end: end.unwrap(),
            })
        } else {
            None
        }
    }
}

fn parse(input: &str) -> (Vec<FreshRange>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    input.split("\n\n").enumerate().for_each(|(index, e)| {
        if index == 0 {
            e.split('\n').for_each(|e| match FreshRange::from(e) {
                Some(e) => ranges.push(e),
                None => {
                    panic!("ah")
                }
            })
        } else {
            e.lines().for_each(|e| match e.parse() {
                Ok(e) => ids.push(e),
                Err(e) => panic!("{}", e),
            })
        }
    });
    (ranges, ids)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = parse(input);
    Some(
        ids.iter()
            .filter(|id| {
            ranges.iter()
                .any(|range| {
                    range.is_in_range(id)
                })
            })
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse(input);
    let mut points = Vec::new();
    ranges.into_iter().for_each(|e| {
        let (one,two) = Point::from(e);
        points.push(one);
        points.push(two);
    });
    points.sort_by_key(|e| e.value);
    let mut result: u64 = 0;
    let mut last = points[0].value;
    let mut counter = 0;
    points.iter().for_each(|e| {
        match e.start {
            true => {
                if counter == 0 {
                    last = e.value;
                }
                counter += 1;
            },
            false => {
                counter -= 1;
                if counter == 0 {
                    result += e.value - last;
                }
            }
        }
    });
    Some(result)
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
        assert_eq!(result, Some(14));
    }
}
