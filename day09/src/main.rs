use std::collections::BTreeMap;

fn parse_instructions(i: &str) -> Vec<(&str, i32)> {
    let d = i
        .lines()
        .map(|x| {
            let x1 = x.split(" ").collect::<Vec<&str>>();
            let d = x1[0];
            let l = x1[1].parse::<i32>().unwrap();

            return (d, l);
        })
        .collect::<Vec<(&str, i32)>>();

    return d;
}

fn move_head(dir: &str, pos: (i32, i32)) -> (i32, i32) {
    return match dir {
        "U" => (pos.0, pos.1 + 1),
        "D" => (pos.0, pos.1 - 1),
        "L" => (pos.0 - 1, pos.1),
        "R" => (pos.0 + 1, pos.1),
        _ => {
            println!("Unknown direction: {}", dir);
            (pos.0, pos.1)
        }
    };
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let (dx, dy) = (head.0 - tail.0, head.1 - tail.1);

    // println!("DX{} DY{}", dx, dy);

    if dx.abs() <= 1 && dy.abs() <= 1 {
        return tail;
    }
    let new_tail = match (dx, dy) {
        (2, 2) => (tail.0 + 1, tail.1 + 1),
        (1, 2) => (tail.0 + 1, tail.1 + 1),
        (0, 2) => (tail.0, tail.1 + 1),
        (-1, 2) => (tail.0 - 1, tail.1 + 1),
        (-2, 2) => (tail.0 - 1, tail.1 + 1),

        (-2, 1) => (tail.0 - 1, tail.1 + 1),
        (-2, 0) => (tail.0 - 1, tail.1),
        (-2, -1) => (tail.0 - 1, tail.1 - 1),

        (-2, -2) => (tail.0 - 1, tail.1 - 1),
        (-1, -2) => (tail.0 - 1, tail.1 - 1),
        (0, -2) => (tail.0, tail.1 - 1),
        (1, -2) => (tail.0 + 1, tail.1 - 1),
        (2, -2) => (tail.0 + 1, tail.1 - 1),

        (2, -1) => (tail.0 + 1, tail.1 - 1),
        (2, 0) => (tail.0 + 1, tail.1),
        (2, 1) => (tail.0 + 1, tail.1 + 1),

        _ => {
            println!("OTHER");
            return (tail.0, tail.1);
        }
    };

    return new_tail;
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

    let moves = parse_instructions(text);
    // let mut head = (0, 0);
    // let mut tail = (0, 0);

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
            // println!("{} {} {:?} {:?}", dir, n_steps, head, tail);
        }
    }

    println!("{:?}", tiles.len());
}
