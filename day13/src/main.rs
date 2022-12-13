use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    *,
};
use std::fmt;

#[derive(Clone)]
enum Item {
    Value(u32),
    List(Vec<Item>),
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Item::Value(x) => write!(f, "{}", x),
            Item::List(x) => write!(f, "{:?}", x.clone()),
        }
    }
}

fn list_parser(i: &str) -> IResult<&str, Vec<Item>> {
    delimited(tag("["), separated_list1(tag(","), item_value), tag("]"))(i)
}

fn item_value(i: &str) -> IResult<&str, Item> {
    alt((
        value(Item::List(vec![]), tag("[]")),
        map(complete::u32, Item::Value),
        map(list_parser, Item::List),
    ))(i)
}

fn parser(i: &str) -> IResult<&str, Vec<(Item, Item)>> {
    separated_list1(tag("\n\n"), separated_pair(item_value, newline, item_value))(i)
}

fn match_list(list_a: Vec<Item>, list_b: Vec<Item>) -> bool {
    // println!("Compare {:?} vs {:?}", list_a, list_b);
    for i in 0..list_a.len() {
        let a = list_a[i].clone();

        if i >= list_b.len() {
            // println!("Right side ran out of items, so inputs are not in the right order");
            return false;
        }
        let b = list_b[i].clone();

        match (&a, &b) {
            (Item::Value(x), Item::Value(y)) => {
                // println!("Val/Val");
                if x < y {
                    // println!("A");
                    return true;
                } else if y < x {
                    // println!("Right side is smaller, so inputs are not in the right order");
                    return false;
                }
            }
            (Item::List(x), Item::Value(y)) => {
                // println!("List/Value");
                return is_in_order((Item::List(x.clone()), Item::List(vec![Item::Value(*y)])));
            }
            (Item::Value(x), Item::List(y)) => {
                // println!("List/Value");
                return is_in_order((Item::List(vec![Item::Value(*x)]), Item::List(y.clone())));
            }
            (Item::List(x), Item::List(y)) => {
                // println!("List/Value");
                return is_in_order((Item::List(x.clone()), Item::List(y.clone())));
            }
        }
    }
    // println!("C");
    return true;
}
fn is_in_order((a, b): (Item, Item)) -> bool {
    match (a, b) {
        (Item::Value(x), Item::Value(y)) => x < y,
        (Item::List(x), Item::List(y)) => match_list(x, y),
        _ => {
            println!("HERE");
            // dbg!(a.clone(), b.clone());
            panic!();
        }
    }
}

fn main() {
    // let input = include_str!("../example.txt");
    let input = include_str!("../input.txt");

    // dbg!(list_parser("[1,[2,[3,[4,[5,6,7]]]],8,9]"));
    // dbg!(list_parser("[[1],[2,3,4]]"));

    let item_pairs = parser(input).unwrap().1;

    let pair = &item_pairs[127];
    let mut out = vec![];
    is_in_order((pair.0.clone(), pair.1.clone()));
    // println!("{:?}", item_pairs[0].1);

    for (idx, pair) in item_pairs.iter().enumerate() {
        if is_in_order((pair.0.clone(), pair.1.clone())) {
            out.push(idx + 1);
        }
    }

    // dbg!(&out);

    dbg!(out.iter().sum::<usize>());
}
