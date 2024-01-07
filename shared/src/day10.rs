use std::collections::{HashSet, BinaryHeap};

use itertools::Itertools;


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

/// Get a pipe that connects between two unqiue directions in a given tile
fn get_matching_pipe(a: Direction, b: Direction) -> PipeType{
    assert_ne!(a, b);

    use Direction::*;
    match (a, b){
        (Up, Right) => PipeType::NE,
        (Up, Down) => PipeType::NS,
        (Up, Left) => PipeType::NW,
        (Right, Up) => PipeType::NE,
        (Right, Down) => PipeType::SE,
        (Right, Left) => PipeType::EW,
        (Down, Up) => PipeType::NS,
        (Down, Right) => PipeType::SE,
        (Down, Left) => PipeType::SW,
        (Left, Up) => PipeType::NS,
        (Left, Right) => PipeType::SE,
        (Left, Down) => PipeType::SW,
        _ => unreachable!(),
    }
}

struct ConnectionIterator<'a>{
    map: &'a Map,
    location: (usize, usize),
    direction: Option<Direction>
}

impl<'a> Iterator for ConnectionIterator<'a>{
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.map.data[self.location.1][self.location.0];

        while let Some(direction) = self.direction{
            match direction{
                Direction::Up => {
                    self.direction = Some(Direction::Right);
                    if self.location.1 > 0 && connects(self.map, a, self.map.data[self.location.1 - 1][self.location.0], Direction::Up){
                        return Some(Direction::Up);
                    }
                }
                Direction::Right => {
                    self.direction = Some(Direction::Down);
                    if self.location.0 < self.map.data[self.location.1].len() - 1 && connects(self.map, a, self.map.data[self.location.1][self.location.0 + 1], Direction::Right){
                        return Some(Direction::Right);
                    }
                }
                Direction::Down => {
                    self.direction = Some(Direction::Left);
                    if self.location.1 < self.map.data.len() - 1 && connects(self.map, a, self.map.data[self.location.1 + 1][self.location.0], Direction::Down){
                        return Some(Direction::Down);
                    }
                }
                Direction::Left => {
                    self.direction = None;
                    if self.location.0 > 0 && connects(self.map, a, self.map.data[self.location.1][self.location.0 -1 ], Direction::Left){
                        return Some(Direction::Left);
                    }
                }
            }
        }

        None
    }
}

fn find_connecting_pipes(map: &Map, location: (usize, usize)) -> impl Iterator<Item = Direction> + '_{
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

fn get_location(origin: (usize, usize), direction: Direction) -> (usize, usize){
    match direction{
        Direction::Up => (origin.0, origin.1 - 1),
        Direction::Right => (origin.0 + 1, origin.1),
        Direction::Down => (origin.0, origin.1 + 1),
        Direction::Left => (origin.0 -1, origin.1),
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
        for direction in find_connecting_pipes(map, item.location){
            let connection = get_location(item.location, direction);
            queue.push(QueuedPath{ location: connection, cost: item.cost + 1});
        }
    }

    furthest_visited
}

fn get_cycle_map(map: &mut Map){
    // Figure out which pipes are part of the loop containing the start
    let mut visited = HashSet::new();
    let start = find_start(map).unwrap();
    let mut queue = Vec::new();
    queue.push(start);
    while let Some(item) = queue.pop(){
        if !visited.insert(item){
            continue;
        }

        for direction in find_connecting_pipes(map, item){
            queue.push(get_location(item, direction));
        }
    }

    // Replace unvisited pipes with ground
    for (y, row) in map.data.iter_mut().enumerate(){
        for (x, value) in row.iter_mut().enumerate(){
            if *value != PipeType::Start && *value != PipeType::None{
                if !visited.contains(&(x, y)){
                    *value = PipeType::None;
                }
            }
        }
    }

    // Replace start with a matching pipe type
    let (a, b) = find_connecting_pipes(map, start).collect_tuple().unwrap();
    map.data[start.1][start.0] = get_matching_pipe(a, b);
}

fn count_inside_loop(map: &mut Map) -> usize{
    // Get a map with just the loop
    get_cycle_map(map);

    let mut count = 0;
    for y in 0..map.data.len(){
        let mut entry = None;
        let mut hits = 0;
        let line = &map.data[y];

        for x in 0..line.len(){
            let pipe = line[x];
            match pipe{
                // | pipes are always hits 
                PipeType::NS => hits += 1, 

                // The first L turn of a pair of turns we encounter is not yet a hit, the direction of the outgoing L turn will decide if it's a hit
                PipeType::NE | PipeType::SE => entry = Some(pipe),

                // We count L turns as hits when the previous L turn we hit on this line was in the opposite direction
                PipeType::NW if entry.unwrap() == PipeType::SE => hits += 1,
                PipeType::SW if entry.unwrap() == PipeType::NE => hits += 1,

                PipeType::None if hits % 2 != 0 => count += 1,
                _ => (),
            }
        }
    }

    count
}

#[aoc_2023_markup::aoc_task(2023, 10, 1)]
fn part1<R: std::io::BufRead>(input: R) -> Option<usize>{
    let map = parse_map(input)?;
    Some(find_furthest_pipe_from_start(&map))
}

#[aoc_2023_markup::aoc_task(2023, 10, 2)]
fn part2<R: std::io::BufRead>(input: R) -> Option<usize>{
    let mut map = parse_map(input)?;
    Some(count_inside_loop(&mut map))
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
            Direction::Right,
            Direction::Down,
        ]);

        itertools::assert_equal(find_connecting_pipes(&map, (2, 3)), [
            Direction::Right,
            Direction::Left,
        ]);
    }

    #[test]
    fn test_furthest_visited(){
        let map = parse_map(INPUT).unwrap();
        assert_eq!(find_furthest_pipe_from_start(&map), 4);

        let map_complex = parse_map(INPUT_COMPLEX).unwrap();
        assert_eq!(find_furthest_pipe_from_start(&map_complex), 8);
    }

    #[test]
    fn test_count_in_loop(){
        const INPUT : &[u8] = indoc!{"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "}.as_bytes();

        let mut map = parse_map(INPUT).unwrap();
        assert_eq!(count_inside_loop(&mut map), 4);

    }

    #[test]
    fn test_count_in_loop_between_pipes(){
        const INPUT : &[u8] = indoc!{"
            ..........
            .S------7.
            .|F----7|.
            .||....||.
            .||....||.
            .|L-7F-J|.
            .|..||..|.
            .L--JL--J.
            ..........
        "}.as_bytes();

        let mut map = parse_map(INPUT).unwrap();
        assert_eq!(count_inside_loop(&mut map), 4);
    }

    #[test]
    fn test_count_in_loop_complex(){
        const INPUT : &[u8] = indoc!{"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "}.as_bytes();

        let mut map = parse_map(INPUT).unwrap();
        assert_eq!(count_inside_loop(&mut map), 10);
    }


}