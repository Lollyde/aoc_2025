advent_of_code::solution!(1);

fn to_rotations(input: &str) -> Vec<i64> {
    input.to_ascii_lowercase().lines().into_iter()
        .map(
            |s|
                {
                    match s.starts_with("r") {
                        true => s.strip_prefix("r").unwrap().parse::<i64>().ok().unwrap(),
                        false => s.strip_prefix("l").unwrap().parse::<i64>().ok().unwrap() * -1,
                    }
                }
        ).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let rotations: Vec<i64> = to_rotations(input);
    let mut code = 0;
    let mut value: i64 = 50;
    rotations.into_iter().for_each(
        |v|
            {
                value += v;
                value %= 100;
                if value == 0 {
                    code += 1;
                }
            }
    );
    Some(code)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rotations = to_rotations(input);
    let mut code: u64 = 0;
    let mut value: i64 = 50;
    rotations.into_iter().for_each(
        |v|
            {
                let mut turns = v;
                while turns > 0 {
                    value += 1;
                    turns -= 1;
                    if value == 100 {
                        value = 0;
                    }
                    if value == 0 {
                        code += 1;
                    }
                }
                while turns < 0 {
                    value -= 1;
                    turns += 1;
                    if value == -1 {
                        value = 99;
                    }
                    if value == 0 {
                        code += 1;
                    }
                }
            }
    );
    Some(code)
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
        assert_eq!(result, Some(6));
    }
}
