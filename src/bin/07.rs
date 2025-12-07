use std::collections::{HashMap, HashSet, VecDeque};

use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, newline},
    combinator::all_consuming,
    multi::many1,
    sequence::terminated,
};

advent_of_code::solution!(7);

enum State {
    Start,
    EmptySpace,
    Splitter,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Start => "S",
                State::EmptySpace => ".",
                State::Splitter => "^",
            }
        )
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let manifold = parse_entire_input(input)?;

    let start_index = manifold
        .first()?
        .iter()
        .position(|s| matches!(s, State::Start))?;

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((0usize, start_index));
    visited.insert((0usize, start_index));

    let mut split_count = 0;

    while !queue.is_empty() {
        let (row, col) = queue.pop_front()?;
        if row == manifold.len() - 1 {
            continue;
        }
        let next_row = row + 1;
        let next_state = &manifold[next_row][col];
        match next_state {
            State::EmptySpace => {
                if visited.insert((next_row, col)) {
                    queue.push_back((next_row, col));
                }
            }
            State::Splitter => {
                split_count += 1;
                if visited.insert((next_row, col - 1)) {
                    queue.push_back((next_row, col - 1));
                }
                if visited.insert((next_row, col + 1)) {
                    queue.push_back((next_row, col + 1));
                }
            }
            _ => {}
        }
    }

    Some(split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let manifold = parse_entire_input(input)?;

    let start_index = manifold
        .first()?
        .iter()
        .position(|s| matches!(s, State::Start))?;

    let path_count = recursive_dfs((0usize, start_index), &manifold, &mut HashMap::new());

    Some(path_count)
}

fn recursive_dfs(
    node: (usize, usize),
    manifold: &Vec<Vec<State>>,
    visited: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    let (row, col) = node;
    if row == manifold.len() - 1 {
        visited.insert(node, 1);
        return 1;
    }
    let next_row = row + 1;
    let next_state = &manifold[next_row][col];
    let val = match next_state {
        State::EmptySpace => match visited.get(&(next_row, col)) {
            Some(&val) => val,
            None => recursive_dfs((next_row, col), manifold, visited),
        },
        State::Splitter => {
            let left_val = match visited.get(&(next_row, col - 1)) {
                Some(&val) => val,
                None => recursive_dfs((next_row, col - 1), manifold, visited),
            };
            let right_val = match visited.get(&(next_row, col + 1)) {
                Some(&val) => val,
                None => recursive_dfs((next_row, col + 1), manifold, visited),
            };
            left_val + right_val
        }
        _ => 0,
    };
    visited.insert(node, val);
    val
}

fn parse_entire_input(input: &str) -> Option<Vec<Vec<State>>> {
    let (_, manifold) = all_consuming(many1(terminated(parse_row, newline)))
        .parse(input)
        .ok()?;
    Some(manifold)
}

fn parse_row(input: &str) -> IResult<&str, Vec<State>> {
    many1(alt((char('S'), char('.'), char('^'))).map_opt(|c| match c {
        'S' => Some(State::Start),
        '.' => Some(State::EmptySpace),
        '^' => Some(State::Splitter),
        _ => None,
    }))
    .parse(input)
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
