use std::{collections::{HashSet, BinaryHeap}, cmp::Reverse};


#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u8)]
enum PipeType{
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Start,
    None,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Direction{
    Up,
    Down,
    Left,
    Right
}

impl Direction{
    fn opposite(self) -> Self{
        match self{
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

fn has_connection(pipe: PipeType, direction: Direction) -> bool{
    match pipe{
        PipeType::NS => direction == Direction::Up || direction == Direction::Down,
        PipeType::EW => direction == Direction::Left || direction == Direction::Right,
        PipeType::NE => direction == Direction::Up || direction == Direction::Right,
        PipeType::NW => direction == Direction::Up || direction == Direction::Left,
        PipeType::SW => direction == Direction::Down|| direction == Direction::Left,
        PipeType::SE => direction == Direction::Down || direction == Direction::Right,
        PipeType::Start => true,
        PipeType::None => false,
    }
}


fn connects(map: &Map, a: PipeType, b: PipeType, direction: Direction) -> bool{
    has_connection(a, direction) && has_connection(b, direction.opposite())
}

struct ConnectionIterator<'a>{
    map: &'a Map,
    location: (usize, usize),
    direction: Option<Direction>
}

impl<'a> Iterator for ConnectionIterator<'a>{
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.map.data[self.location.1][self.location.0];

        while let Some(direction) = self.direction{
            match direction{
                Direction::Up => {
                    self.direction = Some(Direction::Right);
                    if self.location.1 > 0 && connects(self.map, a, self.map.data[self.location.1 - 1][self.location.0], Direction::Up){
                        return Some((self.location.0, self.location.1 - 1));
                    }
                }
                Direction::Right => {
                    self.direction = Some(Direction::Down);
                    if self.location.0 < self.map.data[self.location.1].len() - 1 && connects(self.map, a, self.map.data[self.location.1][self.location.0 + 1], Direction::Right){
                        return Some((self.location.0 + 1, self.location.1));
                    }
                }
                Direction::Down => {
                    self.direction = Some(Direction::Left);
                    if self.location.1 < self.map.data.len() - 1 && connects(self.map, a, self.map.data[self.location.1 + 1][self.location.0], Direction::Down){
                        return Some((self.location.0, self.location.1 + 1));
                    }
                }
                Direction::Left => {
                    self.direction = None;
                    if self.location.0 > 0 && connects(self.map, a, self.map.data[self.location.1][self.location.0 -1 ], Direction::Left){
                        return Some((self.location.0 - 1, self.location.1));
                    }
                }
            }
        }

        None
    }
}

fn find_connecting_pipes(map: &Map, location: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_{
    ConnectionIterator{
        map,
        location,
        direction: Some(Direction::Up),
    }
}

fn parse_pipe(item: char) -> Option<PipeType>{
    match item{
        'S' => Some(PipeType::Start),
        '.' => Some(PipeType::None),
        '|' => Some(PipeType::NS),
        '-' => Some(PipeType::EW),
        'J' => Some(PipeType::NW),
        '7' => Some(PipeType::SW),
        'F' => Some(PipeType::SE),
        'L' => Some(PipeType::NE),
        _ => None
    }
}

fn parse_line(line: &str) -> Option<Vec<PipeType>>{
    line.chars().map(parse_pipe).collect()
}

fn parse_map<R: std::io::BufRead>(input: R) -> Option<Map>{
    let lines : Vec<Vec<PipeType>> = input.lines().map(|line| line.ok().and_then(|line| parse_line(&line))).collect::<Option<_>>()?;
    Some(Map { data: lines })
}

fn find_start(map: &Map) -> Option<(usize, usize)>{
    for y in 0..map.data.len(){
        for x in 0..map.data[y].len(){
            if map.data[y][x] == PipeType::Start{
                return Some((x, y));
            }
        }
    }

    None
}

#[derive(Eq)]
struct QueuedPath{
    location: (usize, usize),
    cost: usize,
}

impl Ord for QueuedPath{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for QueuedPath{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for QueuedPath{
    fn eq(&self, other: &Self) -> bool{
        self.cost == other.cost
    }
}

fn find_furthest_pipe_from_start(map: &Map) -> usize{
    let start = find_start(map).unwrap();
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    let mut furthest_visited= 0;
    queue.push(QueuedPath{ location: start, cost: 0});
    while let Some(item) = queue.pop(){
        // Early out when revisiting, as we should have a higher cost
        if !visited.insert(item.location){
            continue;
        }

        furthest_visited = furthest_visited.max(item.cost);

        // Visit connections
        for connection in find_connecting_pipes(map, item.location){
            queue.push(QueuedPath{ location: connection, cost: item.cost + 1});
        }
    }

    furthest_visited
}

#[aoc_2023_markup::aoc_task(2023, 10, 1)]
fn part1<R: std::io::BufRead>(input: R) -> Option<usize>{
    let map = parse_map(input)?;
    Some(find_furthest_pipe_from_start(&map))
}

#[derive(PartialEq, Eq, Debug)]
struct Map{
    data: Vec<Vec<PipeType>>,
}

#[cfg(test)]
mod tests{
    use super::*;
    use indoc::indoc;

    const INPUT :&[u8] = indoc!{"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "}.as_bytes();

    const INPUT_COMPLEX: &[u8] = indoc!{"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    "}.as_bytes();

    #[test]
    fn test_parse_map(){
        let map = parse_map(INPUT).unwrap();
        assert_eq!(map, Map{
            data: vec![
                vec![ PipeType::None; 5],
                vec![ PipeType::None, PipeType::Start, PipeType::EW, PipeType::SW, PipeType::None ],
                vec![ PipeType::None, PipeType::NS, PipeType::None, PipeType::NS, PipeType::None ],
                vec![ PipeType::None, PipeType::NE, PipeType::EW, PipeType::NW,PipeType::None ],
                vec![ PipeType::None; 5]
            ]
        });
    }

    #[test]
    fn test_find_connecting_pipes(){
        let map = parse_map(INPUT).unwrap();

        itertools::assert_equal(find_connecting_pipes(&map, (1, 1)), [
            (2, 1),
            (1, 2)
        ]);

        itertools::assert_equal(find_connecting_pipes(&map, (2, 3)), [
            (3, 3),
            (1, 3)
        ]);
    }

    #[test]
    fn test_furthest_visited(){
        let map = parse_map(INPUT).unwrap();
        assert_eq!(find_furthest_pipe_from_start(&map), 4);

        let map_complex = parse_map(INPUT_COMPLEX).unwrap();
        assert_eq!(find_furthest_pipe_from_start(&map_complex), 8);
    }

}