advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    // trivial check to filter out areas that definitely do work
    // ...i suppose that was enough?...

    Some(input
        .lines()
        .skip_while(|e| !e.contains("x"))
        .map(|line| {
            let mut line = line.split_ascii_whitespace();
            let size: u64 = line.next().unwrap()
                .strip_suffix(":").unwrap()
                .split("x")
                .map(|e| e.parse::<u64>().unwrap())
                .fold(1, |a, b| a*b);
            let mut sum = 0u64;
            for p in line {
                sum += p.parse::<u64>().unwrap() * 9;
            }
            if size >= sum {
                1
            } else {
                0
            }
        })
        .sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
