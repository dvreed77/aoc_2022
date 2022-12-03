fn main() {
    let input = include_str!("../input.txt");

    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let sacks = input
        .split("\n")
        .filter(|&s| s != "")
        .map(|s| {
            let len = s.len();
            let a = &s[..len / 2];
            let b = &s[len / 2..];
            // let mut sack = vec![];

            return [a, b];
        })
        .collect::<Vec<[&str; 2]>>();

    let mut sum = 0;
    'outer: for sack in &sacks {
        let pocket1 = sack[0];
        let pocket2 = sack[1];

        for c in pocket1.chars() {
            if pocket2.contains(c) {
                let idx = alphabet.chars().position(|c1| c1 == c).unwrap() + 1;
                // println!("{} {}-> {} {}", c, idx, pocket1, pocket2);
                sum += idx;

                continue 'outer;
            }
        }
    }
    println!("Part 1: {:?}", sum);

    let sacks: Vec<&str> = input
        .split("\n")
        .filter(|&s| s != "")
        .collect::<Vec<&str>>();

    let sacks3: Vec<&[&str]> = sacks.chunks(3).collect();

    let mut sum = 0;
    'outer: for sack in sacks3 {
        let a = sack[0];
        let b = sack[1];
        let c = sack[2];
        println!("{} {} {}", a, b, c);

        for d in a.chars() {
            if b.contains(d) && c.contains(d) {
                let idx = alphabet.chars().position(|c1| c1 == d).unwrap() + 1;
                // println!("{} {}-> {} {}", c, idx, pocket1, pocket2);
                sum += idx;

                continue 'outer;
            }
        }
    }

    println!("Part 2: {:?}", sum);

    // println!("{:?}", a);
}
