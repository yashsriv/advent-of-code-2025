use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, digit1, newline, one_of, space0, space1},
    combinator::{all_consuming, map_opt, map_res, opt, peek},
    multi::{many_m_n, many0_count, many1, separated_list1},
    sequence::{delimited, pair, terminated},
};

advent_of_code::solution!(6);

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (numbers, operations)) = parse_entire_input(input).ok()?;
    Some(
        numbers
            .iter()
            .skip(1)
            .fold(numbers[0].clone(), |acc, row| {
                let mut row_result = Vec::with_capacity(operations.len());
                for ((num1, num2), op) in row.iter().zip(acc.iter()).zip(operations.iter()) {
                    row_result.push(match op {
                        Operation::Add => num1 + num2,
                        Operation::Multiply => num1 * num2,
                    })
                }
                row_result
            })
            .iter()
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    parse_entire_input_2(input)
}

fn parse_entire_input(input: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<Operation>)> {
    all_consuming(pair(many1(parse_spaced_numbers), parse_operations)).parse(input)
}

fn parse_entire_input_2(input: &str) -> Option<u64> {
    let input_by_lines = input.lines().collect::<Vec<_>>();
    let mut operators_line = input_by_lines.last()?.to_string();
    // Add a space at the end to help parsing
    operators_line.push(' ');
    let (_, operations) = separated_list1(space1, (parse_operation, peek(many0_count(char(' ')))))
        .parse(&operators_line)
        .ok()?;

    let size_needed = input_by_lines.len() - 1;
    let mut all_lines = Vec::with_capacity(size_needed);
    for line in input_by_lines.into_iter().take(size_needed) {
        let mut current_line = line;
        let mut numbers_lines = Vec::with_capacity(operations.len());
        for (_, max_size) in operations.iter() {
            let (new_line, mut numbers) =
                terminated(many_m_n(1, *max_size, parse_number_2), opt(char(' ')))
                    .parse(current_line)
                    .ok()?;
            while numbers.len() < *max_size {
                numbers.push(None);
            }
            current_line = new_line;
            numbers_lines.push(numbers);
        }
        all_lines.push(numbers_lines);
    }

    let mut total_val = 0;
    for (col_index, (op, size)) in operations.iter().enumerate() {
        let mut col_value = match op {
            Operation::Add => 0,
            Operation::Multiply => 1,
        };
        for n in 0..*size {
            let mut val = 0;
            for row in all_lines.iter() {
                if let Some(d) = row[col_index][size - n - 1] {
                    val = val * 10 + d;
                }
            }
            match op {
                Operation::Add => col_value += val,
                Operation::Multiply => col_value *= val,
            }
        }
        total_val += col_value;
    }
    Some(total_val)
}

fn parse_number_2(input: &str) -> IResult<&str, Option<u64>> {
    one_of("0123456789 ")
        .map(|c| match c {
            '1' => Some(1),
            '2' => Some(2),
            '3' => Some(3),
            '4' => Some(4),
            '5' => Some(5),
            '6' => Some(6),
            '7' => Some(7),
            '8' => Some(8),
            '9' => Some(9),
            _ => None,
        })
        .parse(input)
}

fn parse_spaced_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(
        space0,
        separated_list1(space1, parse_number),
        (space0, newline),
    )
    .parse(input)
}

fn parse_operations(input: &str) -> IResult<&str, Vec<Operation>> {
    delimited(
        space0,
        separated_list1(space1, parse_operation),
        (space0, newline),
    )
    .parse(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    map_opt(alt((char('*'), char('+'))), |c| match c {
        '*' => Some(Operation::Multiply),
        '+' => Some(Operation::Add),
        _ => None,
    })
    .parse(input)
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
