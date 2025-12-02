use nom::{
    IResult, Parser,
    character::complete::{char, digit1},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut counter: u64 = 0;
    let (_, ranges) = parse_entire_input(input).ok()?;
    for range in ranges {
        for n in range.0..=range.1 {
            if is_invalid_id(n) {
                counter += n;
            }
        }
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut counter: u64 = 0;
    let (_, ranges) = parse_entire_input(input).ok()?;
    for range in ranges {
        for n in range.0..=range.1 {
            if is_invalid_id2(n) {
                counter += n;
            }
        }
    }
    Some(counter)
}

fn is_invalid_id(id: u64) -> bool {
    let num_digits = id.checked_ilog10().unwrap_or(0) + 1;
    if !num_digits.is_multiple_of(2) {
        return false;
    }
    let div = 10u64.pow(num_digits / 2);
    let left = id / div;
    let right = id % div;
    left == right
}

fn is_invalid_id2(id: u64) -> bool {
    let num_digits = id.checked_ilog10().unwrap_or(0) + 1;
    for num_chunks in 2..=num_digits {
        if !num_digits.is_multiple_of(num_chunks) {
            continue;
        }
        let chunk_size = num_digits / num_chunks;
        let div = 10u64.pow(chunk_size);
        let mut all_equal = true;
        let first_chunk = id % div;
        for chunk_index in 1..num_chunks {
            let chunk = (id / 10u64.pow(chunk_size * chunk_index)) % div;
            if chunk != first_chunk {
                all_equal = false;
                break;
            }
        }
        if all_equal {
            return true;
        }
    }
    false
}

fn parse_entire_input(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list0(char(','), parse_single_line).parse(input)
}

fn parse_single_line(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(decimal_value, char('-'), decimal_value).parse(input)
}

fn decimal_value(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |out| str::replace(out, "_", "").parse::<u64>()).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
