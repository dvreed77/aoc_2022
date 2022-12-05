use regex::Regex;

fn main() {
    let lines = include_str!("../input.txt")
        .split("\n")
        .collect::<Vec<&str>>();

    let map = &lines
        .clone()
        .into_iter()
        .filter(|l| !l.starts_with("move"))
        .filter(|&s| s != "")
        .collect::<Vec<&str>>();

    let n_lvls = map.len() - 1;
    let n_stacks = (map[n_lvls].len() + 1) / 4;

    let mut out: Vec<Vec<char>> = vec![];
    for _ in 0..n_stacks {
        out.push(vec![]);
    }

    for lvl in (0..n_lvls).rev() {
        for i in 0..n_stacks {
            let c = map[lvl].as_bytes()[i * 4 + 1] as char;
            if c != ' ' {
                out[i].push(c);
            }
        }
    }

    let instructions = lines
        .into_iter()
        .filter(|l| l.starts_with("move"))
        .collect::<Vec<&str>>();

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for instruction in &instructions {
        let cap = re.captures(instruction).unwrap();
        let n_moves = cap[1].parse::<i32>().unwrap();

        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;

        for _ in 0..n_moves {
            let c = out[from].pop().unwrap();
            out[to].push(c);
        }
    }

    let mut s = String::from("");
    for stack in &out {
        s.push(stack.last().unwrap().clone());
    }

    println!("Part 1: {:?}", s);

    let mut out: Vec<Vec<char>> = vec![];
    for _ in 0..n_stacks {
        out.push(vec![]);
    }

    for lvl in (0..n_lvls).rev() {
        for i in 0..n_stacks {
            let c = map[lvl].as_bytes()[i * 4 + 1] as char;
            if c != ' ' {
                out[i].push(c);
            }
        }
    }

    for instruction in &instructions {
        let cap = re.captures(instruction).unwrap();
        let n_moves = cap[1].parse::<usize>().unwrap();

        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;

        let l = out[from].len();

        let v1 = out[from].split_off(l - n_moves);
        out[to].append(&mut v1.clone());
    }

    let mut s = String::from("");
    for stack in &out {
        s.push(stack.last().unwrap().clone());
    }

    println!("Part 2: {:?}", s);
}
