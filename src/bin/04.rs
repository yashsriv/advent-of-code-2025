use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, newline},
    combinator::{all_consuming, map_opt},
    multi::{many1, separated_list0},
};

advent_of_code::solution!(4);

const DIRECTION: [(isize, isize); 8] = [
    (0, -1),  // Left
    (0, 1),   // Right
    (1, -1),  // Down-Left
    (1, 0),   // Down
    (1, 1),   // Down-Right
    (-1, -1), // Up-Left
    (-1, 0),  // Up
    (-1, 1),  // Up-Right
];

pub fn part_one(input: &str) -> Option<u64> {
    let (_, grid) = parse_entire_input(input).ok()?;

    let mut counter = 0;
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if !cell {
                continue;
            }
            let mut cell_counter = 0;
            for (dr, dc) in DIRECTION {
                let r = row_index as isize + dr;
                let c = col_index as isize + dc;
                if r < 0 || c < 0 || r >= grid.len() as isize || c >= row.len() as isize {
                    continue;
                }
                if grid[r as usize][c as usize] {
                    cell_counter += 1;
                }
            }
            if cell_counter < 4 {
                counter += 1;
            }
        }
    }

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, mut grid) = parse_entire_input(input).ok()?;

    let mut counter = 0;

    loop {
        let previous_grid = grid.clone();
        let mut changed = false;
        for (row_index, row) in grid.iter_mut().enumerate() {
            for (col_index, cell) in row.iter_mut().enumerate() {
                if !*cell {
                    continue;
                }
                let mut cell_counter = 0;
                for (dr, dc) in DIRECTION {
                    let r = row_index as isize + dr;
                    let c = col_index as isize + dc;
                    if r < 0
                        || c < 0
                        || r >= previous_grid.len() as isize
                        || c >= previous_grid[0].len() as isize
                    {
                        continue;
                    }
                    if previous_grid[r as usize][c as usize] {
                        cell_counter += 1;
                    }
                }
                if cell_counter < 4 {
                    counter += 1;
                    *cell = false;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    Some(counter)
}

fn parse_entire_input(input: &str) -> IResult<&str, Vec<Vec<bool>>> {
    all_consuming(separated_list0(newline, parse_single_line)).parse(input)
}

fn parse_single_line(input: &str) -> IResult<&str, Vec<bool>> {
    many1(map_opt(alt((char('.'), char('@'))), |c| match c {
        '.' => Some(false),
        '@' => Some(true),
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
