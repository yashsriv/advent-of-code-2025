use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, digit1, newline},
    combinator::{all_consuming, map_res},
    multi::separated_list0,
};

advent_of_code::solution!(1);

type Rotation = (i32, u64);

pub fn part_one(input: &str) -> Option<u64> {
    let mut val: i32 = 50;
    let mut counter: u64 = 0;
    let (_, rotations) = parse_entire_input(input).ok()?;
    for (dir, clicks) in rotations {
        for _ in 0..clicks {
            val += dir;
            if val == 100 {
                val = 0;
            } else if val == -1 {
                val = 99;
            }
        }
        if val == 0 {
            counter += 1;
        }
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut val: i32 = 50;
    let mut counter: u64 = 0;
    let (_, rotations) = parse_entire_input(input).ok()?;
    for (dir, clicks) in rotations {
        for _ in 0..clicks {
            val += dir;
            if val == 100 {
                val = 0;
            } else if val == -1 {
                val = 99;
            }
            if val == 0 {
                counter += 1;
            }
        }
    }
    Some(counter)
}

fn parse_entire_input(input: &str) -> IResult<&str, Vec<Rotation>> {
    all_consuming(separated_list0(newline, parse_single_line)).parse(input)
}

fn parse_single_line(input: &str) -> IResult<&str, Rotation> {
    let (input, dir) = alt((char('L'), char('R'))).parse(input)?;
    let (input, number) = decimal_value(input)?;

    match dir {
        'L' => Ok((input, (-1, number))),
        'R' => Ok((input, (1, number))),
        _ => Ok((input, (0, number))), // unreachable
    }
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
