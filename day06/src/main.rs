use std::collections::HashSet;

fn main() {
    let text = include_str!("../input.txt");

    // let text = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    for i in 0..text.len() - 4 {
        let s = &text[i..i + 4];
        let char_vec: Vec<char> = s.chars().collect();
        let t: HashSet<char> = HashSet::from_iter(char_vec);
        if t.len() == 4 {
            println!("Part 1: {:?}", i + 4);
            break;
        }
    }

    for i in 0..text.len() - 14 {
        let s = &text[i..i + 14];
        let char_vec: Vec<char> = s.chars().collect();
        let t: HashSet<char> = HashSet::from_iter(char_vec);
        if t.len() == 14 {
            println!("Part 2: {:?}", i + 14);
            break;
        }
    }
}
