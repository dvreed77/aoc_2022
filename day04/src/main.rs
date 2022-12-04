fn main() {
    let input = include_str!("../input.txt")
        .split("\n")
        .filter(|&s| s != "")
        .collect::<Vec<&str>>();

    let pairs: Vec<[i32; 4]> = input
        .into_iter()
        .map(|d| {
            let a = d.split(",").collect::<Vec<&str>>();
            let p1 = a[0].split('-').collect::<Vec<&str>>();
            let p11 = p1[0].parse::<i32>().unwrap();
            let p12 = p1[1].parse::<i32>().unwrap();

            let p2 = a[1].split('-').collect::<Vec<&str>>();
            let p21 = p2[0].parse::<i32>().unwrap();
            let p22 = p2[1].parse::<i32>().unwrap();
            return [p11, p12, p21, p22];
        })
        .collect();

    let mut count = 0;
    for [a, b, c, d] in &pairs {
        if a >= c && b <= d {
            count += 1;
        } else if c >= a && d <= b {
            count += 1;
        }
    }

    println!("Part 1: {:?}", count);

    let mut count = 0;
    for [a, b, c, d] in &pairs {
        if a >= c && a <= d {
            count += 1;
        } else if b >= c && b <= d {
            count += 1;
        } else if c >= a && c <= b {
            count += 1;
        } else if d >= a && d <= b {
            count += 1;
        }
    }

    println!("Part 2: {:?}", count);
}
