use std::collections::HashSet;

advent_of_code::solution_with_check!(4, 1351, 8345);

fn parse(input: &str) -> HashSet<(i32,i32)> {
    let mut map = HashSet::new();
    input.lines().filter(|e| e.len() > 2).enumerate()
        .for_each(|(i,line)| {
            line.chars().enumerate().for_each(|(j,c)| {
                if c.eq(&'@') {
                    map.insert((i as i32,j as i32));
                }
            })
        });
    map
}


fn count_movable(input: &HashSet<(i32,i32)>) -> u64 {
    input.iter().filter(|(x,y)| {
        OFFSETS.iter().filter(|(offset_x,offset_y)| {
            input.contains(&(x+offset_x, y+offset_y))
        }).count() < 4
    }).count() as u64
}

fn count_and_remove(input: &mut HashSet<(i32, i32)>) -> u64 {
    let init = input.iter().count();
    input.clone().into_iter().for_each(|(x,y)| {
        if OFFSETS.iter().filter(|(offset_x,offset_y)| {
            input.contains(&(x+offset_x, y+offset_y))
        }).count() < 4 {
            input.remove(&(x,y));
        }
    });
    (init - input.iter().count()) as u64
}

const OFFSETS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);
    Some(count_movable(&input))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = parse(input);
    let mut counter = 0;
    let mut steps = count_and_remove(&mut input);
    while steps > 0 {
        counter += steps;
        steps = count_and_remove(&mut input);
    }
    Some(counter)
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
