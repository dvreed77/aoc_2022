use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    combinator::{eof, iterator},
    multi::many1,
    sequence::terminated,
    *,
};

use petgraph::{algo::dijkstra, prelude::DiGraphMap};

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Wall,
    Floor,
    Blizzard(Vec<Direction>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

type Point = (i32, i32);

#[derive(Debug)]
struct Field(HashMap<Point, Cell>);

impl Field {
    fn get_inner_dimensions(&self) -> Point {
        let x_max = self.0.iter().map(|(v, _)| v.0).max().unwrap();
        let y_max = self.0.iter().map(|(v, _)| v.1).max().unwrap();
        (x_max - 1, y_max - 1)
    }
    fn move_blizzard(&self, new_field: &mut Field, position: &Point, direction: &Direction) {
        let movement = match direction {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        };
        let desired_position = (position.0 + movement.0, position.1 + movement.1);
        match self.0.get(&(desired_position)) {
            Some(Cell::Wall) => {
                let wall_position = self
                    .0
                    .iter()
                    .find(|(ivec, cell)| {
                        let is_wall = cell == &&Cell::Wall;
                        let is_in_row_or_column = match direction {
                            Direction::Left => ivec.0 > position.0 && ivec.1 == position.1,
                            Direction::Right => ivec.0 < position.0 && ivec.1 == position.1,
                            Direction::Up => ivec.1 > position.1 && ivec.0 == position.0,
                            Direction::Down => ivec.1 < position.1 && ivec.0 == position.0,
                        };
                        is_wall && is_in_row_or_column
                    })
                    .unwrap()
                    .0;
                let left_of_wall = (wall_position.0 + movement.0, wall_position.1 + movement.1);

                new_field
                    .0
                    .entry(left_of_wall)
                    .and_modify(|cell| {
                        // dbg!(&cell);
                        if let Cell::Blizzard(directions) = cell {
                            directions.push(*direction);
                        };
                    })
                    .or_insert(Cell::Blizzard(vec![*direction]));
            }
            Some(_) => {
                // dbg!("here");
                new_field
                    .0
                    .entry(desired_position)
                    .and_modify(|cell| {
                        if let Cell::Blizzard(directions) = cell {
                            directions.push(*direction);
                        };
                    })
                    .or_insert(Cell::Blizzard(vec![*direction]));
            }
            None => {
                panic!("shouldn't be none")
            }
        }
    }
    fn step(&self) -> Self {
        let mut new_field: Field = Field(HashMap::new());
        let blizzards = self.0.iter().filter_map(|(pos, cell)| match cell {
            Cell::Wall => None,
            Cell::Blizzard(direction) => Some((pos, direction)),
            Cell::Floor => None,
        });
        for (position, directions) in blizzards {
            for one in directions {
                self.move_blizzard(&mut new_field, position, one);
            }
        }
        // copy walls
        self.0.iter().for_each(|(pos, cell)| match cell {
            Cell::Wall => {
                new_field.0.insert(*pos, cell.clone());
            }
            _ => (),
        });

        let field_size = self.get_inner_dimensions();
        let total_size = (field_size.0 + 2, field_size.1 + 2);
        // dbg!(total_size);
        for (y, x) in (0..total_size.1).cartesian_product(0..total_size.0) {
            let pos = (x, y);
            if new_field.0.get(&pos).is_none() {
                new_field.0.insert(pos, Cell::Floor);
            }
        }
        new_field
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_max = *self.0.keys().into_iter().map(|(x, _)| x).max().unwrap() as usize;

        let value = self
            .0
            .iter()
            .sorted_by(|a, b| (a.0 .1, a.0 .0).cmp(&(b.0 .1, b.0 .0)))
            .map(|(_, cell)| match cell {
                Cell::Wall => "#".to_string(),
                Cell::Floor => ".".to_string(),
                Cell::Blizzard(directions) => match directions.len() {
                    1 => match directions[0] {
                        Direction::Up => "^".to_string(),
                        Direction::Right => ">".to_string(),
                        Direction::Down => "v".to_string(),
                        Direction::Left => "<".to_string(),
                    },
                    n => n.to_string(),
                },
            })
            .chunks(x_max + 1)
            .into_iter()
            .map(|chunk| chunk.collect::<String>())
            .join("\n");

        write!(f, "{}", value)
    }
}

fn field_parser(i: &str) -> IResult<&str, Field> {
    let mut it = iterator(
        i,
        terminated(
            many1(alt((
                complete::char('.').map(|_| Cell::Floor),
                complete::char('#').map(|_| Cell::Wall),
                complete::char('^').map(|_| Cell::Blizzard(vec![Direction::Up])),
                complete::char('>').map(|_| Cell::Blizzard(vec![Direction::Right])),
                complete::char('v').map(|_| Cell::Blizzard(vec![Direction::Down])),
                complete::char('<').map(|_| Cell::Blizzard(vec![Direction::Left])),
            ))),
            alt((line_ending, eof)),
        ),
    );

    let cells = it
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .map(move |(x, cell)| ((x as i32, y as i32), cell))
        })
        .collect::<HashMap<Point, Cell>>();

    let res: IResult<_, _> = it.finish();
    Ok((res.unwrap().0, Field(cells)))
}

