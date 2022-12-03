fn main() {
    let my_str = include_str!("../input.txt");

    let a = my_str.split("\n").collect::<Vec<&str>>();

    let mut elves = vec![];

    let mut sum = 0;
    for i in 0..a.len() {
        let n = a[i];

        match n.parse::<i32>() {
            Ok(n) => {
                sum += n;
            }
            Err(_) => {
                elves.push(sum);
                sum = 0;
            }
        }
    }
    elves.sort();
    println!("Part 1: {:?}", elves.last());
    println!(
        "Part 2: {:?}",
        &elves[elves.len() - 3..].iter().sum::<i32>()
    );
}
