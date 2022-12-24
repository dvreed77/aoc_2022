use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::{terminated, tuple},
    *,
};

#[derive(Debug)]
enum Material {
    Ore(u32),
    Clay(u32),
    Obsidian(u32),
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot: Vec<Material>,
    clay_robot: Vec<Material>,
    obsidian_robot: Vec<Material>,
    geode_robot: Vec<Material>,
}

fn material_parser(i: &str) -> IResult<&str, Vec<Material>> {
    separated_list0(
        tag(" and "),
        alt((
            terminated(complete::u32, tag(" ore")).map(|x| Material::Ore(x)),
            terminated(complete::u32, tag(" clay")).map(|x| Material::Clay(x)),
            terminated(complete::u32, tag(" obsidian")).map(|x| Material::Obsidian(x)),
        )),
    )(i)
}

fn line_parser(i: &str) -> IResult<&str, Blueprint> {
    let (input, (_, id, _, ore_robot, _, clay_robot, _, obsidian_robot, _, geode_robot, _)) =
        tuple((
            tag("Blueprint "),
            complete::u32,
            tag(": Each ore robot costs "),
            material_parser,
            tag(". Each clay robot costs "),
            material_parser,
            tag(". Each obsidian robot costs "),
            material_parser,
            tag(". Each geode robot costs "),
            material_parser,
            tag("."),
        ))(i)?;

    return Ok((
        input,
        Blueprint {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        },
    ));
}

fn parser(i: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(newline, line_parser)(i)
}

fn main() {
    let input = include_str!("../example.txt");
    // let input = include_str!("../input.txt");

    let blueprints = parser(input).unwrap().1;

    dbg!(blueprints);
}
