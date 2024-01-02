use std::collections::HashMap;

use nom::{
    IResult,
    character::complete::{alphanumeric1, char, multispace0},
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
            alphanumeric1,
            preceded(multispace0, char('=')),
            delimited(
                preceded(multispace0, char('(')),
                separated_pair(delimited(multispace0, alphanumeric1, multispace0), char(','), delimited(multispace0, alphanumeric1, multispace0)), 
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

    let start = NodeId::from("AAA");
    let end = NodeId::from("ZZZ");

    let mut commands = map.commands.iter().cycle().enumerate();
    let mut current = &start;
    loop{
        let (step, command) = commands.next().unwrap();
        execute_command(&map, &mut current, command);
        if *current == end{
            return Some(step + 1);
        }
    }
}

fn execute_command<'a, 'b>(map: &'a Map, node: &mut &'b NodeId, command: &Command) where 'a : 'b{
    *node = match command{
        Command::L => &map.nodes[node].left,
        Command::R => &map.nodes[node].right
    };
}

fn count_steps(map: &Map, start: &NodeId) -> usize{
    let mut commands = map.commands.iter().cycle().enumerate();
    let mut current = start;
    loop{
        let (step, command) = commands.next().unwrap();
        execute_command(map, &mut current, command);
        if current.0.ends_with("Z"){
            return step + 1;
        }
    }
}

#[aoc_2023_markup::aoc_task(2023, 8, 2)]
fn follow_map_ghost<R: std::io::BufRead>(input: R) -> Option<usize>{
    use num::Integer;

    let map = parse_map(input)?;
    let start_nodes : Vec<&NodeId> = map.nodes.iter().map(|(name, _)| name).filter(|name| name.0.ends_with("A")).collect();
    start_nodes.iter().map(|start_node| count_steps(&map, start_node)).reduce(|a, b| a.lcm(&b))
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

    #[test]
    fn test_follow_map_ghosts(){
        const INPUT : &[u8] = indoc!{"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "}.as_bytes();

        let count = follow_map_ghost(INPUT).unwrap();
        assert_eq!(count, 6);
    }

}
