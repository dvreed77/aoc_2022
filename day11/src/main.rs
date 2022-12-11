use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{self, alphanumeric1, newline, none_of, not_line_ending},
    combinator::{complete, eof, map, opt},
    multi::{fold_many0, fold_many1, separated_list1},
    sequence::{pair, preceded, separated_pair, tuple},
    *,
};
use num::BigInt;
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug)]
enum Operation {
    Add(u32),
    Multiply(u32),
    Square,
}

#[derive(Debug)]
struct Monkey {
    num: u32,
    items: VecDeque<BigInt>,
    op: Operation,
    t: u32,
    tf: (u32, u32),
}

fn item_parser(i: &str) -> IResult<&str, VecDeque<BigInt>> {
    // Starting items: 79, 98
    let (input, v) = preceded(
        tag("  Starting items: "),
        separated_list1(tag(", "), complete::u32),
    )(i)?;

    let v2: VecDeque<u32> = v.into();
    let v3 = v2
        .iter()
        .map(|&x| BigInt::from(x))
        .collect::<VecDeque<BigInt>>();

    return Ok((input, v3));
}

fn operation2_parser(i: &str) -> nom::IResult<&str, Operation> {
    alt((
        map(preceded(tag("old * "), complete::u32), |x| {
            Operation::Multiply(x)
        }),
        map(preceded(tag("old + "), complete::u32), |x| {
            Operation::Add(x)
        }),
        map(tag("old * old"), |_| Operation::Square),
    ))(i)
}

fn operation_parser(i: &str) -> IResult<&str, Operation> {
    // Operation: new = old * 19
    preceded(tag("  Operation: new = "), operation2_parser)(i)
}

fn test_parser(i: &str) -> IResult<&str, u32> {
    // Test: divisible by 23
    preceded(tag("  Test: divisible by "), complete::u32)(i)
}

fn tf_parser(i: &str) -> IResult<&str, (u32, u32)> {
    // If true: throw to monkey 2
    // If false: throw to monkey 3
    separated_pair(
        preceded(tag("    If true: throw to monkey "), complete::u32),
        newline,
        preceded(tag("    If false: throw to monkey "), complete::u32),
    )(i)
}
fn part1(input: &str) {
    println!("Part 1: {}", input);
}

fn monkey_parser(i: &str) -> IResult<&str, Monkey> {
    let (input, (_, num, _)) = tuple((tag("Monkey "), complete::u32, tag(":\n")))(i)?;

    let (input, items) = item_parser(input)?;

    let (input, _) = newline(input)?;
    let (input, op) = operation_parser(input)?;
    let (input, _) = newline(input)?;
    let (input, t) = test_parser(input)?;
    let (input, _) = newline(input)?;
    let (input, tf) = tf_parser(input)?;

    Ok((
        input,
        Monkey {
            num,
            items,
            op,
            t,
            tf,
        },
    ))
}

fn parser(i: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n\n"), monkey_parser)(i)
}

fn main() {
    // let input = include_str!("../example.txt");
    let input = include_str!("../input.txt");

    let mut monkeys = parser(input).unwrap().1;

    let mut my_map: BTreeMap<u32, BigInt> = BTreeMap::new();

    for round in 0..20 {
        for idx in 0..monkeys.len() {
            let il = monkeys[idx].items.len() as u32;
            my_map
                .entry(idx as u32)
                .and_modify(|x| *x += il)
                .or_insert(BigInt::from(il));
            for _ in 0..monkeys[idx].items.len() {
                let i = monkeys[idx].items.pop_front().unwrap().clone();
                let j = i.clone();
                // dbg!(i);
                let w = match monkeys[idx].op {
                    Operation::Add(x) => i + BigInt::from(x),
                    Operation::Multiply(x) => i * BigInt::from(x),
                    Operation::Square => i * j,
                }
                .clone();
                // dbg!(w);
                let w1: BigInt = w.clone() / 3;
                // dbg!(w);
                let tf = monkeys[idx].tf;
                if &w1 % monkeys[idx].t == BigInt::from(0) {
                    monkeys[tf.0 as usize].items.push_back(w1.clone());
                } else {
                    monkeys[tf.1 as usize].items.push_back(w1);
                }
            }
        }
    }

    let mut v = my_map
        .iter()
        .map(|(&k, v)| v.clone())
        .collect::<Vec<BigInt>>();

    v.sort_by(|a, b| b.cmp(a));

    dbg!(v[0].clone() * v[1].clone());

    let mut monkeys = parser(input).unwrap().1;

    let d = monkeys.iter().map(|m| m.t).product::<u32>();

    dbg!(d);

    let mut my_map: BTreeMap<u32, BigInt> = BTreeMap::new();

    for _ in 0..10_000 {
        for idx in 0..monkeys.len() {
            let il = monkeys[idx].items.len() as u32;
            my_map
                .entry(idx as u32)
                .and_modify(|x| *x += il)
                .or_insert(BigInt::from(il));
            for _ in 0..monkeys[idx].items.len() {
                let i = monkeys[idx].items.pop_front().unwrap().clone();
                let j = i.clone();
                // dbg!(i);
                let w = match monkeys[idx].op {
                    Operation::Add(x) => i + BigInt::from(x),
                    Operation::Multiply(x) => i * BigInt::from(x),
                    Operation::Square => (i * j),
                }
                .clone();
                // dbg!(w);
                let w1: BigInt = w.clone() % d;
                // dbg!(w);
                let tf = monkeys[idx].tf;
                if &w1 % monkeys[idx].t == BigInt::from(0) {
                    monkeys[tf.0 as usize].items.push_back(w1.clone());
                } else {
                    monkeys[tf.1 as usize].items.push_back(w1);
                }
            }
        }
    }

    let mut v = my_map
        .iter()
        .map(|(&k, v)| v.clone())
        .collect::<Vec<BigInt>>();

    v.sort_by(|a, b| b.cmp(a));

    dbg!(my_map);
    dbg!(v[0].clone() * v[1].clone());
}
