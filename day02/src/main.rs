fn main() {
    let input = include_str!("../input.txt");

    let games: Vec<[&str; 2]> = input
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .filter(|&s| s != "")
        .map(|s| {
            let split = s.split(" ").collect::<Vec<&str>>();
            let a = split[0];
            let b = split[1];
            return [a, b];
        })
        .collect();

    let mut score = 0;
    for game in &games {
        match game {
            ["A", "X"] => score += 1 + 3,
            ["A", "Y"] => score += 2 + 6,
            ["A", "Z"] => score += 3,
            ["B", "X"] => score += 1,
            ["B", "Y"] => score += 2 + 3,
            ["B", "Z"] => score += 3 + 6,
            ["C", "X"] => score += 1 + 6,
            ["C", "Y"] => score += 2,
            ["C", "Z"] => score += 3 + 3,
            _ => println!("nope"),
        }
    }

    println!("Part 1: {:?}", score);
}
