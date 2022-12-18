use std::collections::{HashMap, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list1,
    *,
};

type Cube = (i32, i32, i32);

fn parser(i: &str) -> IResult<&str, Vec<Cube>> {
    separated_list1(
        newline,
        map(separated_list1(tag(","), complete::i32), |d| {
            (d[0], d[1], d[2])
        }),
    )(i)
}

fn get_neighbors<'a>(
    (x, y, z): &'a Cube,
    cube_map: &HashMap<&'a Cube, bool>,
) -> (Vec<&'a Cube>, u32) {
    let mut out = vec![];

    let n = [
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
    ];

    let mut exposed_sides = 0;
    for (dx, dy, dz) in n {
        let k = (x + dx, y + dy, z + dz);

        if let Some((cube, visited)) = cube_map.get_key_value(&k) {
            if !visited {
                out.push(*cube)
            }
        } else {
            exposed_sides += 1;
        }
    }

    return (out, exposed_sides);
}

fn part1(cubes: &Vec<Cube>) {
    let mut cube_map: HashMap<&Cube, bool> = cubes
        .into_iter()
        .map(|x| (x, false))
        .collect::<HashMap<_, _>>();

    let mut queue = VecDeque::new();
    let mut sa = 0;
    queue.push_back(&cubes[0]);

    while queue.len() > 0 {
        let curr = queue.pop_front().unwrap();

        if let Some(x) = cube_map.get(curr) {
            if *x {
                continue;
            }
        }

        cube_map.entry(curr).and_modify(|x| *x = true);

        // dbg!(curr);

        let (neighbors, exposed_sides) = get_neighbors(&curr, &cube_map);
        // dbg!(&curr, &neighbors);

        sa += exposed_sides;

        for n in neighbors {
            queue.push_back(n);
        }

        if queue.len() == 0 {
            if let Some(x) = cube_map.iter().find(|x| !x.1) {
                queue.push_back(x.0)
            }
        }
    }

    dbg!(sa);
}

fn main() {
    let input = include_str!("../input.txt");
    let cubes = parser(input).unwrap().1;

    // let n = get_neighbors(&cubes[0]);

    // dbg!(n, &cubes[0]);
    part1(&cubes);
}
