use itertools::Itertools;
use itertools::Position::*;
use std::{collections::HashMap, fmt::Display};

use nom::{
    branch::alt,
    character::complete::{self, line_ending, one_of},
    combinator::iterator,
    multi::many1,
    sequence::{separated_pair, terminated},
    *,
};

type Point = (u32, u32);

#[derive(Debug)]
enum Cell {
    Floor,
    Wall,
}

#[derive(Debug)]
enum Move {
    Paces(u32),
    Turn(Turn),
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self, turn: &Turn) -> Direction {
        use Direction::*;
        match (self, turn) {
            (Up, Turn::Left) => Left,
            (Up, Turn::Right) => Right,
            (Down, Turn::Left) => Right,
            (Down, Turn::Right) => Left,
            (Left, Turn::Left) => Down,
            (Left, Turn::Right) => Up,
            (Right, Turn::Left) => Up,
            (Right, Turn::Right) => Down,
        }
    }
}
#[derive(Debug)]
enum Turn {
    Left,
    Right,
}
#[derive(Debug)]
struct Field(HashMap<Point, Cell>);

impl Field {
    fn get_row(&self, target_y: u32) -> Vec<(&Point, &Cell)> {
        self.0
            .iter()
            .filter(|((_, y), _cell)| y == &target_y)
            .sorted_by(|(vec_a, _), (vec_b, _)| vec_a.0.cmp(&vec_b.0))
            .collect()
    }
    fn get_column(&self, target_x: u32) -> Vec<(&Point, &Cell)> {
        self.0
            .iter()
            .filter(|((x, _), _cell)| x == &target_x)
            .sorted_by(|(vec_a, _), (vec_b, _)| vec_a.1.cmp(&vec_b.1))
            .collect()
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let field_string = self
            .0
            .iter()
            .sorted_by(|((x1, y1), _), ((x2, y2), _)| (y1, x1).cmp(&(y2, x2)))
            .group_by(|((_, y), _)| *y)
            .into_iter()
            .map(|(_y, xs)| {
                let mut padding = "".to_string();
                let line: String = xs
                    .into_iter()
                    .with_position()
                    .map(|position| {
                        let cell = match position {
                            First(((x, y), cell)) => {
                                padding = " ".repeat(*x as usize);
                                cell
                            }
                            Middle((_, cell)) => cell,
                            Last((_, cell)) => cell,
                            Only((_, cell)) => {
                                panic!("only")
                            }
                        };

                        match cell {
                            Cell::Floor => ".",
                            Cell::Wall => "#",
                        }
                    })
                    .collect();
                format!("{padding}{line}")
            })
            .join("\n");
        write!(f, "{}", field_string)
    }
}

fn map_parser(i: &str) -> IResult<&str, Field> {
    let mut it = iterator(i, terminated(many1(one_of(" #.")), line_ending));

    let parsed = it
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .filter_map(move |(x, v)| match v {
                    ' ' => None,
                    '.' => Some(((x as u32, y as u32), Cell::Floor)),
                    '#' => Some(((x as u32, y as u32), Cell::Wall)),
                    _ => panic!("some other character"),
                })
        })
        .collect::<HashMap<_, _>>();

    let res: IResult<_, _> = it.finish();
    res.map(|(input, _)| (input, Field(parsed)))
}

fn move_parser(input: &str) -> IResult<&str, Vec<Move>> {
    many1(alt((
        complete::u32.map(|num| Move::Paces(num)),
        alt((
            complete::char('L').map(|_| Move::Turn(Turn::Left)),
            complete::char('R').map(|_| Move::Turn(Turn::Right)),
        )),
    )))(input)
}

fn map_and_moves_parser(input: &str) -> IResult<&str, (Field, Vec<Move>)> {
    separated_pair(map_parser, line_ending, move_parser)(input)
}

fn walk<'a>(
    current_position: &mut Point,
    paces_to_move: u32,
    positions: impl Iterator<Item = &'a (&'a Point, &'a Cell)> + Clone,
) {
    let current_index = positions
        .clone()
        .position(|(vec, _)| vec == &current_position)
        .unwrap();
    let mut it = positions.cycle();
    it.nth(current_index);

    for _ in 1..=paces_to_move {
        let next_cell = it.next().unwrap();
        if let Cell::Wall = next_cell.1 {
            break;
        } else {
            current_position.0 = next_cell.0 .0;
            current_position.1 = next_cell.0 .1;
        }
    }
}

fn main() {
    // let input = include_str!("../example.txt");
    let input = include_str!("../input.txt");

    let (field, moves) = map_and_moves_parser(input).unwrap().1;

    let mut facing = Direction::Right;

    let starting_position = field
        .0
        .iter()
        .sorted_by(|((x1, y1), _), ((x2, y2), _)| (y1, x1).cmp(&(y2, x2)))
        .next()
        .unwrap();

    // dbg!(starting_position);
    let mut current_position = *starting_position.0;

    for m in moves {
        match m {
            Move::Paces(paces) => match facing {
                Direction::Up => {
                    let x = current_position.0.clone();
                    walk(
                        &mut current_position,
                        paces,
                        field.get_column(x).iter().rev(),
                    );
                }
                Direction::Down => {
                    let x = current_position.0.clone();
                    walk(&mut current_position, paces, field.get_column(x).iter());
                }
                Direction::Left => {
                    let y = current_position.1.clone();
                    walk(&mut current_position, paces, field.get_row(y).iter().rev());
                }
                Direction::Right => {
                    let y = current_position.1.clone();
                    walk(&mut current_position, paces, field.get_row(y).iter());
                }
            },
            Move::Turn(turn) => {
                facing = facing.turn(&turn);
            }
        }
    }

    let out = (1000 * (current_position.1 + 1)
        + 4 * (current_position.0 + 1)
        + match facing {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        })
    .to_string();

    dbg!(current_position.0, current_position.1, out);
}

// 8036, too low
