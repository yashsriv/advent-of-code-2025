advent_of_code::solution!(3);

use nom::{
    IResult, Parser,
    character::complete::{newline, one_of},
    combinator::map_res,
    error::Error,
    multi::{many1, separated_list0},
};

type Bank = Vec<u32>;

pub fn part_one(input: &str) -> Option<u64> {
    let mut joltage: u64 = 0;
    let (_, banks) = parse_entire_input(input).ok()?;
    const MAX_BATTERY_CAPACITY: usize = 2;
    for bank in banks {
        let mut max_joltage = [0; MAX_BATTERY_CAPACITY];
        let bank_len = bank.len();
        for (index, value) in bank.into_iter().enumerate() {
            for (slot, battery) in max_joltage.iter().enumerate() {
                if value > *battery && bank_len - index > MAX_BATTERY_CAPACITY - slot - 1 {
                    max_joltage[slot] = value;
                    for b in max_joltage.iter_mut().skip(slot + 1) {
                        *b = 0;
                    }
                    break;
                }
            }
        }
        let mut max_joltage_value = 0;
        for battery in max_joltage {
            max_joltage_value = max_joltage_value * 10 + battery;
        }
        joltage += max_joltage_value as u64;
    }
    Some(joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut joltage: u64 = 0;
    let (_, banks) = parse_entire_input(input).ok()?;
    const MAX_BATTERY_CAPACITY: usize = 12;
    for bank in banks {
        let mut max_joltage = [0; MAX_BATTERY_CAPACITY];
        let bank_len = bank.len();
        for (index, value) in bank.into_iter().enumerate() {
            for (slot, battery) in max_joltage.iter().enumerate() {
                if value > *battery && bank_len - index > MAX_BATTERY_CAPACITY - slot - 1 {
                    max_joltage[slot] = value;
                    // Set all subsequent slots to 0
                    for b in max_joltage.iter_mut().skip(slot + 1) {
                        *b = 0;
                    }
                    break;
                }
            }
        }
        let mut max_joltage_value = 0;
        for battery in max_joltage {
            max_joltage_value = max_joltage_value * 10 + (battery as u64);
        }
        joltage += max_joltage_value;
    }
    Some(joltage)
}

fn parse_entire_input(input: &str) -> IResult<&str, Vec<Bank>> {
    separated_list0(newline, parse_single_line).parse(input)
}

fn parse_single_line(input: &str) -> IResult<&str, Bank> {
    many1(map_res(one_of("0123456789"), |c| {
        c.to_digit(10).ok_or(Error::new(
            "unable to parse digit",
            nom::error::ErrorKind::Digit,
        ))
    }))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
