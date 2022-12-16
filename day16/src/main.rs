use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::opt,
    multi::{separated_list0, separated_list1},
    sequence::tuple,
    *,
};

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    rate: u32,
    children: Vec<&'a str>,
}

fn line_parser(i: &str) -> IResult<&str, Valve> {
    let (input, (_, name, _, rate, _, _, _, _, _, _, _, children)) = tuple((
        tag("Valve "),
        complete::alpha1,
        tag(" has flow rate="),
        complete::u32,
        tag("; tunnel"),
        opt(tag("s")),
        tag(" lead"),
        opt(tag("s")),
        tag(" to valve"),
        opt(tag("s")),
        opt(tag(" ")),
        separated_list0(tag(", "), complete::alpha1),
    ))(i)?;

    return Ok((
        input,
        Valve {
            name,
            rate,
            children,
        },
    ));
}

fn parser(i: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(newline, line_parser)(i)
}

fn rec<'a>(
    node: &'a str,
    path: &mut Vec<&'a str>,
    flow_map: &HashMap<&str, u32>,
    graph: &'a HashMap<&str, Vec<&str>>,
    remaining: i32,
    cache: &mut HashMap<(&'a str, Vec<&'a str>, i32), u32>,
) -> u32 {
    if remaining <= 0 {
        return 0;
    }

    if let Some(&ans) = cache.get(&(node, path.clone(), remaining)) {
        return ans;
    }

    let mut best = 0;

    let f = *flow_map.get(node).unwrap();
    let visited = path.contains(&node);

    if f > 0 && !visited {
        path.push(node);
        for &child in graph.get(node).unwrap() {
            let a = rec(child, path, &flow_map, &graph, remaining - 2, cache);
            best = best.max(a + flow_map.get(node).unwrap() * (remaining as u32 - 1));
        }
        path.pop();
    }

    for &child in graph.get(node).unwrap() {
        let a = rec(child, path, &flow_map, &graph, remaining - 1, cache);
        best = best.max(a);
    }

    cache.insert((node, path.clone(), remaining), best);

    best
}

fn rec2<'a>(
    node: &'a str,
    path: &mut Vec<&'a str>,
    flow_map: &HashMap<&str, u32>,
    graph: &'a HashMap<&str, Vec<&str>>,
    remaining: i32,
    cache: &mut HashMap<(&'a str, Vec<&'a str>, i32), u32>,
) -> u32 {
    if remaining <= 0 {
        return 0;
    }

    if let Some(&ans) = cache.get(&(node, path.clone(), remaining)) {
        return ans;
    }

    let mut best = 0;

    let f = *flow_map.get(node).unwrap();
    let visited = path.contains(&node);

    if f > 0 && !visited {
        path.push(node);
        for &child in graph.get(node).unwrap() {
            let a = rec2(child, path, &flow_map, &graph, remaining - 2, cache);
            best = best.max(a + flow_map.get(node).unwrap() * (remaining as u32 - 1));
        }
        path.pop();
    }

    for &child in graph.get(node).unwrap() {
        let a = rec2(child, path, &flow_map, &graph, remaining - 1, cache);
        best = best.max(a);
    }

    cache.insert((node, path.clone(), remaining), best);

    best
}

fn part1(valves: Vec<Valve>) {
    let flow_map = valves
        .iter()
        .map(|x| (x.name.clone(), x.rate))
        .collect::<HashMap<_, _>>();

    let graph = valves
        .iter()
        .map(|x| (x.name.clone(), x.children.clone()))
        .collect::<HashMap<_, _>>();
    let mut path: Vec<&str> = Vec::new();
    let mut cache = HashMap::new();
    let ans: u32 = rec("AA", &mut path, &flow_map, &graph, 30, &mut cache);

    println!("Part 1: {}", ans);
}

fn part2(valves: Vec<Valve>) {
    let flow_map = valves
        .iter()
        .map(|x| (x.name.clone(), x.rate))
        .collect::<HashMap<_, _>>();

    let graph = valves
        .iter()
        .map(|x| (x.name.clone(), x.children.clone()))
        .collect::<HashMap<_, _>>();
    let mut path: Vec<&str> = Vec::new();
    let mut cache = HashMap::new();
    let ans: u32 = rec2("AA", &mut path, &flow_map, &graph, 26, &mut cache);

    println!("Part 2: {}", ans);
}

fn main() {
    // let input = include_str!("../input.txt");
    let input = include_str!("../example.txt");

    let valves = parser(input).unwrap().1;

    // part1(valves);
    part2(valves);
}
