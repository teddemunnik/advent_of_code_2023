use std::collections::{BinaryHeap, HashSet};
use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u8)]
enum PipeType {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Start,
    None,
}

impl PipeType{
    /// Checks whether the given pipe type can potentially support a connection to a pipe in the given direction
    fn can_connect_to_direction(self, direction: Direction) -> bool {
        match self{
            PipeType::NS => direction == Direction::Up || direction == Direction::Down,
            PipeType::EW => direction == Direction::Left || direction == Direction::Right,
            PipeType::NE => direction == Direction::Up || direction == Direction::Right,
            PipeType::NW => direction == Direction::Up || direction == Direction::Left,
            PipeType::SW => direction == Direction::Down || direction == Direction::Left,
            PipeType::SE => direction == Direction::Down || direction == Direction::Right,
            PipeType::Start => true,
            PipeType::None => false,
        }
    }

    /// Get a pipe that connects between two unqiue directions in a given tile
    fn from_directions(a: Direction, b: Direction) -> PipeType {
        assert_ne!(a, b);

        use Direction::*;
        match (a, b) {
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
}

impl TryFrom<char> for PipeType{
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value{
            'S' => Ok(PipeType::Start),
            '.' => Ok(PipeType::None),
            '|' => Ok(PipeType::NS),
            '-' => Ok(PipeType::EW),
            'J' => Ok(PipeType::NW),
            '7' => Ok(PipeType::SW),
            'F' => Ok(PipeType::SE),
            'L' => Ok(PipeType::NE),
            _ => Err(format!("Unexpected char value {}", value))
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    /// Moves the cursor from passed into location a single tile in this direction
    fn move_location(self, origin: (usize, usize)) -> (usize, usize) {
        match self{
            Direction::Up => (origin.0, origin.1 - 1),
            Direction::Right => (origin.0 + 1, origin.1),
            Direction::Down => (origin.0, origin.1 + 1),
            Direction::Left => (origin.0 - 1, origin.1),
        }
    }
}

fn connects(a: PipeType, b: PipeType, direction: Direction) -> bool {
    a.can_connect_to_direction(direction) && b.can_connect_to_direction(direction.opposite())
}

struct ConnectionIterator<'a> {
    map: &'a Map,
    location: (usize, usize),
    direction: Option<Direction>,
}

impl<'a> Iterator for ConnectionIterator<'a> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.map.data[self.location.1][self.location.0];

        while let Some(direction) = self.direction {
            match direction {
                Direction::Up => {
                    self.direction = Some(Direction::Right);
                    if self.location.1 > 0
                        && connects(
                            a,
                            self.map.data[self.location.1 - 1][self.location.0],
                            Direction::Up,
                        )
                    {
                        return Some(Direction::Up);
                    }
                }
                Direction::Right => {
                    self.direction = Some(Direction::Down);
                    if self.location.0 < self.map.data[self.location.1].len() - 1
                        && connects(
                            a,
                            self.map.data[self.location.1][self.location.0 + 1],
                            Direction::Right,
                        )
                    {
                        return Some(Direction::Right);
                    }
                }
                Direction::Down => {
                    self.direction = Some(Direction::Left);
                    if self.location.1 < self.map.data.len() - 1
                        && connects(
                            a,
                            self.map.data[self.location.1 + 1][self.location.0],
                            Direction::Down,
                        )
                    {
                        return Some(Direction::Down);
                    }
                }
                Direction::Left => {
                    self.direction = None;
                    if self.location.0 > 0
                        && connects(
                            a,
                            self.map.data[self.location.1][self.location.0 - 1],
                            Direction::Left,
                        )
                    {
                        return Some(Direction::Left);
                    }
                }
            }
        }

        None
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Map {
    data: Vec<Vec<PipeType>>,
}

impl Map{
    /// Iterate between the connecting directions for a pipe in a given location
    fn find_connecting_directions(
        &self,
        location: (usize, usize),
    ) -> impl Iterator<Item = Direction> + '_ {
        ConnectionIterator {
            map: self,
            location,
            direction: Some(Direction::Up),
        }
    }

    fn find_start(&self) -> Option<(usize, usize)> {
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                if self.data[y][x] == PipeType::Start {
                    return Some((x, y));
                }
            }
        }

        None
    }

    fn get_pipes_in_loop(&self) -> HashSet<(usize, usize)>{
        let mut visited = HashSet::new();
        let start = self.find_start().unwrap();
        let mut queue = Vec::new();
        queue.push(start);
        while let Some(item) = queue.pop() {
            if !visited.insert(item) {
                continue;
            }

            for direction in self.find_connecting_directions(item){
                queue.push(direction.move_location(item))
            }
        }
        visited
    }

    /// Gets a map with only pipes that are part of the loop.
    /// Pipes not in the loop are replaced with ground
    /// The start pipe is replaced with a it's infered pipe type
    fn get_loop_map(&self) -> Map{
        // Figure out which pipes are part of the loop containing the start
        let pipes_in_loop = self.get_pipes_in_loop();

        // Replace pipes that are not in the loops with ground
        let mut map = self.clone();
        for (y, row) in map.data.iter_mut().enumerate() {
            for (x, value) in row.iter_mut().enumerate() {
                if *value != PipeType::Start && *value != PipeType::None {
                    if !pipes_in_loop.contains(&(x, y)) {
                        *value = PipeType::None;
                    }
                }
            }
        }

        // Replace start with a matching pipe type
        let start = self.find_start().unwrap();
        let (a, b) = map.find_connecting_directions(start).collect_tuple().unwrap();
        map.data[start.1][start.0] = PipeType::from_directions(a, b);

        map
    }
}

