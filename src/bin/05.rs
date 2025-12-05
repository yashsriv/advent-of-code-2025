use nom::{
    IResult, Parser,
    character::complete::{char, digit1, newline},
    combinator::{all_consuming, map_res},
    multi::many0,
    sequence::{separated_pair, terminated},
};

advent_of_code::solution!(5);

type Range = (u64, u64);

fn merge_ranges(ranges: Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() {
        return vec![];
    }
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort();
    let mut merged: Vec<(u64, u64)> = vec![];
    let mut current_range = sorted_ranges[0];
    for range in sorted_ranges.into_iter().skip(1) {
        if range.0 <= current_range.1 + 1 {
            current_range.1 = current_range.1.max(range.1);
        } else {
            merged.push(current_range);
            current_range = range;
        }
    }
    merged.push(current_range);
    merged
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (ranges, ids)) = parse_entire_input(input).ok()?;

    let merged_ranges = merge_ranges(ranges);
    let mut counter: u64 = 0;
    for id in ids {
        let search_result = merged_ranges.binary_search_by(|probe| {
            if probe.1 < id {
                std::cmp::Ordering::Less
            } else if probe.0 > id {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        if search_result.is_ok() {
            counter += 1;
        }
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (ranges, _)) = parse_entire_input(input).ok()?;

    let merged_ranges = merge_ranges(ranges);
    let mut counter: u64 = 0;
    for range in merged_ranges {
        counter += range.1 - range.0 + 1;
    }
    Some(counter)
}

fn parse_entire_input(input: &str) -> IResult<&str, (Vec<Range>, Vec<u64>)> {
    all_consuming(separated_pair(
        many0(parse_ranges),
        newline,
        many0(parse_ids),
    ))
    .parse(input)
}

fn parse_ranges(input: &str) -> IResult<&str, Range> {
    terminated(
        separated_pair(parse_number, char('-'), parse_number),
        newline,
    )
    .parse(input)
}

fn parse_ids(input: &str) -> IResult<&str, u64> {
    terminated(parse_number, newline).parse(input)
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
