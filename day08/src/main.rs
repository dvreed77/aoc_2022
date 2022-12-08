fn parse_input(i: &str) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = vec![];

    for line in i.lines() {
        let mut row: Vec<u32> = vec![];
        for num in line.chars().collect::<Vec<char>>() {
            // dbg!(&num);
            // if num != "\n" {
            row.push((num.to_string()).parse::<u32>().unwrap());
            // }
        }
        map.push(row);
    }

    return map;
}

fn main() {
    let text = include_str!("../input.txt");

    let example = "30373
25512
65332
33549
35390";

    let map = parse_input(text);

    let size = map[0].len();
    let size1 = map.iter().map(|x| x[0]).len();
    dbg!(size, size1);

    let mut s = 2 * size + 2 * (size - 2);

    for i in 1..size - 1 {
        for j in 1..size - 1 {
            let a = map[i][j];

            let left = &map[i][0..j].iter().all(|x| x < &a);
            let right = &map[i][j + 1..].iter().all(|x| x < &a);
            let top = &map[..i].iter().map(|x| x[j]).all(|x| x < a);
            let bottom = &map[i + 1..].iter().map(|x| x[j]).all(|x| x < a);

            if *left || *right || *top || *bottom {
                s += 1;
            }
        }
    }

    dbg!(s);

    let mut scores: Vec<u32> = vec![];

    for i in 1..size - 1 {
        for j in 1..size - 1 {
            let a = map[i][j];

            let mut left_distance = 0;
            let mut right_distance = 0;
            let mut up_distance = 0;
            let mut down_distance = 0;

            let mut left = map[i][0..j].to_owned();

            left.reverse();

            for x in &left {
                left_distance += 1;
                if x >= &a {
                    break;
                }
            }
            let right = &map[i][j + 1..];
            for x in right {
                right_distance += 1;
                if x >= &a {
                    break;
                }
            }

            let mut top = map[..i].iter().map(|x| x[j].clone()).collect::<Vec<u32>>();

            top.reverse();
            for x in top {
                up_distance += 1;
                if x >= a {
                    break;
                }
            }

            let bottom = &map[i + 1..].iter().map(|x| x[j]).collect::<Vec<u32>>();
            for x in bottom {
                down_distance += 1;
                if x >= &a {
                    break;
                }
            }

            // if (i == 1 && j == 2) {
            //     dbg!(left, right, top, bottom);
            // }

            let score = left_distance * right_distance * up_distance * down_distance;
            scores.push(score);
        }
    }
    dbg!(&scores.iter().max());
}

// 1564

// 2795520 for part 2 too high
