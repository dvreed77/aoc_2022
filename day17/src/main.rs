use std::collections::HashSet;

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

fn parse(i: &str) -> Vec<Move> {
    i.chars()
        .map(|c| match c {
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!("bad input"),
        })
        .collect()
}

type Point = (i32, i32);

const SHAPE1: [Point; 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const SHAPE2: [Point; 5] = [(1, 0), (0, 1), (1, 2), (2, 1), (1, 1)];
const SHAPE3: [Point; 5] = [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
const SHAPE4: [Point; 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const SHAPE5: [Point; 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];

struct Board {
    board: HashSet<Point>,
    top: i32,
}

impl Board {
    fn new() -> Self {
        Self {
            board: HashSet::new(),
            top: 0,
        }
    }

    fn lock_shape(&mut self, shape: &Shape) {
        for p in &shape.points {
            let point = (p.0 + shape.anchor.0, p.1 + shape.anchor.1);
            self.board.insert(point);
        }
        self.top = self.top.max(shape.get_max_y() + 1);
    }

    // fn simulate(&self, dir: Move) {
    //     self.active_shape.move_shape(dir);
    // }

    fn draw(&self, active_shape: &Shape) {
        for j in (0..self.top + 8).rev() {
            print!("|");
            for i in 0..7 {
                let point = (i, j);
                if self.board.contains(&point) {
                    print!("#");
                } else if active_shape.in_shape(point) {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        println!("+-------+");
    }
}

struct Shape {
    points: HashSet<Point>,
    anchor: Point,
}

impl Shape {
    fn new(n: u64) -> Self {
        let points = match n % 5 {
            0 => HashSet::from(SHAPE1),
            1 => HashSet::from(SHAPE2),
            2 => HashSet::from(SHAPE3),
            3 => HashSet::from(SHAPE4),
            4 => HashSet::from(SHAPE5),
            _ => panic!("bad shape"),
        };

        Self {
            points,
            anchor: (2, 3),
            // board,
        }
    }
    fn in_shape(&self, point: Point) -> bool {
        let pt = (point.0 - self.anchor.0, point.1 - self.anchor.1);
        self.points.contains(&pt)
    }
    fn init(&mut self, top: i32, n: u64) -> &Self {
        let points = match n % 5 {
            0 => HashSet::from(SHAPE1),
            1 => HashSet::from(SHAPE2),
            2 => HashSet::from(SHAPE3),
            3 => HashSet::from(SHAPE4),
            4 => HashSet::from(SHAPE5),
            _ => panic!("bad shape"),
        };
        self.anchor = (2, top + 3);
        self.points = points;
        self
    }

    fn get_max_y(&self) -> i32 {
        self.points.iter().map(|p| p.1).max().unwrap() + self.anchor.1
    }

    fn collides(&self, board: &Board) -> bool {
        for p in &self.points {
            let point = (p.0 + self.anchor.0, p.1 + self.anchor.1);
            if board.board.contains(&point) || point.0 > 6 || point.0 < 0 || point.1 < 0 {
                return true;
            }
        }
        return false;
    }

    fn move_shape(&mut self, m: &Move, board: &Board) -> &Self {
        match m {
            Move::Left => self.anchor.0 -= 1,
            Move::Right => self.anchor.0 += 1,
        }

        if self.collides(board) {
            match m {
                Move::Left => self.anchor.0 += 1,
                Move::Right => self.anchor.0 -= 1,
            }
        }

        self
    }

    fn move_down(&mut self, board: &Board) -> bool {
        self.anchor.1 -= 1;

        for p in &self.points {
            let point = (p.0 + self.anchor.0, p.1 + self.anchor.1);
            if board.board.contains(&point) || point.0 > 6 || point.0 < 0 || point.1 < 0 {
                self.anchor.1 += 1;
                return true;
            }
        }
        return false;
    }
}

fn main() {
    let input = include_str!("../example.txt");
    // let input = include_str!("../input.txt");

    let moves = parse(input);

    let mut time = 0;

    let mut n = 0;
    let mut s = Shape::new(n);

    let mut b = Board::new();

    // b.draw(&s);

    // b.draw();
    // dbg!(parse(input));

    while n < 1000000000000 {
        let move_ = &moves[time % moves.len()];
        // b.draw(&s);
        s.move_shape(&move_, &b);
        if s.move_down(&b) {
            b.lock_shape(&s);
            n += 1;
            s.init(b.top, n);
            // b.draw(&s);
            // println!("{}", b.top);
        }
        // b.draw(&s);
        time += 1;
    }

    dbg!(b.top);
}
