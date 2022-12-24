use tracing::*;
use tracing_subscriber;

fn parser(i: &str) -> Vec<(usize, i64)> {
    i.split("\n")
        .into_iter()
        .enumerate()
        .map(|(idx, x)| (idx, x.parse::<i64>().unwrap()))
        .collect::<Vec<_>>()
}

#[instrument(skip(seq))]
fn part1(seq: &Vec<(usize, i64)>) {
    let mut state = seq.clone();
    let l = seq.len() as i64;

    info!("{:?}", &state.iter().map(|x| x.1).collect::<Vec<_>>());
    // println!();
    for (id, val) in seq {
        let idx = state.iter().position(|x| x.0 == *id).unwrap();

        let state_before = state.clone();

        let current = state.remove(idx);

        let idx_a = idx as i64 + current.1;
        let idx_b = (idx + 1) as i64 + current.1;
        let idx_a1 = idx_a.rem_euclid(state.len() as i64) as usize;
        let idx_b1 = ((idx_a1 + 1) as i64).rem_euclid(state.len() as i64) as usize;

        let b1 = state[idx_a1].1;
        let b2 = state[idx_b1].1;
        info!(
            "{val}, which is at index {idx}, moves between {b1} and {b2}. {idx} + {val} = {idx_a}({idx_a1}), {idx_b}({idx_b1})",
        );

        state.insert(idx_a1, current);
        info!("{:?}", &state.iter().map(|x| x.1).collect::<Vec<_>>());
    }

    let idx = state.iter().position(|x| x.1 == 0).unwrap() as i64;
    let a = state[((idx + 1000) % l) as usize];
    let b = state[((idx + 2000) % l) as usize];
    let c = state[((idx + 3000) % l) as usize];

    let out = a.1 + b.1 + c.1;
    println!("{}", out);
}

#[instrument(skip(seq))]
fn part2(seq: &Vec<(usize, i64)>) {
    let seq2 = seq
        .clone()
        .iter()
        .map(|x| (x.0, x.1 * 811589153))
        .collect::<Vec<_>>();

    let mut state = seq2.clone();
    let l = seq2.len() as i64;

    info!("{:?}", &state.iter().map(|x| x.1).collect::<Vec<_>>());
    // println!();
    for _ in 0..10 {
        for (id, val) in seq2.iter() {
            let idx = state.iter().position(|x| x.0 == *id).unwrap();

            let current = state.remove(idx);

            let idx_a = idx as i64 + current.1;
            let idx_b = (idx + 1) as i64 + current.1;
            let idx_a1 = idx_a.rem_euclid(state.len() as i64) as usize;
            let idx_b1 = ((idx_a1 + 1) as i64).rem_euclid(state.len() as i64) as usize;

            let b1 = state[idx_a1].1;
            let b2 = state[idx_b1].1;
            info!(
            "{val}, which is at index {idx}, moves between {b1} and {b2}. {idx} + {val} = {idx_a}({idx_a1}), {idx_b}({idx_b1})",
        );

            state.insert(idx_a1, current);
            info!("{:?}", &state.iter().map(|x| x.1).collect::<Vec<_>>());
        }
    }

    let idx = state.iter().position(|x| x.1 == 0).unwrap() as i64;
    let a = state[((idx + 1000) % l) as usize];
    let b = state[((idx + 2000) % l) as usize];
    let c = state[((idx + 3000) % l) as usize];

    let out = a.1 + b.1 + c.1;
    println!("{}", out);
}
fn main() {
    tracing_subscriber::fmt::init();
    // let input = include_str!("../example.txt");
    let input = include_str!("../input.txt");
    let seq = parser(input);

    part1(&seq);
    part2(&seq);
}

// 5691 to low
// 7539 high
