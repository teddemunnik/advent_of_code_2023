use std::io::empty;



struct Map{
    galaxies: Vec<(usize, usize)>,
}

impl Map{
    fn empty() -> Self{
        Self { galaxies: vec![] }
    }
}

fn parse_map<R: std::io::BufRead>(input: R) -> Option<Map>{
    let mut galaxies = Vec::new();
    for (y, line) in input.lines().enumerate(){
        for (x, char) in line.ok()?.chars().enumerate(){
            match char{
                '#' => {
                    galaxies.push((x, y));
                },
                '.' => (),
                _ => return None,
            }
        }
    }

    Some(Map{ galaxies })
}

fn cosmic_expansion(input: &Map, empty_scale: usize) -> Map{
    let (max_x, max_y) = match input.galaxies.iter().copied().reduce(|a, b| (a.0.max(b.0), a.1.max(b.1))){
        Some(a) => a,
        None => return Map::empty(),
    };

    // Go over the galaxies and remember which rows and columns contain anything
    let mut row_has_galaxy = vec![false; max_x + 1];
    let mut column_has_galaxy = vec![false; max_y + 1];
    for galaxy in input.galaxies.iter(){
        row_has_galaxy[galaxy.1] = true;
        column_has_galaxy[galaxy.0] = true;
    }

    // Accumulate an offset for each row and cooumn
    let row_offset : Vec<usize> = row_has_galaxy.iter().scan(0, |state, value| {
        let state_before = *state;
        if !value{ *state += empty_scale; } else { *state += 1; }
        Some(state_before)
    }).collect();
    let column_offset : Vec<usize> = column_has_galaxy.iter().scan(0, |state, value| {
        let state_before = *state;
        if !value{ *state += empty_scale; } else { *state += 1; }
        Some(state_before)
    }).collect();

    let galaxies : Vec<(usize, usize)> = input.galaxies.iter().copied().map(|galaxy| (column_offset[galaxy.0], row_offset[galaxy.1])).collect();
    Map{ galaxies }
}

fn sum_shortest_paths(map: &Map) -> usize{
    let mut sum = 0;
    for i in 0..map.galaxies.len(){
        for j in (i+1)..map.galaxies.len(){
            let a = map.galaxies[i];
            let b = map.galaxies[j];
            sum += b.0.abs_diff(a.0) + b.1.abs_diff(a.1);
        }
    }

    sum
}

#[aoc_2023_markup::aoc_task(2023, 11, 1)]
fn part1<R: std::io::BufRead>(input: R) -> Option<usize>{
    let map = cosmic_expansion(&parse_map(input)?, 2);
    Some(sum_shortest_paths(&map))
}

#[aoc_2023_markup::aoc_task(2023, 11, 2)]
fn part2<R: std::io::BufRead>(input: R) -> Option<usize>{
    let map = cosmic_expansion(&parse_map(input)?, 1000000);
    Some(sum_shortest_paths(&map))
}

#[cfg(test)]
mod tests{
    use indoc::indoc;
    use super::*;

    const INPUT : &[u8] = indoc!{"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "}.as_bytes();

    #[test]
    fn test_parse_map(){
        assert_eq!(parse_map(INPUT).unwrap().galaxies, [
            (3, 0),
            (7, 1),
            (0, 2),
            (6, 4),
            (1, 5),
            (9, 6),
            (7, 8),
            (0, 9),
            (4, 9)
        ]);

    }

    #[test]
    fn test_cosmic_expansion(){
        let map = parse_map(INPUT).unwrap();
        let map = cosmic_expansion(&map, 2);
        assert_eq!(map.galaxies, [
            (4, 0),
            (9, 1),
            (0, 2),
            (8, 5),
            (1, 6),
            (12, 7),
            (9, 10),
            (0, 11),
            (5, 11)
        ])
    }

    #[test]
    fn test_sum_shortest_path(){
        let map = cosmic_expansion(&parse_map(INPUT).unwrap(), 2);
        let result = sum_shortest_paths(&map);
        assert_eq!(result, 374);
    }

}