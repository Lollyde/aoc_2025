advent_of_code::solution_with_check!(2, 31210613313, 41823587546);

#[derive(Debug, PartialOrd, PartialEq)]
struct Range (u64, u64);

fn parse_input(input: &str) -> Vec<Range> {
    input.lines().take(1).collect::<Vec<&str>>().first().unwrap().split(',').map(
        |e| {
            let v: Vec<u64> = e.split('-').map( |x| x.parse().unwrap()).collect();
            Range(v[0], v[1])
        }
    ).collect()
}

fn is_invalid_part2(input: u64, len: usize) -> bool {
    if (input.checked_ilog10().unwrap_or(0) + 1) as usize % len != 0 {
        return false;
    }
    let s = input.to_string();
    let t = s.chars().take(len).cycle();
    s.chars().zip(t).all(|(x, y)| {
        x == y
    })
}

fn is_invalid_part1(input: u64) -> bool {
    let len = input.checked_ilog10().unwrap_or(0) + 1;
    if len%2 != 0 {
        return false;
    }
    is_invalid_part2(input, len as usize/2)
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut output: u64 = 0;
    input.iter().for_each(|t| {
        for i in (t.0)..=t.1 {
            if is_invalid_part1(i) {
                output += i;
            }
        }
    });
    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut output: u64 = 0;
    input.iter().for_each(|t| {
        for i in (t.0)..=t.1 {
            let len = i.checked_ilog10().unwrap_or(0) + 1;
            for j in 1..=(len/2) {
                if is_invalid_part2(i,j as usize) {
                    output += i;
                    break
                }
            }
        }
    });
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
