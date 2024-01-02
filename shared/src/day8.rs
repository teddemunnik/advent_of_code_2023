use std::collections::HashMap;

use nom::{
    IResult,
    character::complete::{alpha1, char, multispace0},
    bytes::complete::take_while,
    branch::alt,
    multi::many1,
    combinator::{value,map},
    sequence::{tuple,delimited,separated_pair, preceded, terminated}
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Command{
    L,
    R
}

#[derive(Debug, PartialEq, Eq)]
struct NodeDescription<'a>{
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse_command(input: &str) -> IResult<&str, Command>{
    alt((value(Command::L, char('L')), value(Command::R, char('R'))))(input)
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>>{
    many1(parse_command)(input)
}

fn parse_node(input: &str) -> IResult<&str, NodeDescription>{
    map(
        tuple((
            alpha1,
            preceded(multispace0, char('=')),
            delimited(
                preceded(multispace0, char('(')),
                separated_pair(delimited(multispace0, alpha1, multispace0), char(','), delimited(multispace0, alpha1, multispace0)), 
                terminated(char(')'), multispace0)
            ))
        ), 
        |(name, _, (left, right))|{
            NodeDescription{
                name,
                left,
                right
            }
        }
    )(input)
}


#[derive(Eq, PartialEq, Hash, Debug)]
struct NodeId(String);

impl From<&str> for NodeId{
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node{
    left: NodeId,
    right: NodeId,
}

struct Map{
    commands: Vec<Command>,
    nodes: std::collections::HashMap<NodeId, Node>
}

fn parse_map<R: std::io::BufRead>(input: R) -> Option<Map>{
    let mut lines = input.lines();
    let (_, commands) = parse_commands(&lines.next()?.ok()?).ok()?;
    lines.next()?.ok()?;


    let mut nodes = HashMap::new();
    for line in lines{
        let input = line.ok()?;
        let (_, node) = parse_node(&input).ok()?;
        nodes.insert(NodeId(node.name.into()), Node{
            left: NodeId(node.left.into()),
            right: NodeId(node.right.into()),
        });
    }

    Some(Map{
        commands,
        nodes
    })
}

#[aoc_2023_markup::aoc_task(2023, 8, 1)]
fn follow_map<R: std::io::BufRead>(input: R) -> Option<usize>{
    let map = parse_map(input)?;

    let mut commands = map.commands.iter().cycle();

    let start = NodeId::from("AAA");
    let end = NodeId::from("ZZZ");

    let mut current = &start;
    let mut steps = 0;

    loop{
        let command = commands.next()?;

        let next = match command{
            Command::L => &map.nodes[current].left,
            Command::R => &map.nodes[current].right
        };

        steps += 1;

        if *next == end{
            return Some(steps);
        }

        if *next == *current{
            return None; 
        }

        current = next;
    }
}

#[cfg(test)]
mod tests{
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_parse_commands(){
        let (_, res) = parse_commands("LLRL").unwrap();
        assert_eq!(res, [
            Command::L,
            Command::L,
            Command::R,
            Command::L
        ])
    }

    #[test]
    fn test_parse_node(){
        let (_, res) = parse_node("AAA = (BBB, CCC)").unwrap();
        assert_eq!(res, NodeDescription{
            name: "AAA",
            left: "BBB",
            right: "CCC"
        });
    }

    const INPUT : &[u8] = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)       
    "}.as_bytes();

    #[test]
    fn test_parse_map(){

        let map = parse_map(INPUT).unwrap();
        assert_eq!(map.nodes.len(), 7);
        assert_eq!(map.nodes.get(&NodeId::from("AAA")), Some(Node{
            left: NodeId::from("BBB"),
            right: NodeId::from("CCC")
        }).as_ref());
    }

    #[test]
    fn test_follow_map(){
        let count = follow_map(INPUT).unwrap();
        assert_eq!(count, 2);
    }

}
