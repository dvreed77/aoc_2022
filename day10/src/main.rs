use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{self, alphanumeric1, newline, none_of, not_line_ending},
    combinator::{eof, map, opt},
    multi::{fold_many0, fold_many1, separated_list1},
    sequence::{pair, preceded, separated_pair},
    *,
};

fn add_parser(i: &str) -> IResult<&str, i32> {
    preceded(tag("addx "), complete::i32)(i)
}

fn instruction_parser(i: &str) -> nom::IResult<&str, Instruction> {
    alt((
        map(add_parser, |x| Instruction::Add(x)),
        map(tag("noop"), |_| Instruction::Noop),
    ))(i)
}
#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Add(i32),
}

fn main() {
    // let text = include_str!("../example.txt");
    let text = include_str!("../input.txt");

    let instructions = separated_list1(newline, instruction_parser)(text)
        .unwrap()
        .1;

    let mut out = vec![];
    let mut cycle = 1;
    let mut acc = 1;

    let cycles = [20, 60, 100, 140, 180, 220];
    for instruction in &instructions {
        match instruction {
            Instruction::Add(x) => {
                cycle += 1;
                if cycles.contains(&cycle) {
                    out.push((cycle, acc));
                }
                cycle += 1;
                acc += x;
                if cycles.contains(&cycle) {
                    out.push((cycle, acc));
                }
            }
            Instruction::Noop => {
                cycle += 1;
                if cycles.contains(&cycle) {
                    out.push((cycle, acc));
                }
            }
        }
    }

    let a = out.into_iter().map(|(x, y)| x * y).sum::<i32>();

    println!("Part1: {}", a);

    let mut sprite_pos = 1;
    let mut cycle = 1;
    let mut out = vec![];
    for instruction in &instructions {
        match instruction {
            Instruction::Add(x) => {
                let pixel: i32 = (cycle - 1) % 40;
                let draw = if (pixel - sprite_pos).abs() <= 1 {
                    true
                } else {
                    false
                };
                out.push(draw);

                cycle += 1;

                let pixel: i32 = (cycle - 1) % 40;
                let draw = if (pixel - sprite_pos).abs() <= 1 {
                    true
                } else {
                    false
                };
                out.push(draw);

                cycle += 1;
                sprite_pos += x;
            }
            Instruction::Noop => {
                let pixel: i32 = (cycle - 1) % 40;
                let draw = if (pixel - sprite_pos).abs() <= 1 {
                    true
                } else {
                    false
                };
                out.push(draw);

                cycle += 1;
            }
        }
    }

    for (i, c) in out.iter().enumerate() {
        if i % 40 == 0 {
            println!();
        }
        if *c {
            print!("#");
        } else {
            print!(".");
        }
    }
}
