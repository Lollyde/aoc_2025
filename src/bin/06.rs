advent_of_code::solution_with_check!(6, 3968933219902, 6019576291014);

fn add(acc: &mut u64, e: &u64) {
    *acc += e;
}

fn mul(acc: &mut u64, e: &u64) {
    *acc *= e;
}

fn get_instructions(input: &Vec<Vec<&str>>) -> Vec<Box<&'static dyn Fn(&mut u64, &u64)>> {
    input.last().map(|line| {
        line.iter().map(|e| -> Box<&dyn Fn(&mut u64, &u64)> {
            match e {
                &"+" => Box::new(&add),
                &"*" => Box::new(&mul),
                _ => unreachable!()
            }
        }).collect()
    }).unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.lines().map(|e| e.split_ascii_whitespace().collect()).collect::<Vec<Vec<&str>>>();
    let mut sum: Vec<u64> = input[0].iter().map(|e| e.parse().unwrap()).collect();
    let instructions = get_instructions(&input);
    for row in input[1..].split_last().unwrap().1 {
        row.iter().map(|e| e.parse::<u64>().unwrap()).enumerate().zip(&instructions).for_each(|((index,e), instr)| {
            instr(&mut sum[index], &e);
        });
    }
    Some(sum.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.lines().map(|e| {
        e.chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();
    let mut sum = 0;
    let columns = input.first().unwrap().len();
    let mut is_add = false;

    let mut buf: Vec<u64> = Vec::new();

    for i in 0..columns {
        let column: Vec<&char> = input.iter().map(|e| e.iter().skip(i).take(1).next().unwrap()).collect();
        if column.iter().all(|e| (*e).eq(&' ')) {
            sum += match is_add {
                true => buf.iter().sum(),
                false => buf.iter().fold(1u64, |acc, e| acc * e),
            };
            buf = Vec::new();
            continue;
        }
        let mut num = 0;
        let mut instr_set = false;
        column.iter().for_each(|c| {
            match c {
                &&'\n' => {},
                &&' ' => {},
                &&'+' => {
                    is_add = true;
                    instr_set = true;
                },
                &&'*' => {
                    is_add = false;
                    instr_set = true;
                },
                _ => {
                    num *= 10;
                    num += c.to_digit(10).unwrap() as u64;
                }
            }
        });
        buf.push(num);
    }
    sum += match is_add {
        true => buf.iter().sum(),
        false => buf.iter().fold(1u64, |acc, e| acc * e),
    };
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
