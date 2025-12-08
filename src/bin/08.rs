advent_of_code::solution!(8);

#[derive(Debug)]
struct Vertex {
    id: u16,
    x: u32,
    y: u32,
    z: u32
}

impl Vertex {
    fn from_str(input: &str, id: usize) -> Vertex {
        let mut input = input.split(',');
        Vertex {
            x: input.next().unwrap().parse().unwrap(),
            y: input.next().unwrap().parse().unwrap(),
            z: input.next().unwrap().parse().unwrap(),
            id: id as u16
        }
    }
}

#[derive(Debug)]
struct Edge<'a> {
    from: &'a u16,
    to: &'a u16,
    distance: u64
}

impl Vertex {
    fn distance_to_squared_with_ids<'a>(&'a self, other: &'a Vertex) -> Edge<'a> {
        let mut distance = (self.x.abs_diff(other.x) as u64).pow(2);
        distance += (self.y.abs_diff(other.y) as u64).pow(2);
        distance += (self.z.abs_diff(other.z)as u64).pow(2);
        let (smaller, larger) = match self.id < other.id {
            true => (&self.id, &other.id),
            false => (&other.id, &self.id)
        };
        Edge {
            from: smaller,
            to: larger,
            distance
        }
    }
}

fn find_root(roots: &Vec<usize>, index: usize) -> usize {
    if roots[index] == index {
        return index
    }
    find_root(roots, roots[index])
}

fn combine(roots: &mut Vec<usize>, rank: &mut Vec<usize>, a: usize, b: usize) {
    let aroot = find_root(&roots, a);
    let broot = find_root(&roots, b);

    if rank[aroot] < rank[broot] {
        roots[aroot] = broot;
    } else if rank[aroot] > rank[broot] {
        roots[broot] = aroot;
    } else {
        roots[aroot] = broot;
        rank[broot] += 1;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut distances: Vec<Edge> = Vec::with_capacity(1000000);
    let points: Vec<Vertex> = input.lines().enumerate().map(|(index, e)| Vertex::from_str(e, index)).collect();
    points.iter().for_each(|p1| {
        points.iter().skip_while(|e| e.id <= p1.id).for_each(|p2| {
            distances.push(p1.distance_to_squared_with_ids(p2));
        })
    });
    distances.sort_by_key(|e| e.distance);

    let mut networks: Vec<Vec<&u16>> = Vec::new();

    let count = match cfg!(test) {
        true => 10,
        false => 1000
    };


    distances.iter().take(count).for_each(|distance| {
        let mut candidates = networks.iter_mut().filter(|e| e.contains(&distance.from) || e.contains(&distance.to));
        let primary = candidates.next();
        match primary {
            Some(n) => {
                if !n.contains(&distance.from) {n.push(distance.from)};
                if !n.contains(&distance.to) {n.push(distance.to)};
                for rest in candidates {
                    rest.iter().filter(|e| e != &&distance.from && e != &&distance.to).for_each(|e| n.push(e));
                    for _ in 0..(rest.len()) {
                        _ = rest.remove(0);
                    }
                }
            },
            None => {
                networks.push(vec!(distance.from, distance.to));
                return;
            }
        }
    });
    networks.sort_by_key(|e| e.len());
    Some(
        networks.iter().rev().take(3).map(|e| e.len() as u64).fold(1u64, |acc, e| acc * e)
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut edges: Vec<Edge> = Vec::with_capacity(1000000);
    let vertices: Vec<Vertex> = input.lines().enumerate().map(|(index, e)| Vertex::from_str(e, index)).collect();
    vertices.iter().for_each(|p1| {
        vertices.iter().skip_while(|e| e.id <= p1.id).for_each(|p2| {
            edges.push(p1.distance_to_squared_with_ids(p2));
        })
    });
    edges.sort_by_key(|e| e.distance);

    let mut roots = Vec::with_capacity(1000);
    let mut ranks = Vec::with_capacity(1000);

    for v in 0..vertices.len() {
        roots.push(v);
        ranks.push(0usize);
    }

    let mut result = Vec::new();
    let mut counter = 0;
    let mut edges_iter = edges.iter();

    while counter < vertices.len() - 1 {
        let next = edges_iter.next().unwrap();
        let startroot = find_root(&roots, *next.from as usize);
        let toroot = find_root(&roots, *next.to as usize);
        if startroot != toroot {
            result.push(next);
            counter += 1;
            combine(&mut roots, &mut ranks, startroot, toroot);
        }
    }
    result.sort_by_key(|e| e.distance);
    let r = result.last().unwrap();

    Some(
        vertices[*r.from as usize].x as u64 * vertices[*r.to as usize].x as u64
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
