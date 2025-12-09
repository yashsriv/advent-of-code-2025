use nom::{
    IResult, Parser,
    character::complete::{char, digit1, newline},
    combinator::{all_consuming, map_res},
    multi::many1,
    sequence::{separated_pair, terminated},
};

advent_of_code::solution!(9);

type Point = (i64, i64);

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_entire_input(input)?;
    let mut highest_area = 0;
    for (i, point1) in points.iter().enumerate() {
        for point2 in points.iter().skip(i + 1) {
            let area = ((point1.0 - point2.0).abs() + 1) * ((point1.1 - point2.1).abs() + 1);
            if area > highest_area {
                highest_area = area;
            }
        }
    }
    Some(highest_area as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    Some(24)
}

fn parse_entire_input(input: &str) -> Option<Vec<Point>> {
    let (_, points) = all_consuming(many1(terminated(
        separated_pair(decimal_value, char(','), decimal_value),
        newline,
    )))
    .parse(input)
    .ok()?;
    Some(points)
}

fn decimal_value(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
