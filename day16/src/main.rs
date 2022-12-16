use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::{map, opt},
    multi::{separated_list0, separated_list1},
    sequence::tuple,
    *,
};

use petgraph::Graph;
use petgraph::{algo::bellman_ford, dot::Config};
use petgraph::{dot::Dot, prelude::*};

#[derive(Debug)]
struct Valve {
    name: String,
    rate: f32,
    children: Vec<String>,
}

fn line_parser(i: &str) -> IResult<&str, Valve> {
    let (input, (_, name, _, rate, _, _, _, _, _, _, _, children)) = tuple((
        tag("Valve "),
        complete::alpha1,
        tag(" has flow rate="),
        complete::i32,
        tag("; tunnel"),
        opt(tag("s")),
        tag(" lead"),
        opt(tag("s")),
        tag(" to valve"),
        opt(tag("s")),
        opt(tag(" ")),
        separated_list0(tag(", "), map(complete::alpha1, |x: &str| x.to_string())),
    ))(i)?;

    return Ok((
        input,
        Valve {
            name: name.to_string(),
            rate: rate as f32,
            children,
        },
    ));
}

fn parser(i: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(newline, line_parser)(i)
}

fn part1(valves: Vec<Valve>) {
    let mut g = Graph::new();

    let mut map = HashMap::new();

    for v in &valves {
        let node = g.add_node(v.rate);
        map.insert(v.name.clone(), node);
    }

    for v in &valves {
        let this_node = map.get(&v.name).unwrap();
        for c in &v.children {
            let child_node = map.get(c).unwrap();
            g.add_edge(this_node.clone(), child_node.clone(), 1 as f32);
        }
    }

    // println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

    let a = bellman_ford(&g, *map.get("AA").unwrap()).unwrap();

    println!("{:?}", a.predecessors);
}

fn main() {
    // let input = include_str!("../input.txt");
    let input = include_str!("../example.txt");

    let valves = parser(input).unwrap().1;

    part1(valves);
}
