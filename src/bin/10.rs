use good_lp::{
    Expression, Solution, SolverModel, constraint, microlp, variable, variable::ProblemVariables,
};
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
        let mut vars = Vec::new();
        let mut problem = ProblemVariables::new();
        let mut total_value: Expression = 0.into();
        for _ in &buttons {
            let var = problem.add(variable().integer().min(0));
            total_value += var;
            vars.push(var);
        }
        let mut solution = problem.minimise(total_value).using(microlp);
        for (i, joltage) in joltages.into_iter().enumerate() {
            let mut lhs: Expression = 0.into();
            let rhs: Expression = (joltage as i32).into();
            for (button_idx, button) in buttons.iter().enumerate() {
                for &indicator in button {
                    if indicator == i {
                        lhs += vars[button_idx];
                    }
                }
            }
            solution = solution.with(constraint!(lhs == rhs));
        }
        let solution = solution.solve().ok()?;

        total_presses += vars.iter().map(|&v| solution.value(v)).sum::<f64>() as u64;
    }
    Some(total_presses)
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
