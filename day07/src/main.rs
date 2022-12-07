use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{self, alphanumeric1, newline, none_of, not_line_ending},
    combinator::eof,
    multi::{fold_many0, fold_many1, separated_list1},
    sequence::{preceded, separated_pair},
    *,
};

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

#[derive(Debug)]
struct Dir {
    path: Vec<String>,
    files: Vec<String>,
    // dirs: Vec<Dir>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

fn dir_parser(i: &str) -> IResult<&str, &str> {
    preceded(tag("dir "), alphanumeric1)(i)
}

fn file_parser(i: &str) -> IResult<&str, (u64, &str)> {
    let (input, a) = separated_pair(complete::u64, tag(" "), alphanumeric1)(i)?;

    return Ok((input, a));
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
    let out2 = parser3(example2).unwrap().1;
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

        out3.push(Dir {
            path: cur_path.clone(),
            files: p.1.clone().into_iter().map(|s| s.to_string()).collect(),
        })
    }

    // dbg!(out3);

    dbg!(file_parser("8033020 d.log"));
}
