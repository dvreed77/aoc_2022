use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    *,
};
use std::{cmp::Ordering, collections::HashSet, fmt};

#[derive(Clone, PartialEq, Eq)]
enum Item {
    Value(u32),
    List(Vec<Item>),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Value(s), Item::Value(o)) => s.cmp(o),
            (Item::Value(n), Item::List(_)) => Item::List(vec![Item::Value(*n)]).cmp(other),
            (Item::List(_), Item::Value(n)) => self.cmp(&Item::List(vec![Item::Value(*n)])),
            (Item::List(left), Item::List(right)) => {
                for i in 0..left.len().min(right.len()) {
                    match left[i].cmp(&right[i]) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        _ => {}
                    }
                }
                left.len().cmp(&right.len())
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

fn match_list(list_a: Vec<Item>, list_b: Vec<Item>) -> Ordering {
    // println!("Compare {:?} vs {:?}", list_a, list_b);
    for i in 0..list_a.len().min(list_b.len()) {
        let a = list_a[i].clone();

        // if i >= list_b.len() {
        //     // println!("Right side ran out of items, so inputs are not in the right order");
        //     return Ordering::Greater;
        // }
        let b = list_b[i].clone();

        match (&a, &b) {
            (Item::Value(x), Item::Value(y)) => match x.cmp(&y) {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                _ => {}
            },
            (Item::List(x), Item::Value(y)) => {
                return is_in_order((Item::List(x.clone()), Item::List(vec![Item::Value(*y)])));
            }
            (Item::Value(x), Item::List(y)) => {
                return is_in_order((Item::List(vec![Item::Value(*x)]), Item::List(y.clone())));
            }
            (Item::List(x), Item::List(y)) => {
                return is_in_order((Item::List(x.clone()), Item::List(y.clone())));
            }
        }
    }
    // println!("C");
    return list_a.len().cmp(&list_b.len());
    // return true;
}
fn is_in_order((a, b): (Item, Item)) -> Ordering {
    match (a, b) {
        (Item::Value(x), Item::Value(y)) => x.cmp(&y),
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

    let item_pairs = parser(input).unwrap().1;
    let pair = &item_pairs[140];
    is_in_order((pair.0.clone(), pair.1.clone()));

    let mut out = vec![];
    let mut out2 = vec![];

    for (idx, pair) in item_pairs.iter().enumerate() {
        if is_in_order((pair.0.clone(), pair.1.clone())) != Ordering::Greater {
            out2.push(idx + 1);
        }

        if pair.0 < pair.1 {
            out.push(idx + 1)
        }
    }

    let a: HashSet<usize> = HashSet::from_iter(out2.clone());
    let b: HashSet<usize> = HashSet::from_iter(out.clone());

    dbg!(item_pairs[140].clone());

    // dbg!(b.difference(&a));

    dbg!(out.iter().sum::<usize>());
    dbg!(out2.iter().sum::<usize>());
}
