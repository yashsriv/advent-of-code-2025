use std::collections::{HashMap, HashSet};

use nom::{
    IResult, Parser,
    bytes::complete::{tag, take},
    character::complete::{char, newline},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let nodes = parse_entire_input(input)?;
    Some(find_num_paths(
        &nodes,
        "you",
        "out",
        HashSet::new(),
        &vec![],
    ))
}

fn find_num_paths(
    graph: &HashMap<&str, Vec<&str>>,
    current: &str,
    end: &str,
    path: HashSet<&str>,
    required_nodes: &Vec<&str>,
) -> u64 {
    if current == end {
        if required_nodes.iter().all(|n| path.contains(n)) {
            return 1;
        } else {
            return 0;
        }
    }
    match graph.get(current) {
        None => 0,
        Some(linked_nodes) => linked_nodes
            .iter()
            .filter(|&node| !path.contains(node))
            .map(|node| {
                find_num_paths(
                    graph,
                    node,
                    end,
                    {
                        let mut cl = path.clone();
                        cl.insert(node);
                        cl
                    },
                    required_nodes,
                )
            })
            .sum(),
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let nodes = parse_entire_input(input)?;
    Some(
        find_num_paths(&nodes, "svr", "dac", HashSet::new(), &vec![])
            * find_num_paths(&nodes, "dac", "fft", HashSet::new(), &vec![])
            * find_num_paths(&nodes, "fft", "out", HashSet::new(), &vec![])
            + find_num_paths(&nodes, "svr", "fft", HashSet::new(), &vec![])
                * find_num_paths(&nodes, "fft", "dac", HashSet::new(), &vec![])
                * find_num_paths(&nodes, "dac", "out", HashSet::new(), &vec![]),
    )
}

fn parse_entire_input(input: &str) -> Option<HashMap<&str, Vec<&str>>> {
    let (_, nodes) = all_consuming(many1(terminated(
        separated_pair(
            parse_node,
            tag(": "),
            separated_list1(char(' '), parse_node),
        ),
        newline,
    )))
    .parse(input)
    .ok()?;
    let mut adjacency_list = HashMap::new();
    for (node, links) in nodes.into_iter() {
        adjacency_list.insert(node, links);
    }
    Some(adjacency_list)
}

fn parse_node(input: &str) -> IResult<&str, &str> {
    take(3usize)(input)
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
