advent_of_code::solution_with_check!(7, 1646, 32451134474991);

pub fn part_one(input: &str) -> Option<u64> {
    let max_index = input.find('\n').unwrap();
    let mut beam_timelines:Vec<u64> = vec![0;max_index];
    let mut input = input.lines().step_by(2);
    beam_timelines[input.next().unwrap().find('S').unwrap()] = 1;
    Some(input.map(|line| {
            line.chars()
            .enumerate()
            .filter(|(i,e)| beam_timelines[*i] == 1 && e.eq(&'^'))
            .collect::<Vec<(usize,char)>>().iter()
            .fold(0,|splits, (index, _)| {
                beam_timelines[*index + 1] = 1;
                beam_timelines[*index - 1] = 1;
                beam_timelines[*index] = 0;
                splits + 1
            })
    }).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let max_index = input.find('\n').unwrap();
    let mut beam_timelines:Vec<u64> = vec![0;max_index];
    let mut input = input.lines().step_by(2);
    beam_timelines[input.next().unwrap().find('S').unwrap()] = 1;
    for line in input {
        line
            .chars()
            .enumerate()
            .filter(|(i,e)| beam_timelines[*i] > 0 && e.eq(&'^'))
            .collect::<Vec<(usize,char)>>().iter()
            .for_each(|(index, _)| {
                beam_timelines[*index + 1] += beam_timelines[*index];
                beam_timelines[*index - 1] += beam_timelines[*index];
                beam_timelines[*index] = 0;
            });

    }
    Some(beam_timelines.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
