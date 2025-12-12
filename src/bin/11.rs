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
    let mut next = match graph.get(from){
        Some(v) => v.clone(),
        None => return 0
    };
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

// https://old.reddit.com/r/adventofcode/comments/1pjp1rm/2025_day_11_solutions/ntl1jlg/
// https://topaz.github.io/paste/#XQAAAQBqAwAAAAAAAAAkk4ZG/3FmkOEiq1dFAAFDFSTdMr6/b8mE3YPdTt/5lzxZMpUhTYTHPIx1KQTzz0gFw4jNvbk0+5bXHczfghuyhpIZUvzY4pz3Yac9kVWiyUBRbskxIPTVsRHjiB97Das8YLJ1qA1D+mVVuae2UQUsIDO5g6pNKObWCYPA9B8OHfzx6Uj6blqUSB/EG8QLP5zwd7gNE7qjRmkpQvCDGc18T4Fw1x4jQYAub3LYbnaVIP6oCdtaPmow9pjyUOW35pSoVDrgQNT3ht5T6wTO9C+2fWtklIXxjw2ubk2Tw6U9gj5SQXHiQ1MSRMVUkrBRERhGxM9/diQ/CB+/WYu/Yqa6spoO8Nn/u0zgyX52j5GWpvEtewntMUx4Frh4tEuSWJufSqWMgdl23Dx7H1BGz5JICvSMQ0pVgo2ki0TfjLvbG0bHn+XZ2au8ZsGXD/GYl+EzKj2w+D7l0zMpetOuZQypaMX09KKEF78MoBboGYuIlQOJgoWkfItuvKr//YHWcg==
fn count_fast<'a>(graph: &BTreeMap<&'a str, Vec<&'a str>>, from: &'a str, to: &str, visited: &mut Vec<&'a str>, scores: &mut BTreeMap<&'a str, u64>) -> u64 {
    if from.eq(to) {
        return 1;
    }
    if from.eq("out") || visited.contains(&from) {
        return 0;
    }
    match scores.get(from) {
        Some(score) => return *score,
        None => {}
    }
    visited.push(from);
    let total: u64 = graph.get(from).unwrap().iter().map(|k| {
        count_fast(graph, k, to, visited, scores)
    }).sum();
    visited.remove(visited.iter().position(|e| *e == from).unwrap());
    scores.insert(from, total);
    total
}

fn prune<'a>(mut graph: BTreeMap<&'a str, Vec<&'a str>>, from: &str, to: &str) -> BTreeMap<&'a str, Vec<&'a str>> {
    if to == "out" {
        return graph;
    }
    let mut pruned = Vec::with_capacity(graph.len());
    pruned.push("out");
    loop {
        graph
            .iter()
            .for_each(|(node, value)| {
                if *node == to {
                    return
                }
                if value.iter().all(|e| pruned.contains(e)) {
                    pruned.push(node);
                }
            });
        graph = graph
            .iter()
            .map(|(k, v)| {
                (*k, v.into_iter().filter(|e| !pruned.contains(e)).map(|e| *e).collect::<Vec<&str>>())
            }).collect();
        let before = graph.len();
        for p in &pruned {
            graph.remove(*p);
        }
        if graph.len() == before {
            return graph
        }
    }
}


pub fn part_one(input: &str) -> Option<u64> {
    let mut graph = BTreeMap::new();
    parse(&input, &mut graph);
    Some(count_fast(&graph, "you", "out", &mut Vec::with_capacity(graph.len()), &mut BTreeMap::new()))
    //Some(count_paths(&graph, "you", "out"))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut graph = BTreeMap::new();
    parse(&input, &mut graph);
    let a = count_fast(&graph, "svr", "dac", &mut Vec::with_capacity(graph.len()), &mut BTreeMap::new());
    let b = count_fast(&graph, "dac", "fft", &mut Vec::with_capacity(graph.len()), &mut BTreeMap::new());
    let c = count_fast(&graph, "fft", "out", &mut Vec::with_capacity(graph.len()), &mut BTreeMap::new());
    let d = count_fast(&graph, "svr", "fft", &mut Vec::with_capacity(graph.len()), &mut BTreeMap::new());
    let e = count_fast(&graph, "fft", "dac", &mut Vec::with_capacity(graph.len()), &mut BTreeMap::new());
    let f = count_fast(&graph, "dac", "out", &mut Vec::with_capacity(graph.len()), &mut BTreeMap::new());
    dbg!(a,b,c,d,e,f);
    Some((a*b*c) + (d*e*f))
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
