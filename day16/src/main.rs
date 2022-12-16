use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::{map, opt},
    multi::{separated_list0, separated_list1},
    sequence::tuple,
    *,
};

use itertools::Itertools;
use petgraph::{algo::astar, stable_graph::NodeIndex, Graph};
// use petgraph::{algo::bellman_ford, dot::Config};
// use petgraph::{dot::Dot, prelude::*};

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

fn path_cost(
    path: Vec<String>,
    dist_map: HashMap<(String, String), f32>,
    rate_map: HashMap<String, f32>,
) -> Option<f32> {
    let mut total_pressure = 0.0;
    let mut last_pressure = 0.0;
    let mut time = 0.0;
    for (i, node) in path.iter().enumerate() {
        if i == 0 {
            time += dist_map.get(&("AA".to_owned(), node.clone())).unwrap() + 1.0;
            last_pressure = rate_map.get(node).unwrap().clone();

            continue;
        }
        let d = dist_map.get(&(path[i - 1].clone(), node.clone())).unwrap();
        let r = rate_map.get(node).unwrap();
        total_pressure = total_pressure + last_pressure * (d + 1.0);
        last_pressure += r;
        time += d + 1.0;
        if time > 30.0 {
            return None;
        }
        // dbg!(&path[i - 1], &path[i], total_pressure, last_pressure, time);
    }

    if (time <= 30.0) {
        let d = 30.0 - time;
        total_pressure = total_pressure + last_pressure * d;
        Some(total_pressure)
    } else {
        None
    }
}

fn distances(
    nodes: Vec<String>,
    g: Graph<String, f32>,
    map: HashMap<String, NodeIndex>,
) -> HashMap<(String, String), f32> {
    let mut out = HashMap::new();

    for a in nodes.iter().combinations(2) {
        let n1 = map.get(a[0]).unwrap();
        let n2 = map.get(a[1]).unwrap();

        let path = astar(
            &g,
            n1.clone(),
            |finish| finish == n2.clone(),
            |e| *e.weight(),
            |_| 0.0,
        );
        let d = path.unwrap().0;
        out.insert((a[0].clone(), a[1].clone()), d);
        out.insert((a[1].clone(), a[0].clone()), d);
    }

    for n in nodes.iter() {
        let n1 = map.get("AA").unwrap();
        let n2 = map.get(n).unwrap();

        let path = astar(
            &g,
            n1.clone(),
            |finish| finish == n2.clone(),
            |e| *e.weight(),
            |_| 0.0,
        );
        let d = path.unwrap().0;
        out.insert(("AA".to_owned(), n.clone()), d);
        out.insert((n.clone(), "AA".to_owned()), d);
    }

    out
}

fn part1(valves: Vec<Valve>) {
    // dbg!(valves);
    let mut g = Graph::new();

    let rate_map = valves
        .iter()
        .map(|x| (x.name.clone(), x.rate))
        .collect::<HashMap<_, _>>();

    let mut map = HashMap::new();

    for v in &valves {
        let node = g.add_node(v.name.clone());
        map.insert(v.name.clone(), node);
    }

    for v in &valves {
        let this_node = map.get(&v.name).unwrap();
        for c in &v.children {
            let child_node = map.get(c).unwrap();
            g.add_edge(this_node.clone(), child_node.clone(), 1 as f32);
        }
    }

    let non_zero_valves = valves
        .iter()
        .filter(|x| x.rate > 0.0)
        .map(|x| x.name.clone())
        .collect::<Vec<_>>();

    // dbg!(non_zero_valves);

    let dist_map = distances(non_zero_valves.clone(), g, map);
    // let path = ["DD", "BB", "JJ", "HH", "EE", "CC"]
    //     .iter()
    //     .map(|x| x.to_string())
    //     .collect_vec();

    // let d = path_cost(path, dist_map, rate_map);

    // dbg!(d);
    let perms = non_zero_valves
        .iter()
        .cloned()
        .permutations(non_zero_valves.len());

    let mut costs = vec![];
    for p in perms {
        costs.push(path_cost(p, dist_map.clone(), rate_map.clone()));
    }

    dbg!(costs
        .iter()
        .filter_map(|&x| x)
        .map(|x| x as i32)
        .collect_vec()
        .iter()
        .max());
}

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../example.txt");

    let valves = parser(input).unwrap().1;

    part1(valves);
}
