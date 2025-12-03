advent_of_code::solution_with_check!(3, 17443, 172167155440541);

fn parse(input: &str) -> impl Iterator<Item=Vec<u32>> {
    input.strip_suffix('\n').unwrap().split_ascii_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse(input)
        .map(|bank| -> u64 {
            let max = bank.iter().max().unwrap();
            if bank.iter().filter(|e| e==&max).count() >= 2 {
                return (max*11) as u64;
            }
            bank.split(|e| e == max)
                .map(|half| {
                    half.iter().max()
                })
                .enumerate().map(|(n,e)| {
                    match e {
                        None => 0,
                        Some(e) => {
                            if n==0 {
                                e*10+max
                            } else {
                                e+max*10
                            }
                        }
                    }
                })
                .max().unwrap() as u64
        }).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse(input)//.take(1)
        .map(|bank| {
            let mut result: Vec<u32> = Vec::new();
            let mut nextskip = 0;
            let mut counter = 12usize;
            while counter != 0 {
                let n = bank.iter().skip(nextskip).rev().skip(counter - 1).max().unwrap();
                nextskip = bank.iter().skip(nextskip).position(|e| e==n).unwrap() + nextskip + 1;
                result.push(n.clone());
                counter -= 1;
            }
            result.iter().rev().enumerate().map(|(n, e)|{
                *e as u64*(10usize.pow(n as u32) as u64)
            }).sum::<u64>()
        })
        .sum::<u64>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