fn part1() {
    let input = include_str!("../example.txt");
    // let input = include_str!("../input.txt");
    let mut field = field_parser(input).unwrap().1;

    let field_size = field.get_inner_dimensions();
    let step_cycle_number = [
        (field_size.0..).step_by(field_size.0 as usize),
        (field_size.1..).step_by(field_size.1 as usize),
    ]
    .into_iter()
    .kmerge()
    .tuple_windows()
    .find(|(a, b)| a == b)
    .unwrap()
    .0;

    let end_position = field
        .0
        .iter()
        .filter_map(|(pos, cell)| match cell {
            Cell::Wall => None,
            Cell::Blizzard(_) => None,
            Cell::Floor => Some(pos),
        })
        .max_by(|(x1, y1), (x2, y2)| (y1, x1).cmp(&(y2, x2)))
        .unwrap()
        .clone();

    let mut edges: Vec<((i32, i32, i32), (i32, i32, i32))> = vec![];
    for i in 0..(step_cycle_number) {
        let next_field = field.step();
        let origin_spaces = field.0.iter().filter_map(|(pos, cell)| match cell {
            Cell::Wall => None,
            Cell::Blizzard(_) => None,
            Cell::Floor => Some(pos),
        });

        for origin_position in origin_spaces {
            // println!("{:?}", origin_position);

            let possible_next_positions = vec![(-1, 0), (0, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|(y, x)| {
                    let next_position = (x + origin_position.0, y + origin_position.1);
                    if let Some(Cell::Floor) = next_field.0.get(&next_position) {
                        Some(next_position)
                    } else {
                        None
                    }
                });

            for pos in possible_next_positions {
                edges.push((
                    (origin_position.0, origin_position.1, i),
                    (
                        pos.0,
                        pos.1,
                        if step_cycle_number == i + 1 {
                            // dbg!("step_cycle_number");
                            0
                        } else {
                            i + 1
                        },
                    ),
                ))
            }
        }
        field = next_field;
    }
    // construct graph
    let graph = DiGraphMap::<(i32, i32, i32), ()>::from_edges(edges);

    // let dot =
    //     Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    // // println!(
    // //     "{:?}",
    // //     Dot::with_config(&graph, &[Config::EdgeNoLabel])
    // // );
    // let mut file = File::create("graph.dot").unwrap();
    // file.write_all(format!("{:?}", dot).as_bytes())
    //     .unwrap();
    // connect final cycle to first cycle
    let result = dijkstra(
        &graph,
        (1, 0, 0),
        None, // Some(end_edge_cell),
        |_| 1,
    );
    let a = result
        .iter()
        .filter_map(|(end, value)| {
            if end.0 == end_position.0 && end.1 == end_position.1 {
                Some(value)
            } else {
                None
            }
        })
        .min()
        .unwrap()
        .to_string();

    dbg!(a);
}

fn part2() {
    // let input = include_str!("../example.txt");
    let input = include_str!("../input.txt");
    let mut field = field_parser(input).unwrap().1;

    let field_size = field.get_inner_dimensions();
    let step_cycle_number = [
        (field_size.0..).step_by(field_size.0 as usize),
        (field_size.1..).step_by(field_size.1 as usize),
    ]
    .into_iter()
    .kmerge()
    .tuple_windows()
    .find(|(a, b)| a == b)
    .unwrap()
    .0;

    let end_position = field
        .0
        .iter()
        .filter_map(|(pos, cell)| match cell {
            Cell::Wall => None,
            Cell::Blizzard(_) => None,
            Cell::Floor => Some(pos),
        })
        .max_by(|(x1, y1), (x2, y2)| (y1, x1).cmp(&(y2, x2)))
        .unwrap()
        .clone();

    let mut edges: Vec<((i32, i32, i32), (i32, i32, i32))> = vec![];
    for i in 0..(step_cycle_number) {
        let next_field = field.step();
        let origin_spaces = field.0.iter().filter_map(|(pos, cell)| match cell {
            Cell::Wall => None,
            Cell::Blizzard(_) => None,
            Cell::Floor => Some(pos),
        });

        for origin_position in origin_spaces {
            // println!("{:?}", origin_position);

            let possible_next_positions = vec![(-1, 0), (0, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|(y, x)| {
                    let next_position = (x + origin_position.0, y + origin_position.1);
                    if let Some(Cell::Floor) = next_field.0.get(&next_position) {
                        Some(next_position)
                    } else {
                        None
                    }
                });

            for pos in possible_next_positions {
                edges.push((
                    (origin_position.0, origin_position.1, i),
                    (
                        pos.0,
                        pos.1,
                        if step_cycle_number == i + 1 {
                            // dbg!("step_cycle_number");
                            0
                        } else {
                            i + 1
                        },
                    ),
                ))
            }
        }
        field = next_field;
    }
    // construct graph
    let graph = DiGraphMap::<(i32, i32, i32), ()>::from_edges(edges);

    let result = dijkstra(
        &graph,
        (1, 0, 0),
        None, // Some(end_edge_cell),
        |_| 1,
    );
    let to_goal = result
        .iter()
        .filter(|(end, value)| {
            if end.0 == end_position.0 && end.1 == end_position.1 {
                true
            } else {
                false
            }
        })
        .min_by_key(|(_, value)| *value)
        .unwrap();
    // dbg!(to_goal);

    let result = dijkstra(
        &graph,
        *to_goal.0,
        None, // Some(end_edge_cell),
        |_| 1,
    );
    let back_to_camp = result
        .iter()
        .filter(|(end, value)| {
            if end.0 == 1 && end.1 == 0 {
                true
            } else {
                false
            }
        })
        .min_by_key(|(_, value)| *value)
        .unwrap();
    // dbg!(back_to_camp);
    let result = dijkstra(
        &graph,
        *back_to_camp.0,
        None, // Some(end_edge_cell),
        |_| 1,
    );
    let back_to_goal = result
        .iter()
        .filter(|(end, value)| {
            if end.0 == end_position.0 && end.1 == end_position.1 {
                true
            } else {
                false
            }
        })
        .min_by_key(|(_, value)| *value)
        .unwrap();
    // dbg!(back_to_goal);
    let a = (to_goal.1 + back_to_camp.1 + back_to_goal.1).to_string();

    dbg!(a);
}

fn main() {
    part2();
}
