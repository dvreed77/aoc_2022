use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{self, line_ending, newline, one_of},
    combinator::iterator,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    *,
};

type Point = (u32, u32);

#[derive(Debug)]
enum Cell {
    Elf,
    Ground,
}

fn parser(i: &str) -> IResult<&str, HashMap<Point, Cell>> {
    // let (input, a) = separated_list1(newline, many1(one_of(".#")))(i)?;
    let mut it = iterator(i, terminated(many1(one_of("#.")), line_ending));

    let parsed = it
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .filter_map(move |(x, v)| match v {
                    ' ' => None,
                    '.' => Some(((x as u32, y as u32), Cell::Ground)),
                    '#' => Some(((x as u32, y as u32), Cell::Elf)),
                    _ => panic!("some other character"),
                })
        })
        .collect::<HashMap<_, _>>();

    let res: IResult<_, _> = it.finish();
    res.map(|(input, _)| (input, parsed))
}
fn main() {
    println!("Hello, world!");

    let input = include_str!("../example.txt");
    let a = parser(input).unwrap().1;

    dbg!(a);
}
