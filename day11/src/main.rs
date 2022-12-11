use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    *,
};
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: Operation,
    t: u64,
    tf: (u64, u64),
}

fn item_parser(i: &str) -> IResult<&str, VecDeque<u64>> {
    // Starting items: 79, 98
    let (input, v) = preceded(
        tag("  Starting items: "),
        separated_list1(tag(", "), complete::u64),
    )(i)?;

    let v2: VecDeque<u64> = v.into();

    return Ok((input, v2));
}

fn operation2_parser(i: &str) -> nom::IResult<&str, Operation> {
    alt((
        map(preceded(tag("old * "), complete::u64), |x| {
            Operation::Multiply(x)
        }),
        map(preceded(tag("old + "), complete::u64), |x| {
            Operation::Add(x)
        }),
        map(tag("old * old"), |_| Operation::Square),
    ))(i)
}

fn operation_parser(i: &str) -> IResult<&str, Operation> {
    // Operation: new = old * 19
    preceded(tag("  Operation: new = "), operation2_parser)(i)
}

fn test_parser(i: &str) -> IResult<&str, u64> {
    // Test: divisible by 23
    preceded(tag("  Test: divisible by "), complete::u64)(i)
}

fn tf_parser(i: &str) -> IResult<&str, (u64, u64)> {
    // If true: throw to monkey 2
    // If false: throw to monkey 3
    separated_pair(
        preceded(tag("    If true: throw to monkey "), complete::u64),
        newline,
        preceded(tag("    If false: throw to monkey "), complete::u64),
    )(i)
}

fn monkey_parser(i: &str) -> IResult<&str, Monkey> {
    let (input, (_, _, _)) = tuple((tag("Monkey "), complete::u64, tag(":\n")))(i)?;

    let (input, items) = item_parser(input)?;

    let (input, _) = newline(input)?;
    let (input, op) = operation_parser(input)?;
    let (input, _) = newline(input)?;
    let (input, t) = test_parser(input)?;
    let (input, _) = newline(input)?;
    let (input, tf) = tf_parser(input)?;

    Ok((input, Monkey { items, op, t, tf }))
}

fn parser(i: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n\n"), monkey_parser)(i)
}

fn main() {
    // let input = include_str!("../example.txt");
    let input = include_str!("../input.txt");

    let mut monkeys = parser(input).unwrap().1;

    let mut my_map: BTreeMap<u64, u64> = BTreeMap::new();

    for round in 0..20 {
        for idx in 0..monkeys.len() {
            let il = monkeys[idx].items.len() as u64;
            my_map
                .entry(idx as u64)
                .and_modify(|x| *x += il)
                .or_insert(il);
            for _ in 0..monkeys[idx].items.len() {
                let i = monkeys[idx].items.pop_front().unwrap().clone();
                let j = i.clone();
                // dbg!(i);
                let mut w = match monkeys[idx].op {
                    Operation::Add(x) => i + x,
                    Operation::Multiply(x) => i * x,
                    Operation::Square => i * i,
                }
                .clone();
                // dbg!(w);
                w = w / 3;
                // dbg!(w);
                let tf = monkeys[idx].tf;
                if &w % monkeys[idx].t == 0 {
                    monkeys[tf.0 as usize].items.push_back(w);
                } else {
                    monkeys[tf.1 as usize].items.push_back(w);
                }
            }
        }
    }

    let mut v = my_map.iter().map(|(&k, v)| v.clone()).collect::<Vec<u64>>();

    v.sort_by(|a, b| b.cmp(a));

    dbg!(v[0] * v[1]);

    let mut monkeys = parser(input).unwrap().1;

    let d = monkeys.iter().map(|m| m.t).product::<u64>();

    dbg!(d);

    let mut my_map: BTreeMap<u64, u64> = BTreeMap::new();

    for _ in 0..10_000 {
        for idx in 0..monkeys.len() {
            let il = monkeys[idx].items.len() as u64;
            my_map
                .entry(idx as u64)
                .and_modify(|x| *x += il)
                .or_insert(il);
            for _ in 0..monkeys[idx].items.len() {
                let i = monkeys[idx].items.pop_front().unwrap().clone();
                let j = i.clone();
                // dbg!(i);
                let mut w = match monkeys[idx].op {
                    Operation::Add(x) => i + x,
                    Operation::Multiply(x) => i * x,
                    Operation::Square => (i * j),
                }
                .clone();

                w = w % d;

                let tf = monkeys[idx].tf;
                if &w % monkeys[idx].t == 0 {
                    monkeys[tf.0 as usize].items.push_back(w);
                } else {
                    monkeys[tf.1 as usize].items.push_back(w);
                }
            }
        }
    }

    let mut v = my_map.iter().map(|(&k, v)| v.clone()).collect::<Vec<u64>>();

    v.sort_by(|a, b| b.cmp(a));

    dbg!(my_map);
    dbg!(v[0] * v[1]);
}
