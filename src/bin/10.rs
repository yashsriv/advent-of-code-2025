use std::collections::HashMap;

use nom::{
    IResult, Parser,
    character::complete::{char, digit1, newline, one_of},
    combinator::{all_consuming, map_res},
    multi::{many1, separated_list1},
    sequence::{delimited, terminated},
};

advent_of_code::solution!(10);

type Row1 = (u64, Vec<u64>, Vec<usize>);
type Row2 = (u64, Vec<Button2>, Vec<usize>);
type Button2 = Vec<usize>;

pub fn part_one(input: &str) -> Option<u64> {
    let rows = parse_entire_input(input)?;
    let mut total_presses = 0;
    for (target, buttons, _) in rows {
        let presses = recurse_row(0, target, 0, &buttons);
        total_presses += presses;
    }
    Some(total_presses as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rows = parse_entire_input_2(input)?;
    let mut total_presses = 0;
    for (_, buttons, joltages) in rows {
        let presses = recurse_row_2(joltages, &buttons, &mut HashMap::new());
        total_presses += presses;
    }
    Some(total_presses as u64)
}

fn recurse_row(
    current_val: u64,
    target_val: u64,
    press_count: usize,
    remaining_buttons: &[u64],
) -> usize {
    if current_val == target_val {
        return press_count;
    }
    if remaining_buttons.is_empty() {
        return usize::MAX;
    }
    let mut min_presses = usize::MAX;
    for (i, &button) in remaining_buttons.iter().enumerate() {
        let new_val = current_val ^ button;
        let presses = recurse_row(
            new_val,
            target_val,
            press_count + 1,
            &remaining_buttons[i + 1..],
        );
        if presses < min_presses {
            min_presses = presses;
        }
    }
    min_presses
}

fn recurse_row_2(
    current_val: Vec<usize>,
    remaining_buttons: &[Vec<usize>],
    precomp: &mut HashMap<String, usize>,
) -> usize {
    if current_val.iter().all(|&v| v == 0) {
        return 0;
    }
    if remaining_buttons.is_empty() {
        return usize::MAX;
    }

    let hash_string = format!("{:?}:{}", current_val, remaining_buttons.len());
    if let Some(&min_presses) = precomp.get(&hash_string) {
        return min_presses;
    }

    let mut min_presses = usize::MAX;
    let mut current_button_count = 0;
    let mut current_val = current_val;
    loop {
        let presses = recurse_row_2(current_val.clone(), &remaining_buttons[1..], precomp);
        if presses != usize::MAX && (presses + current_button_count) < min_presses {
            min_presses = presses + current_button_count;
        }

        let mut next_loop_valid = true;
        for &index in remaining_buttons
            .first()
            .expect("checked remaining buttons is not empty above")
        {
            if current_val[index] == 0 {
                next_loop_valid = false;
                break;
            }

            current_val[index] -= 1;
        }
        if !next_loop_valid {
            break;
        }
        current_button_count += 1;
    }

    precomp.insert(hash_string, min_presses);
    min_presses
}

fn parse_entire_input(input: &str) -> Option<Vec<Row1>> {
    let (_, rows) = all_consuming(many1(terminated(parse_row, newline)))
        .parse(input)
        .ok()?;
    Some(rows)
}

fn parse_entire_input_2(input: &str) -> Option<Vec<Row2>> {
    let (_, rows) = all_consuming(many1(terminated(parse_row_2, newline)))
        .parse(input)
        .ok()?;
    Some(rows)
}

fn parse_row_2(input: &str) -> IResult<&str, Row2> {
    (parse_diagram, parse_buttons_2, parse_joltages).parse(input)
}

fn parse_row(input: &str) -> IResult<&str, (u64, Vec<u64>, Vec<usize>)> {
    (parse_diagram, parse_buttons, parse_joltages).parse(input)
}

fn parse_diagram(input: &str) -> IResult<&str, u64> {
    delimited(
        char('['),
        many1(one_of(".#")).map(|v| {
            v.iter().rev().fold(0, |acc, v| match v {
                '.' => acc * 2,
                '#' => acc * 2 + 1,
                _ => acc,
            })
        }),
        char(']'),
    )
    .parse(input)
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(
        char(' '),
        separated_list1(char(' '), parse_button),
        char(' '),
    )
    .parse(input)
}

fn parse_buttons_2(input: &str) -> IResult<&str, Vec<Button2>> {
    delimited(
        char(' '),
        separated_list1(char(' '), parse_button_2),
        char(' '),
    )
    .parse(input)
}

fn parse_joltages(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        char('{'),
        separated_list1(char(','), decimal_value),
        char('}'),
    )
    .parse(input)
}

fn parse_button(input: &str) -> IResult<&str, u64> {
    delimited(
        char('('),
        separated_list1(char(','), decimal_value)
            .map(|v| v.into_iter().fold(0, |acc, n| acc | (1 << n))),
        char(')'),
    )
    .parse(input)
}

fn parse_button_2(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        char('('),
        separated_list1(char(','), decimal_value),
        char(')'),
    )
    .parse(input)
}

fn decimal_value(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
