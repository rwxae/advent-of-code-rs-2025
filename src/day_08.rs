use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance(&self, other: &Self) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}

type Graph<'a> = HashMap<&'a Point, Vec<&'a Point>>;

fn get_point_stats<'a>(graph: &'a Graph, seen: &mut HashSet<&'a Point>, point: &'a Point) -> u64 {
    if seen.contains(point) {
        return 0;
    }
    seen.insert(point);
    1 + graph
        .get(point)
        .unwrap()
        .iter()
        .map(|p| get_point_stats(graph, seen, p))
        .sum::<u64>()
}

fn get_stats(graph: &Graph) -> impl Iterator<Item = u64> {
    let mut seen: HashSet<&Point> = HashSet::new();
    graph.keys().filter_map(move |point| {
        if seen.contains(point) {
            None
        } else {
            Some(get_point_stats(graph, &mut seen, point))
        }
    })
}

fn has_point<'a>(
    graph: &'a Graph,
    seen: &mut HashSet<&'a Point>,
    current: &'a Point,
    point: &'a Point,
) -> bool {
    seen.insert(current);
    if current == point {
        return true;
    }
    graph.get(current).unwrap().iter().any(|candidate| {
        if seen.contains(candidate) {
            return false;
        }
        has_point(graph, seen, candidate, point)
    })
}

fn is_connected(graph: &Graph, a: &Point, b: &Point) -> bool {
    has_point(graph, &mut HashSet::new(), a, b)
}

pub fn solution_1(input: &str) -> u64 {
    let points = input
        .trim()
        .split_whitespace()
        .map(|line| {
            let mut line = line
                .split(',')
                .map(|v| v.parse::<i64>().expect("Coordinate must be an integer"));
            let x = line.next().expect("X coordinate");
            let y = line.next().expect("Y coordinate");
            let z = line.next().expect("Z coordinate");
            Point { x, y, z }
        })
        .collect::<Vec<_>>();

    // In the minimal example there are only 10 connections
    // With actual data, there should be 1000.
    let limit = if points.len() > 20 { 1000 } else { 10 };

    let mut pairs = points
        .iter()
        .enumerate()
        .map(|(i, point)| {
            points[i + 1..]
                .iter()
                .map(|point2| (point, point2))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    pairs.sort_by(|a, b| a.0.distance(a.1).total_cmp(&b.0.distance(b.1)));

    let mut graph: HashMap<&Point, Vec<&Point>> = HashMap::new();
    for (a, b) in &pairs[..limit] {
        if let (Some(_), Some(_)) = (graph.get(a), graph.get(b)) {
            if is_connected(&graph, a, b) {
                continue;
            }
            graph.entry(a).and_modify(|v| v.push(b));
            graph.entry(b).and_modify(|v| v.push(a));
        } else {
            graph.entry(a).and_modify(|v| v.push(b)).or_insert(vec![b]);
            graph.entry(b).and_modify(|v| v.push(a)).or_insert(vec![a]);
        }
    }

    let mut stats = get_stats(&graph).collect::<Vec<_>>();
    stats.sort();
    stats.iter().rev().take(3).product()
}

pub fn solution_2(input: &str) -> i64 {
    let points = input
        .trim()
        .split_whitespace()
        .map(|line| {
            let mut line = line
                .split(',')
                .map(|v| v.parse::<i64>().expect("Coordinate must be an integer"));
            let x = line.next().expect("X coordinate");
            let y = line.next().expect("Y coordinate");
            let z = line.next().expect("Z coordinate");
            Point { x, y, z }
        })
        .collect::<Vec<_>>();

    let mut pairs = points
        .iter()
        .enumerate()
        .map(|(i, point)| {
            points[i + 1..]
                .iter()
                .map(|point2| (point, point2))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    pairs.sort_by(|a, b| a.0.distance(a.1).total_cmp(&b.0.distance(b.1)));

    let mut graph: HashMap<&Point, Vec<&Point>> = HashMap::new();
    let mut last_connection: Option<(&Point, &Point)> = None;
    let mut left_connections = points.len() - 1;

    for (a, b) in pairs {
        if left_connections == 0 {
            break;
        }
        if let (Some(_), Some(_)) = (graph.get(a), graph.get(b)) {
            if is_connected(&graph, a, b) {
                continue;
            }
            graph.entry(a).and_modify(|v| v.push(b));
            graph.entry(b).and_modify(|v| v.push(a));
        } else {
            graph.entry(a).and_modify(|v| v.push(b)).or_insert(vec![b]);
            graph.entry(b).and_modify(|v| v.push(a)).or_insert(vec![a]);
        }
        left_connections -= 1;
        last_connection.replace((a, b));
    }

    last_connection.map(|(a, b)| a.x * b.x).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve;

    #[test]
    fn test_1_easy() {
        assert_eq!(solve(8, 1, solution_1), 40);
    }

    #[test]
    fn test_1_hard() {
        assert_eq!(solve(8, 2, solution_1), 29406);
    }

    #[test]
    fn test_2_easy() {
        assert_eq!(solve(8, 1, solution_2), 25272);
    }

    #[test]
    fn test_2_hard() {
        assert_eq!(solve(8, 2, solution_2), 7499461416);
    }
}
