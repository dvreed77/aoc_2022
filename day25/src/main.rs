use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    *,
};

fn line_parser(i: &str) -> IResult<&str, Vec<i64>> {
    let (input, a) = many1(one_of("210=-"))(i)?;

    let out = a
        .iter()
        .map(|x| match x {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("other char"),
        })
        .collect();

    Ok((input, out))
}

fn parser(i: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(newline, line_parser)(i)
}

fn convert(five_vec: &Vec<i64>) -> i64 {
    let mut out = 0;

    for (idx1, idx2) in (0..five_vec.len()).rev().enumerate() {
        let m = (5 as i64).pow(idx1 as u32);
        let b = five_vec[idx2];
        out += b * m;
    }

    out
}

fn convert_back(i: i64) -> String {
    let mut d = i.clone() as f64;
    let mut out = "".to_string();
    loop {
        let a = d / 5.0;

        let mut carry = 0.0;
        let r = ((a - a.floor()) * 5.0).round() as u64;

        let c = match r {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => {
                carry = 1.0;
                "="
            }
            4 => {
                carry = 1.0;
                "-"
            }
            _ => panic!("weird"),
        };

        d = a.floor() + carry;

        out = c.to_owned() + &out;
        if d == 0.0 {
            break;
        }
    }

    out
}

fn main() {
    // let input = include_str!("../example.txt");
    let input = include_str!("../input.txt");

    let fives = parser(input).unwrap().1;

    let converted = fives.iter().map(|f| convert(f)).sum::<i64>();
    dbg!(converted);
    dbg!(convert_back(converted));
}
