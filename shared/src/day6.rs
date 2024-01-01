// v = tHeld
// tRemain = tTotal - tHeld
// d = v * tRemain

// d = tHeld * (tTotal - tHeld) - score = 0;
// d = tHeld * tTotal - tHeld^2
// abc
// d = b^2 - 4ac
// a = -1
// b = tTotal
// c = -score

// -tTotal +- sqrt(tTotal^2 - 4score) / 2

// (-b - sqrt(b^2 - 4ac)) / 2a

// tHeld * (tTotal - tHeld) - score = 0
//-tHeld^2 + tHeld * tTotal - score = 0

//a = -1
//b = tTotal
//c = -score



#[derive(Debug, PartialEq, Eq)]
struct Entry{
    time: usize,
    distance: usize,
}

fn parse_row(input: &str) -> Option<Vec<usize>>{
    input.split_ascii_whitespace().skip(1).map(|item| item.parse::<usize>().ok()).collect()
}

fn parse_table<R: std::io::BufRead>(input: R) -> Option<Vec<Entry>>{
    let mut lines = input.lines();
    let time_line = lines.next()?.ok()?;
    let distance_line = lines.next()?.ok()?;

    let times = parse_row(&time_line)?;
    let distances = parse_row(&distance_line)?;

    if times.len() != distances.len(){
        return None;
    }

    let mut result = Vec::with_capacity(times.len());
    for i in 0..times.len(){
        result.push(Entry{
            time: times[i],
            distance: distances[i]
        });
    }

    Some(result)
}

fn parse_row_no_kerning(input: &str) -> Option<usize>{
    let number : String = input.split(':').nth(1)?.chars().filter(|char| !char.is_whitespace()).collect();
    number.parse::<usize>().ok()
}

fn parse_table_no_kerning<R: std::io::BufRead>(input: R) -> Option<Entry>{
    let mut lines = input.lines();
    let time_line = lines.next()?.ok()?;
    let distance_line = lines.next()?.ok()?;
    let time = parse_row_no_kerning(&time_line)?;
    let distance = parse_row_no_kerning(&distance_line)?;
    Some(Entry { time, distance})
}

fn num_beating(entry: &Entry) -> usize{
    let time = entry.time as f64;
    let distance = entry.distance as f64;
    let b = ((time * time) - 4.0 * distance).sqrt();
    let a = (-time + b) / -2.0;
    let b = (-time - b) / -2.0;
    let one = (a + 1.0).floor();
    let two = b.ceil();
    (two - one).floor() as usize
}

#[aoc_2023_markup::aoc_task(2023, 6, 1)]
fn multiply_ways_to_win<R: std::io::BufRead>(input: R) -> Option<usize>{
    let table = parse_table(input)?;
    table.iter().map(|entry| num_beating(entry)).reduce(|a, b| a * b)
}

#[aoc_2023_markup::aoc_task(2023, 6, 2)]
fn ways_to_win_no_kerning<R: std::io::BufRead>(input: R) -> Option<usize>{
    let table = parse_table_no_kerning(input)?;
    Some(num_beating(&table))
}

#[cfg(test)]
mod tests{
    use super::*;
    use indoc::indoc;

    const INPUT : &[u8] = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "}.as_bytes();

    #[test]
    fn test_parse_table(){

        let table = parse_table(INPUT).unwrap();
        assert_eq!(table, [
            Entry{ time: 7, distance: 9},
            Entry{ time: 15, distance: 40},
            Entry{ time: 30, distance: 200 },
        ]);
    }

    #[test]
    fn test_parse_table_no_kerning(){
        let table = parse_table_no_kerning(INPUT).unwrap();
        assert_eq!(table, Entry{
            time: 71530,
            distance: 940200
        });
    }

    #[test]
    fn test_num_beating(){
        let table = parse_table(INPUT).unwrap();
        let test : Vec<usize> = table.iter().map(|entry| num_beating(entry)).collect();
        assert_eq!(test, [ 4, 8, 9 ]);
    }

    #[test]
    fn test_multiply_ways_to_win(){
        let result = multiply_ways_to_win(INPUT).unwrap();
        assert_eq!(result, 288);
    }

    #[test]
    fn test_ways_to_win_no_kerning(){
        let ways_to_win = ways_to_win_no_kerning(INPUT).unwrap();
        assert_eq!(ways_to_win, 71503);
    }

}