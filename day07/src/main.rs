use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{self, alphanumeric1, newline, none_of, not_line_ending},
    combinator::{eof, opt},
    multi::{fold_many0, fold_many1, separated_list1},
    sequence::{pair, preceded, separated_pair},
    *,
};

fn filename_parser(i: &str) -> nom::IResult<&str, (&str, Option<&str>)> {
    pair(alphanumeric1, opt(preceded(tag("."), alphanumeric1)))(i)
}

fn ls_parser(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::tag("$ ls")(i)
}

fn hello_parser(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::tag("hello")(i)
}

fn line(i: &str) -> nom::IResult<&str, &str> {
    // nom::bytes::complete::take_until("$")(i)
    return nom::IResult::Ok(("", i));
}

fn dave_parser(i: &str) -> nom::IResult<&str, Vec<&str>> {
    nom::multi::separated_list1(nom::character::complete::newline, line)(i)
}

fn line_parser(s: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(newline, not_line_ending)(s)
}

fn cur_dir_parser(i: &str) -> IResult<&str, String> {
    let (input, s) = preceded(tag("$ cd "), cd_name_parser)(i)?;

    Ok((input, s.to_string()))
}

fn contents_parser(i: &str) -> IResult<&str, Vec<&str>> {
    preceded(tag("\n$ ls\n"), separated_list1(newline, not_line_ending))(i)
}

#[derive(Debug, Clone)]
struct Dir {
    path: Vec<String>,
    children: Vec<Child>,
    // dirs: Vec<Dir>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

fn dir_parser(i: &str) -> IResult<&str, Child> {
    let (input, name) = preceded(tag("dir "), alphanumeric1)(i)?;

    return Ok((input, Child::Dir(name.to_string())));
}

fn file_parser(i: &str) -> IResult<&str, Child> {
    let (input, a) = separated_pair(complete::u64, tag(" "), filename_parser)(i)?;

    return Ok((input, Child::File(a.0, a.1 .0.to_string())));
}

fn till_dollar(s: &str) -> IResult<&str, &str> {
    take_until("$ cd")(s)
}

fn dave3_parser(i: &str) -> IResult<&str, (String, Vec<&str>)> {
    let (input, s) = cur_dir_parser(i)?;

    let (input, lines) = till_dollar(input)?;

    if lines == "\n" {
        return Ok((input, (s, vec![])));
    }
    let (_, files) = contents_parser(lines)?;

    let files = files
        .into_iter()
        .filter(|&x| x != "")
        .collect::<Vec<&str>>();

    Ok((input, (s, files)))
}

fn parser3(s: &str) -> IResult<&str, Vec<(String, Vec<&str>)>> {
    let (input, mut t) = fold_many1(dave3_parser, Vec::new, |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
    })(s)?;

    // parse the last one
    let (input, s) = cur_dir_parser(input)?;
    let (_, files) = contents_parser(input)?;
    let files = files
        .into_iter()
        .filter(|&x| x != "")
        .collect::<Vec<&str>>();
    t.push((s, files));

    return Ok(("", t));
}

fn cd_name_parser(s: &str) -> IResult<&str, String> {
    fold_many1(none_of("\n$"), String::new, |mut acc: String, item| {
        acc.push(item);
        acc
    })(s)
}

#[derive(Debug, Clone)]
enum Child {
    Dir(String),
    File(u64, String),
}

fn calc_size2<'a>(
    mut size_btree: BTreeMap<Vec<String>, u64>,
    d: &Dir,
) -> BTreeMap<Vec<String>, u64> {
    for i in 0..d.path.len() {
        let sum = d
            .children
            .iter()
            .filter_map(|c| match c {
                Child::Dir(_) => None,
                Child::File(size, _) => Some(size),
            })
            .sum::<u64>();

        // let sum = 0;
        size_btree
            .entry(d.path[0..=i].to_vec())
            .and_modify(|v| *v += sum)
            .or_insert(sum);
    }

    return size_btree;
}
fn main() {
    let text = include_str!("../input.txt");

    let example = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    // println!("{:?}", hello_parser("hello world"));
    // println!("{:?}", ls_parser("$ ls"));

    // println!("{:?}", dave_parser(example));

    // println!("{:?}", parser(example));

    let example1 = "$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst";

    // println!("{:?}", dir_parser(example1));

    let example2 = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    // println!("{:?}", parser2("$ cd /$ cd dave$ cd ..$ cd ..$ cd d"));
    // println!("{:?}", parser3("$ cd /$ cd dave\n$ cd ..$ cd ..$ cd d"));
    let out2 = parser3(text).unwrap().1;
    // dbg!(&out2);

    // let path = vec![];

    let mut cur_path: Vec<String> = vec![];

    let mut out3 = vec![];
    for p in &out2 {
        let s = p.0.to_owned();

        if s == ".." {
            cur_path.pop();
            continue;
        }
        cur_path.push(s);

        let children =
            p.1.clone()
                .into_iter()
                .map(|s| {
                    let (_, s) = alt((dir_parser, file_parser))(s).unwrap();
                    return s;
                })
                .collect::<Vec<Child>>();

        out3.push(Dir {
            path: cur_path.clone(),
            children,
        })
    }

    let b = out3.iter().fold(BTreeMap::new(), calc_size2);

    let total = b
        .iter()
        .filter(|(_, &size)| size < 100_000)
        .map(|(_, size)| size)
        .sum::<u64>()
        .to_string();

    dbg!(total);

    let root = vec!["/".to_string()];

    let max_total = b.get(&root).unwrap();

    let current_free_space = 70_000_000 - max_total;
    let need_to_free_at_least = 30_000_000 - current_free_space;
    let total2 = b
        .iter()
        .filter(|(_, &size)| size > need_to_free_at_least)
        .map(|(_, size)| size)
        // .collect::<Vec<&u64>>();
        .min();

    // 700 - 483 < 300, yes 217
    // 700 - 249 < 300, no

    dbg!(total2);
}
