use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    multi::separated_list1,
    sequence::tuple,
    *,
};
#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
    Divide,
    Subtract,
}

#[derive(Debug)]
enum Monkey<'a> {
    Math(&'a str, &'a str, &'a str, Operation),
    Value(&'a str, f64),
}

fn operation_parser(i: &str) -> IResult<&str, Operation> {
    alt((
        tag(" + ").map(|_| Operation::Add),
        tag(" - ").map(|_| Operation::Subtract),
        tag(" * ").map(|_| Operation::Multiply),
        tag(" / ").map(|_| Operation::Divide),
    ))(i)
}

fn math_monkey(i: &str) -> IResult<&str, Monkey> {
    let (input, (id, _, m1, op, m2)) =
        tuple((alpha1, tag(": "), alpha1, operation_parser, alpha1))(i)?;

    Ok((input, Monkey::Math(id, m1, m2, op)))
}

fn value_monkey(i: &str) -> IResult<&str, Monkey> {
    let (input, (id, _, val)) = tuple((alpha1, tag(": "), complete::u32))(i)?;

    Ok((input, Monkey::Value(id, val as f64)))
}

fn parser(i: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(newline, alt((math_monkey, value_monkey)))(i)
}

fn get_value(node_str: &str, monkey_map: &HashMap<&str, &Monkey>) -> f64 {
    let node = monkey_map.get(node_str).unwrap();
    match node {
        Monkey::Value(_, v) => v.clone(),
        Monkey::Math(_, c1, c2, op) => {
            let v1 = get_value(c1, monkey_map);
            let v2 = get_value(c2, monkey_map);

            match op {
                Operation::Add => v1 + v2,
                Operation::Subtract => v1 - v2,
                Operation::Multiply => v1 * v2,
                Operation::Divide => v1 / v2,
            }
        }
    }
}

fn main() {
    // let input = include_str!("../example.txt");
    let input = include_str!("../input.txt");

    let monkeys = parser(input).unwrap().1;

    let monkey_map: HashMap<&str, &Monkey> = monkeys
        .iter()
        .map(|m| {
            let id = match m {
                Monkey::Math(id, _, _, _) => *id,
                Monkey::Value(id, _) => *id,
            };
            let p = m.clone();
            (id, p)
        })
        .collect();

    let v = get_value("root", &monkey_map);

    dbg!(v);
}
