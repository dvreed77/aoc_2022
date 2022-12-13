use std::collections::{HashSet, VecDeque};

// #[derive(Debug, Clone, Copy)]
// enum Tile {
//     Start { x: i32, y: i32, h: i32 },
//     End { x: i32, y: i32, h: i32 },
//     Path { x: i32, y: i32, h: i32 },
// }
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Tile {
    idx: usize,
    x: i32,
    y: i32,
    h: i32,
}

fn x_y_to_idx(x: usize, y: usize, width: usize) -> usize {
    (y * width + x) as usize
}

fn idx_to_x_y(idx: usize, width: usize) -> (usize, usize) {
    let x = idx % width;
    let y = idx / width;

    (x, y)
}

fn get_neighbors(idx: usize, width: usize, height: usize) -> Vec<usize> {
    let (x, y) = idx_to_x_y(idx, width);

    let mut neighbors = Vec::new();

    if x > 0 {
        neighbors.push(x_y_to_idx((x - 1) as usize, y as usize, width));
    }

    if x < width - 1 {
        neighbors.push(x_y_to_idx((x + 1) as usize, y as usize, width));
    }

    if y > 0 {
        neighbors.push(x_y_to_idx(x as usize, (y - 1) as usize, width));
    }

    if y < height - 1 {
        neighbors.push(x_y_to_idx(x as usize, (y + 1) as usize, width));
    }

    neighbors
}

fn parse_input(i: &str) -> (Vec<Tile>, usize, usize, usize, usize) {
    let mut tiles = Vec::new();

    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    let mut s_idx: usize = 0;
    let mut e_idx: usize = 0;
    let width = i.lines().next().unwrap().len();
    let height = i.lines().count();
    for (y, line) in i.lines().enumerate() {
        let chars = line.chars();
        for (x, c) in chars.enumerate() {
            let tile = match c {
                'S' => {
                    s_idx = x_y_to_idx(x, y, width);

                    let h = alphabet
                        .chars()
                        .position(|c1| c1 == 'a')
                        .expect(&format!("{}", c)) as i32;

                    Tile {
                        idx: s_idx,
                        x: x as i32,
                        y: y as i32,
                        h,
                    }
                }
                'E' => {
                    e_idx = x_y_to_idx(x, y, width);

                    let h = alphabet
                        .chars()
                        .position(|c1| c1 == 'z')
                        .expect(&format!("{}", c)) as i32;

                    Tile {
                        idx: e_idx,
                        x: x as i32,
                        y: y as i32,
                        h,
                    }
                }
                _ => {
                    let h = alphabet
                        .chars()
                        .position(|c1| c1 == c)
                        .expect(&format!("{}", c)) as i32;

                    Tile {
                        idx: x_y_to_idx(x, y, width),
                        x: x as i32,
                        y: y as i32,
                        h,
                    }
                }
            };

            tiles.push(tile);
        }
    }

    return (tiles, s_idx, e_idx, width, height);
}

fn main() {
    let input = include_str!("../input.txt");

    // let input = include_str!("../example.txt");

    let (tiles, s_idx, e_idx, width, height) = parse_input(input);

    let mut queue = VecDeque::from(vec![(tiles[s_idx], 0)]);
    let mut visited: HashSet<Tile> = HashSet::from_iter(vec![tiles[s_idx]]);

    let d = loop {
        if queue.len() == 0 {
            break -1;
        }

        let (t, dist) = queue.pop_front().unwrap();

        if t.idx == e_idx {
            break dist;
        }

        let neighbors = get_neighbors(t.idx, width, height);

        for n in neighbors {
            if !visited.contains(&tiles[n]) && (tiles[n].h - t.h <= 1) {
                queue.push_back((tiles[n], dist + 1));
                visited.insert(tiles[n]);
            }
        }
    };

    println!("Part 1: {}", d);

    let starts = tiles
        .iter()
        .filter(|t| t.h == 0)
        .map(|t| t.idx)
        .collect::<Vec<usize>>();

    // dbg!(&starts);

    let mut dists = vec![];
    for s in starts {
        let mut queue = VecDeque::from(vec![(tiles[s], 0)]);
        let mut visited: HashSet<Tile> = HashSet::from_iter(vec![tiles[s]]);

        let d = loop {
            if queue.len() == 0 {
                break None;
            }

            let (t, dist) = queue.pop_front().unwrap();

            if t.idx == e_idx {
                break Some(dist);
            }

            let neighbors = get_neighbors(t.idx, width, height);

            for n in neighbors {
                if !visited.contains(&tiles[n]) && (tiles[n].h - t.h <= 1) {
                    queue.push_back((tiles[n], dist + 1));
                    visited.insert(tiles[n]);
                }
            }
        };

        dists.push(d);
    }

    let f = dists.iter().filter_map(|&d| d).collect::<Vec<i32>>();
    dbg!(f.iter().min().unwrap());

    // println!("Part 1: {}", d);
}