fn parse_line(line: &str) -> Option<Vec<PipeType>> {
    line.chars().map(|item| item.try_into().ok()).collect()
}

fn parse_map<R: std::io::BufRead>(input: R) -> Option<Map> {
    let lines: Vec<Vec<PipeType>> = input
        .lines()
        .map(|line| line.ok().and_then(|line| parse_line(&line)))
        .collect::<Option<_>>()?;
    Some(Map { data: lines })
}

#[derive(Eq)]
struct QueuedPath {
    location: (usize, usize),
    cost: usize,
}

impl Ord for QueuedPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for QueuedPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for QueuedPath {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

fn find_furthest_pipe_from_start(map: &Map) -> usize {
    let start = map.find_start().unwrap();
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    let mut furthest_visited = 0;
    queue.push(QueuedPath {
        location: start,
        cost: 0,
    });
    while let Some(item) = queue.pop() {
        // Early out when revisiting, as we should have a higher cost
        if !visited.insert(item.location) {
            continue;
        }

        furthest_visited = furthest_visited.max(item.cost);

        // Visit connections
        for direction in map.find_connecting_directions(item.location) {
            let connection = direction.move_location(item.location);
            queue.push(QueuedPath {
                location: connection,
                cost: item.cost + 1,
            });
        }
    }

    furthest_visited
}

fn count_inside_loop(map: &Map) -> usize {
    let map = map.get_loop_map();

    let mut count = 0;
    for y in 0..map.data.len() {
        let mut entry = None;
        let mut hits = 0;
        let line = &map.data[y];

        for x in 0..line.len() {
            let pipe = line[x];
            match pipe {
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
fn part1<R: std::io::BufRead>(input: R) -> Option<usize> {
    let map = parse_map(input)?;
    Some(find_furthest_pipe_from_start(&map))
}

#[aoc_2023_markup::aoc_task(2023, 10, 2)]
fn part2<R: std::io::BufRead>(input: R) -> Option<usize> {
    let mut map = parse_map(input)?;
    Some(count_inside_loop(&mut map))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &[u8] = indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "}
    .as_bytes();

    const INPUT_COMPLEX: &[u8] = indoc! {"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    "}
    .as_bytes();

    #[test]
    fn test_parse_map() {
        let map = parse_map(INPUT).unwrap();
        assert_eq!(
            map,
            Map {
                data: vec![
                    vec![PipeType::None; 5],
                    vec![
                        PipeType::None,
                        PipeType::Start,
                        PipeType::EW,
                        PipeType::SW,
                        PipeType::None
                    ],
                    vec![
                        PipeType::None,
                        PipeType::NS,
                        PipeType::None,
                        PipeType::NS,
                        PipeType::None
                    ],
                    vec![
                        PipeType::None,
                        PipeType::NE,
                        PipeType::EW,
                        PipeType::NW,
                        PipeType::None
                    ],
                    vec![PipeType::None; 5]
                ]
            }
        );
    }

    #[test]
    fn test_find_connecting_directions() {
        let map = parse_map(INPUT).unwrap();

        itertools::assert_equal(
            map.find_connecting_directions((1, 1)),
            [Direction::Right, Direction::Down],
        );

        itertools::assert_equal(
            map.find_connecting_directions((2, 3)),
            [Direction::Right, Direction::Left],
        );
    }

    #[test]
    fn test_furthest_visited() {
        let map = parse_map(INPUT).unwrap();
        assert_eq!(find_furthest_pipe_from_start(&map), 4);

        let map_complex = parse_map(INPUT_COMPLEX).unwrap();
        assert_eq!(find_furthest_pipe_from_start(&map_complex), 8);
    }

    #[test]
    fn test_count_in_loop() {
        const INPUT: &[u8] = indoc! {"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "}
        .as_bytes();

        let mut map = parse_map(INPUT).unwrap();
        assert_eq!(count_inside_loop(&mut map), 4);
    }

    #[test]
    fn test_count_in_loop_between_pipes() {
        const INPUT: &[u8] = indoc! {"
            ..........
            .S------7.
            .|F----7|.
            .||....||.
            .||....||.
            .|L-7F-J|.
            .|..||..|.
            .L--JL--J.
            ..........
        "}
        .as_bytes();

        let mut map = parse_map(INPUT).unwrap();
        assert_eq!(count_inside_loop(&mut map), 4);
    }

    #[test]
    fn test_count_in_loop_complex() {
        const INPUT: &[u8] = indoc! {"
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
        "}
        .as_bytes();

        let mut map = parse_map(INPUT).unwrap();
        assert_eq!(count_inside_loop(&mut map), 10);
    }
}
