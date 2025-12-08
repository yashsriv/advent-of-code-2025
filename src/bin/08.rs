use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use nom::{
    IResult, Parser,
    character::complete::{char, digit1, newline},
    combinator::{all_consuming, map_res},
    multi::{many1, separated_list1},
    sequence::terminated,
};

advent_of_code::solution!(8);

#[derive(Debug)]
struct Location {
    x: u64,
    y: u64,
    z: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let locations = parse_entire_input(input)?;
    let mut distances = Vec::new();
    for (i, loc) in locations.iter().enumerate() {
        for (j, other_loc) in locations.iter().enumerate().skip(i + 1) {
            let distance = (((loc.x as i64 - other_loc.x as i64).pow(2)
                + (loc.y as i64 - other_loc.y as i64).pow(2)
                + (loc.z as i64 - other_loc.z as i64).pow(2)) as f64)
                .sqrt();
            distances.push((distance, i, j));
        }
    }
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut graph = vec![vec![false; locations.len()]; locations.len()];

    let max_connections = if cfg!(test) { 10 } else { 1000 };
    for k in 0..max_connections {
        graph[distances[k].1][distances[k].2] = true;
        graph[distances[k].2][distances[k].1] = true;
    }
    let mut size_counts = Vec::new();
    let mut visited = HashSet::new();
    for i in 0..locations.len() {
        if visited.contains(&i) {
            continue;
        }
        visited.insert(i);
        let count = recursive_dfs(i, &graph, &mut visited);
        size_counts.push(count);
    }
    size_counts.sort();
    size_counts.iter().rev().take(3).product::<u64>().into()
}

fn recursive_dfs(node: usize, graph: &Vec<Vec<bool>>, visited: &mut HashSet<usize>) -> u64 {
    let row = &graph[node];
    if row.iter().all(|&b| !b) {
        1
    } else {
        let mut total = 1;
        for (i, &connected) in row.iter().enumerate() {
            if connected && !visited.contains(&i) {
                visited.insert(i);
                total += recursive_dfs(i, graph, visited);
            }
        }
        total
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let locations = parse_entire_input(input)?;
    let mut distances = Vec::new();
    for (i, loc) in locations.iter().enumerate() {
        for (j, other_loc) in locations.iter().enumerate().skip(i + 1) {
            let distance = (((loc.x as i64 - other_loc.x as i64).pow(2)
                + (loc.y as i64 - other_loc.y as i64).pow(2)
                + (loc.z as i64 - other_loc.z as i64).pow(2)) as f64)
                .sqrt();
            distances.push((distance, i, j));
        }
    }
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut graphs = HashMap::<usize, Rc<RefCell<HashSet<usize>>>>::new();
    for (_, i, j) in distances {
        let gi = graphs.get(&i);
        let gj = graphs.get(&j);
        match (gi, gj) {
            (Some(gi), Some(gj)) => {
                let new_set = gi
                    .borrow()
                    .union(&gj.borrow())
                    .cloned()
                    .collect::<HashSet<usize>>();
                let new_set = Rc::new(RefCell::new(new_set));
                for &val in new_set.borrow().iter() {
                    graphs.insert(val, new_set.clone());
                }
                if new_set.borrow().len() == locations.len() {
                    return Some(locations[i].x * locations[j].x);
                }
            }
            (Some(_), None) => {
                {
                    graphs.get(&i)?.borrow_mut().insert(j);
                }
                graphs.insert(j, graphs.get(&i)?.clone());
                if graphs.get(&i)?.borrow().len() == locations.len() {
                    return Some(locations[i].x * locations[j].x);
                }
            }
            (None, Some(_)) => {
                {
                    graphs.get(&j)?.borrow_mut().insert(i);
                }
                graphs.insert(i, graphs.get(&j)?.clone());
                if graphs.get(&j)?.borrow().len() == locations.len() {
                    return Some(locations[i].x * locations[j].x);
                }
            }
            (None, None) => {
                let mut new_set = HashSet::new();
                new_set.insert(i);
                new_set.insert(j);
                let new_set = Rc::new(RefCell::new(new_set));
                graphs.insert(i, new_set.clone());
                graphs.insert(j, new_set.clone());
                if new_set.borrow().len() == locations.len() {
                    return Some(locations[i].x * locations[j].x);
                }
            }
        }
    }
    None
}

fn parse_entire_input(input: &str) -> Option<Vec<Location>> {
    let (_, locations) = all_consuming(many1(terminated(
        separated_list1(char(','), decimal_value),
        newline,
    )))
    .parse(input)
    .ok()?;
    Some(
        locations
            .into_iter()
            .map(|vals| Location {
                x: vals.first().cloned().unwrap_or(0),
                y: vals.get(1).cloned().unwrap_or(0),
                z: vals.get(2).cloned().unwrap_or(0),
            })
            .collect(),
    )
}

fn decimal_value(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse).parse(input)
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
