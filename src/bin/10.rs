advent_of_code::solution_with_check!(10, 419, 18369);

//use std::collections::BTreeMap;
use std::ops::BitXor;
use std::str::FromStr;
use rayon::prelude::*;


#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ButtonState {
    current_state: Vec<u16>
}

impl ButtonState {
#[allow(dead_code)]
    fn new(desired_state: &Vec<u16>) -> ButtonState {
        let current_state = vec![0; desired_state.len()];
        ButtonState {
            //desired_state: desired_state.clone(),
            current_state,
        }
    }

#[allow(dead_code)]
    fn distance(&self, desired_state: &Vec<u16>) -> Option<u16> {
        let result = desired_state
            .iter()
            .rev()
            .zip(&self.current_state)
            .map(|(desired, current)| {
                desired.checked_sub(*current)
            })
            .collect::<Option<Vec<u16>>>();
        match result {
            Some(v) => {
                Some(v.iter().map(|e| *e as u16).sum::<u16>())
            },
            None => None
        }
    }

#[allow(dead_code)]
    fn apply(&self, buttons: &Vec<u16>, desired_state: &Vec<u16>) -> Vec<ButtonState> {
        buttons
            .iter()
            .map(|button| {
                let mut current_state = self.current_state.clone();
                let mut button = *button;
                let mut index = 0;
                while button > 0 {
                    if button & 1 != 0 {
                        current_state[index] += 1;
                    }
                    index += 1;
                    button = button >> 1;
                }
                let res = ButtonState {
                    current_state
                };
                match res.distance(desired_state) {
                    Some(_) => Some(res),
                    None => None
                }
            })
            .filter(|e| e.is_some())
            .map(|e| {
                match e {
                    Some(e) => e,
                    None => unreachable!()
                }
            })
            .collect()
    }
}

#[derive(Debug)]
struct Machine {
    desired_state: u16,
    buttons: Vec<u16>,
    joltages: Vec<u16>
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(' ');
        let desired_state = s
            .clone()
            .next().unwrap()
            .chars()
            .rev()
            .skip(1)
            .take_while(|e| e != &']')
            .map(|char| char == '#')
            .fold(0u16, |acc, e| {
                if e {
                    (acc << 1) + 1
                } else {
                    acc << 1
                }
            }) >> 1;
        let buttons: Vec<u16> = s
            .clone()
            .take_while(|e| !e.contains('{'))
            .map(|button| {
                button
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .map(|c| c.to_digit(10).unwrap())
                    .fold(0u16, |acc, e| {
                        acc + (1u16 << e)
                    })
            })
            .collect();
        let joltages = s
            .skip_while(|e| !e.contains('{'))
            .next().unwrap()
            .strip_prefix('{').unwrap()
            .strip_suffix('}').unwrap()
            .split(',')
            .rev()
            .map(|e| e.parse().unwrap())
            .collect();
        Ok(
            Machine {
                desired_state,
                buttons,
                joltages
            }
        )
    }
}

impl Machine {
    fn solve_part1(&self) -> u64 {
        let mut count = 0;
        let mut states = Vec::with_capacity(32);
        states.push(0u16);
        'a: loop {
            states = states
                .iter()
                .map(|state| {
                    self.buttons.iter().map(|button| {
                        state.bitxor(button)
                    })
                })
                .flatten()
                .collect();
            states.dedup();
            count += 1;

            if states.iter().any(|e| e == &self.desired_state) {
                break 'a
            }
        }
        count
    }

#[allow(dead_code)]
    #[allow(unused_assignments)]
    fn solve_part2(&self) -> u64 {
        let mut count = 0;
        let mut min = u16::MAX;
        let mut states = Vec::with_capacity(self.joltages.len());
        states.push(ButtonState::new(&self.joltages));
        'a: loop {
            min = states.par_iter().min_by_key(|e| e.distance(&self.joltages)).unwrap().distance(&self.joltages).unwrap();
            if min == 0 {
                break 'a;
            }
            let mut new =
                states
                    .par_iter()
                    .filter(|e| e.distance(&self.joltages).unwrap() ==  min)
                    .map(|e| {
                        e.apply(&self.buttons, &self.joltages)
                    })
                    .flatten()
                    .collect();
            states.append(&mut new);
            count += 1;
        }
        count
    }
    /*fn solve_part2(&self) -> u64 {
        let mut count = 0;
        let goal = Coordinates::new(&self.joltages);
        count
    }*/
}

/*#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Coordinates([u8;10]);

impl std::ops::Deref for Coordinates {
    type Target = [u8;10];

    fn deref(& self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Coordinates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Add<&Vec<u8>> for &Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: &Vec<u8>) -> Self::Output {
        let mut target = Coordinates::new(rhs);
        for i in 0..10 {
            target[i] += self[i];
        }
        target
    }
}

impl Coordinates {
    fn zero() -> Coordinates {
        Coordinates([0;10])
    }

    fn new(from: &Vec<u8>) -> Coordinates {
        let mut coords = [0u8;10];
        let mut nums = from.iter();
        for i in 0..10 {
            match nums.next() {
                Some(n) => coords[i]=*n,
                None => coords[i]=0
            }
        }
        Coordinates(coords)
    }

    fn distance_to(&self, other: &Coordinates) -> u16 {
        let mut distance: u16 = 0;
        for i in 0..10 {
            if self[i] > other[i] {
                return u16::MAX;
            }
            distance += match other[i].checked_sub(self[i]) {
                Some(v) => v,
                None => unreachable!()
            } as u16;
        }
        distance
    }
}*/



pub fn part_one(input: &str) -> Option<u64> {
    Some(input.
        lines()
        .map(|line| line.parse::<Machine>().unwrap())
        .map(|machine| machine.solve_part1())
        .sum())
}

// https://github.com/timvisee/advent-of-code-2025/blob/master/day10b/src/main.rs
pub fn part_two(input: &str) -> Option<u64> {
    use microlp::{LinearExpr, OptimizationDirection, Problem};
    let presses = input
        .lines()
        .map(|line| {
            let (first, last) = line.split_at(line.chars().position(|b| b == '{').unwrap());

            let btns = first[1..]
                .split_ascii_whitespace()
                .skip(1)
                .filter(|btns| !btns.is_empty())
                .map(|btns| {
                    btns[1..]
                        .split(',')
                        .map(|n| 1 << n[0..1].parse::<u16>().unwrap())
                        .sum()
                })
                .collect::<Vec<u16>>();
            let jolts = last[1..]
                .strip_suffix('}').unwrap()
                .split(',')
                .map(|b| b.parse::<u16>().unwrap())
                .collect::<Vec<u16>>();

            let mut problem = Problem::new(OptimizationDirection::Minimize);
            let max = jolts.iter().copied().max().unwrap();
            let vars = (0..btns.len())
                .map(|_| problem.add_integer_var(1.0, (0, max as i32)))
                .collect::<Vec<_>>();
            for (i, &n) in jolts.iter().enumerate() {
                problem.add_constraint(
                    btns.iter()
                        .zip(&vars)
                        .filter(|&(mask, _)| mask & (1 << i) != 0)
                        .fold(LinearExpr::empty(), |mut ex, (_, &var)| {
                            ex.add(var, 1.0);
                            ex
                        }),
                    microlp::ComparisonOp::Eq,
                    n as f64,
                );
            }
            problem.solve().unwrap().objective().round() as usize
        })
        .sum::<usize>();
    Some(presses as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(3));
    }
    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(2));
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(12));
    }
    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(11));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
