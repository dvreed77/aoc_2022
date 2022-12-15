use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::tuple,
    *,
};

type Point = (i32, i32);

fn line_parser(i: &str) -> IResult<&str, (Point, Point)> {
    let (input, (_, sx, _, sy, _, bx, _, by)) = tuple((
        tag("Sensor at x="),
        complete::i32,
        tag(", y="),
        complete::i32,
        tag(": closest beacon is at x="),
        complete::i32,
        tag(", y="),
        complete::i32,
    ))(i)?;

    Ok((input, ((sx, sy), (bx, by))))
}

fn parser(i: &str) -> IResult<&str, Vec<(Point, Point)>> {
    separated_list1(newline, line_parser)(i)
}

fn sensor_range((sx, sy): Point, (bx, by): Point) -> (i32, i32, i32, i32) {
    let dx = (sx - bx).abs();
    let dy = (sy - by).abs();

    (sx + dx, sx - dx, sy - dy, sy + dy)
}

fn get_bounds(sensors: Vec<(Point, Point)>) -> (i32, i32, i32, i32) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for (sensor, beacon) in sensors {
        let (x1, x2, y1, y2) = sensor_range(sensor, beacon);

        min_x = min_x.min(x1).min(x2);
        max_x = max_x.max(x1).max(x2);
        min_y = min_y.min(y1).min(y2);
        max_y = max_y.max(y1).max(y2);
    }

    (min_x, max_x, min_y, max_y)
}

fn get_distances(sensors: Vec<(Point, Point)>) -> Vec<(Point, i32)> {
    sensors
        .iter()
        .map(|(sensor, beacon)| (sensor.clone(), get_distance(*sensor, *beacon)))
        .collect::<Vec<_>>()
}

fn get_min_bounds(sensor_dists: Vec<(Point, i32)>) -> Vec<(Point, i32, i32, i32, i32)> {
    sensor_dists
        .iter()
        .map(|((sx, sy), d)| {
            (
                (*sx, *sy).clone(),
                sx - d / 2,
                sx + d / 2,
                sy - d / 2,
                sy + d / 2,
            )
        })
        .collect::<Vec<(Point, i32, i32, i32, i32)>>()
}

fn get_distance((sx, sy): Point, (bx, by): Point) -> i32 {
    (sx - bx).abs() + (sy - by).abs()
}

fn is_in_range((s1x, s1y): Point, (s2x, s2y): Point, d: i32) -> bool {
    let d1 = get_distance((s1x, s1y), (s2x, s2y));

    d1 <= d
}

fn part1(sensors: Vec<(Point, Point)>) {
    let bounds = get_bounds(sensors.clone());

    dbg!(bounds);
    let s_dists = get_distances(sensors.clone());

    let beacons: HashSet<Point> = sensors.iter().map(|(_, b)| b.clone()).collect();

    let row = 2000000;
    let n_beacons = beacons.iter().filter(|b| b.1 == row).count();
    let mut count = 0;
    for i in bounds.0..bounds.1 {
        let this_s = (i, row);
        let a = s_dists
            .iter()
            .map(|(s, d)| is_in_range(this_s, *s, *d))
            .any(|x| x);

        if a {
            count += 1;
        }
    }

    dbg!(count, n_beacons, count - n_beacons);
}

fn part2(sensors: Vec<(Point, Point)>) {
    let s_dists = get_distances(sensors.clone());
    let bounds = get_min_bounds(s_dists.clone());

    let (min_x, max_x) = (0, 4_000_000);

    let rng = min_x..max_x;

    'b: for (s1, d1) in s_dists.clone() {
        let a = s1.0 - d1 - 1;
        let b = s1.0 + d1 + 1;
        for x in a..b {
            if x > max_x {
                break;
            } else if x < 0 {
                continue;
            }

            let dy = d1 - (x - s1.0).abs() + 1;
            let c = s1.1 + dy;
            let d = s1.1 - dy;
            'a: for y in [c, d] {
                if y <= max_x && y >= 0 {
                    for (s2, d2) in s_dists.clone() {
                        if (s2.0 - x).abs() + (s2.1 - y).abs() <= d2 {
                            break 'a;
                        }
                    }
                    dbg!(x, y, x as u64 * 4_000_000 + y as u64);
                    break 'b;
                }
            }
        }
    }
}

fn main() {
    // let input = include_str!("../example.txt");
    let input = include_str!("../input.txt");
    let sensors = parser(input).unwrap().1;

    // part1(sensors);
    part2(sensors);
}

// 4712648 (too low)
