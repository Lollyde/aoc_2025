use std::collections::BTreeMap;
use std::hint::unreachable_unchecked;
use rayon::prelude::*;
advent_of_code::solution!(11);

fn parse<'a>(input: &'a str, graph: &mut BTreeMap<&'a str,Vec<&'a str>>) {
    input
        .lines()
        .for_each(|line| {
            let mut line =
                line
                    .split_ascii_whitespace();
            let name =
                line
                    .next().unwrap()
                    .strip_suffix(':').unwrap();
            graph
                .insert(name, line.collect::<Vec<&str>>());
        });
}

fn count_paths(graph: &BTreeMap<&str, Vec<&str>>, from: &str, to: &str) -> u64 {
    let mut counts: BTreeMap<&str, u64> = BTreeMap::new();
    let mut next = graph.get(from).unwrap_or_else(|| {
        dbg!(from, graph);
        panic!()
    }).clone();
    loop {
        next.iter().for_each(|e| {
            match counts.get_mut(e) {
                Some(c) => *c += 1,
                None => _ =counts.insert(e, 1)
            };
        });
        next =
            next
                .par_iter()
                .filter(|e| {
                    !e.eq(&&to) && !e.eq(&&"out") && !e.eq(&&from)
                })
                .map(|e| {
                    match graph.get(e) {
                        Some(v) => v,
                        None => {
                            unsafe {
                                unreachable_unchecked()
                            }
                        }
                    }
                })
                .flatten()
                .map(|e| {
                    *e
                })
                .collect();
        if next.len() == 0 {
            break
        }
    }
    *counts.get(to).unwrap_or(&0u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut graph = BTreeMap::new();
    parse(&input, &mut graph);
    Some(count_paths(&graph, "you", "out"))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut graph = BTreeMap::new();
    parse(&input, &mut graph);
    Some([
        [("svr", "dac"), ("dac", "fft"), ("fft", "out")],
        [("svr", "fft"), ("fft", "dac"), ("dac", "out")]
    ].iter()
        .map(|path| {
            path
                .iter()
                .map(|(from, to)| {
                    let result = count_paths(&graph, from, to);
                    println!("finished path from {} to {}", from, to);
                    result
                })
                .reduce(|acc, e| acc * e)
                .unwrap()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}
