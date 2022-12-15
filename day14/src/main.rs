use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    *,
};

type Point = (i32, i32);
type Path = Vec<Point>;

fn parser(i: &str) -> IResult<&str, Vec<Path>> {
    separated_list1(
        newline,
        separated_list1(
            tag(" -> "),
            separated_pair(complete::i32, tag(","), complete::i32),
        ),
    )(i)
}
#[derive(Hash, PartialEq, Eq, Clone)]
enum Block {
    Sand(Point),
    Rock(Point),
}

fn expand_paths(paths: Vec<Path>) -> HashMap<Point, Block> {
    let mut paths2 = vec![];
    for path in paths {
        let mut new_paths = vec![];
        for i in 0..path.len() - 1 {
            let (x1, y1) = path[i];
            let (x2, y2) = path[i + 1];
            new_paths.push((x1, y1));
            match (x2 - x1, y2 - y1) {
                (dx, 0) => {
                    let rng;
                    if dx < 0 {
                        rng = x2..x1;
                    } else {
                        rng = x1..x2;
                    }
                    for x in rng {
                        new_paths.push((x, y1));
                    }
                }
                (0, dy) => {
                    let rng;
                    if dy < 0 {
                        rng = y2..y1;
                    } else {
                        rng = y1..y2;
                    }
                    for y in rng {
                        new_paths.push((x1, y));
                    }
                }
                _ => panic!("bad path"),
            }
        }
        new_paths.push(path[path.len() - 1]);
        paths2.push(new_paths.clone());
    }

    let p = paths2
        .into_iter()
        .flatten()
        .map(|x| (x, Block::Rock(x)))
        .collect::<HashMap<Point, Block>>();

    return p;
}

fn bounds(paths: HashMap<Point, Block>) -> (i32, i32, i32, i32) {
    let all_pts = paths.iter().map(|(x, _)| x).collect::<Vec<_>>();

    let min_x = *all_pts.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *all_pts.iter().map(|(x, _)| x).max().unwrap();
    let min_y = 0;
    let max_y = *all_pts.iter().map(|(_, y)| y).max().unwrap();

    (min_x, max_x, min_y, max_y)
}

fn bounds2(paths: Vec<Path>, all_sand: Vec<Point>) -> (i32, i32, i32, i32) {
    let mut all_pts = paths.iter().flatten().collect::<Vec<_>>();
    let mut all_pts2 = all_sand.iter().collect::<Vec<_>>();
    all_pts.append(&mut all_pts2);

    let min_x = *all_pts.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *all_pts.iter().map(|(x, _)| x).max().unwrap();
    let min_y = 0;
    let max_y = *all_pts.iter().map(|(_, y)| y).max().unwrap();

    (min_x, max_x, min_y, max_y)
}

fn draw(paths: HashMap<Point, Block>, sand: Point) {
    let (min_x, max_x, min_y, max_y) = bounds(paths.clone());
    let start = (500, 0);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if (x, y) == start {
                print!("+");
            } else if (x, y) == sand {
                print!("o");
            } else if paths.contains_key(&(x, y)) {
                match paths.get(&(x, y)).unwrap() {
                    Block::Sand(_) => print!("o"),
                    Block::Rock(_) => print!("#"),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn draw2(paths: Vec<Path>, all_sand: Vec<Point>, sand: Point) {
    let (min_x, max_x, min_y, max_y) = bounds2(paths.clone(), all_sand.clone());
    let start = (500, 0);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if (x, y) == start {
                print!("+");
            } else if (x, y) == sand {
                print!("o");
            } else if all_sand.contains(&(x, y)) {
                print!("o");
            } else if paths.iter().any(|path| path.contains(&(x, y))) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn simulate(rocks: HashMap<Point, Block>) {
    let mut rocks = rocks.clone();

    let (_, _, min_y, max_y) = bounds(rocks.clone());
    'outer: loop {
        let mut sand = (500, 0);
        loop {
            let (_, y) = sand;

            if y > max_y || y < min_y {
                break 'outer;
            }

            let down = (sand.0, sand.1 + 1);
            let left = (sand.0 - 1, sand.1 + 1);
            let right = (sand.0 + 1, sand.1 + 1);

            if !rocks.contains_key(&down) {
                sand = down;
                continue;
            }

            if !rocks.contains_key(&left) {
                sand = left;
                continue;
            }

            if !rocks.contains_key(&right) {
                sand = right;
                continue;
            }

            rocks.insert(sand, Block::Sand(sand));
            break;
        }
    }

    let sand = rocks
        .iter()
        .filter_map(|(_, b)| match b {
            Block::Sand(_) => Some(b),
            _ => None,
        })
        .collect::<Vec<_>>();

    draw(rocks.clone(), (0, 0));
    dbg!(sand.len());
}

fn simulate2(rocks: HashMap<Point, Block>) {
    let mut rocks = rocks.clone();

    let (_, _, min_y, max_y) = bounds(rocks.clone());

    let floor = max_y + 1;
    'outer: loop {
        let mut sand = (500, 0);
        if rocks.contains_key(&sand) {
            break 'outer;
        }
        loop {
            let (_, y) = sand;

            let down = (sand.0, sand.1 + 1);
            let left = (sand.0 - 1, sand.1 + 1);
            let right = (sand.0 + 1, sand.1 + 1);

            if !rocks.contains_key(&down) && sand.1 < floor {
                sand = down;
                continue;
            }

            if !rocks.contains_key(&left) && sand.1 < floor {
                sand = left;
                continue;
            }

            if !rocks.contains_key(&right) && sand.1 < floor {
                sand = right;
                continue;
            }

            rocks.insert(sand, Block::Sand(sand));
            break;
        }
    }

    let sand = rocks
        .iter()
        .filter_map(|(_, b)| match b {
            Block::Sand(_) => Some(b),
            _ => None,
        })
        .collect::<Vec<_>>();

    // draw(rocks.clone(), (0, 0));
    dbg!(sand.len());
}

// fn simulate2(paths: Vec<Path>) {
//     let mut all_sand: Vec<Point> = vec![];
//     let (_, _, _, max_y) = bounds(paths.clone());

//     let floor = max_y + 1;
//     'outer: loop {
//         let mut sand = (500, 0);

//         if all_sand.contains(&sand) {
//             break 'outer;
//         }

//         loop {
//             let (_, y) = sand;

//             let down = (sand.0, sand.1 + 1);
//             let left = (sand.0 - 1, sand.1 + 1);
//             let right = (sand.0 + 1, sand.1 + 1);

//             if !paths.iter().any(|path| path.contains(&down))
//                 && !all_sand.contains(&down)
//                 && sand.1 < floor
//             {
//                 sand = down;
//                 continue;
//             }

//             if !paths.iter().any(|path| path.contains(&left))
//                 && !all_sand.contains(&left)
//                 && sand.1 < floor
//             {
//                 sand = left;
//                 continue;
//             }

//             if !paths.iter().any(|path| path.contains(&right))
//                 && !all_sand.contains(&right)
//                 && sand.1 < floor
//             {
//                 sand = right;
//                 continue;
//             }

//             all_sand.push(sand);
//             break;
//         }
//     }

//     draw2(paths.clone(), all_sand.clone(), (0, 0));
//     dbg!(all_sand.len());
// }

fn main() {
    // let input = include_str!("../example.txt");
    let input = include_str!("../input.txt");

    let paths = parser(input).unwrap().1;

    let paths = expand_paths(paths);

    // simulate(paths.clone());
    simulate2(paths.clone());
}
