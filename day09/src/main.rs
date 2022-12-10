use nom::{
    branch::alt, bytes::complete::tag, character::complete, combinator::map,
    sequence::separated_pair, *,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn dir_parser(i: &str) -> nom::IResult<&str, Dir> {
    alt((
        map(tag("U"), |_| Dir::Up),
        map(tag("D"), |_| Dir::Down),
        map(tag("L"), |_| Dir::Left),
        map(tag("R"), |_| Dir::Right),
    ))(i)
}

fn line(i: &str) -> nom::IResult<&str, (Dir, u32)> {
    return separated_pair(dir_parser, tag(" "), complete::u32)(i);
}

fn parser(i: &str) -> nom::IResult<&str, Vec<(Dir, u32)>> {
    nom::multi::separated_list1(nom::character::complete::newline, line)(i)
}

fn move_head(dir: Dir, pos: (i32, i32)) -> (i32, i32) {
    return match dir {
        Dir::Up => (pos.0, pos.1 + 1),
        Dir::Down => (pos.0, pos.1 - 1),
        Dir::Left => (pos.0 - 1, pos.1),
        Dir::Right => (pos.0 + 1, pos.1),
    };
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let (dx, dy) = (head.0 - tail.0, head.1 - tail.1);

    if dx.abs() <= 1 && dy.abs() <= 1 {
        return tail;
    }

    let nx = match dx {
        x if x > 0 => 1,
        x if x < 0 => -1,
        _ => 0,
    };

    let ny = match dy {
        y if y > 0 => 1,
        y if y < 0 => -1,
        _ => 0,
    };

    return (tail.0 + nx, tail.1 + ny);
}

fn main() {
    let text = include_str!("../input.txt");

    let example_input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    let moves = parser(text).unwrap().1;

    let len = 10;

    let mut snake = vec![(0, 0)];

    for _ in 0..len {
        snake.push((0, 0));
    }

    let mut tiles: BTreeMap<(i32, i32), i32> = BTreeMap::new();
    for (dir, n_steps) in moves {
        for _ in 0..n_steps {
            snake[0] = move_head(dir, snake[0]);

            for i in 1..len {
                snake[i] = move_tail(snake[i - 1], snake[i]);
            }

            tiles
                .entry(snake[len - 1])
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    println!("{:?}", tiles.len());
}
