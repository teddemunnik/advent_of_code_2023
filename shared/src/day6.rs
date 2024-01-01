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
    time: u32,
    distance: u32,
}

fn parse_row(input: &str) -> Option<Vec<u32>>{
    input.split_ascii_whitespace().skip(1).map(|item| item.parse::<u32>().ok()).collect()
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

fn num_beating(entry: &Entry) -> u32{
    let time = entry.time as f64;
    let distance = entry.distance as f64;
    let b = ((time * time) - 4.0 * distance).sqrt();
    let a = (-time + b) / -2.0;
    let b = (-time - b) / -2.0;
    let one = (a + 1.0).floor();
    let two = b.ceil();
    (two - one).floor() as u32
}

#[aoc_2023_markup::aoc_task(2023, 6, 1)]
fn multiply_ways_to_win<R: std::io::BufRead>(input: R) -> Option<u32>{
    let table = parse_table(input)?;
    table.iter().map(|entry| num_beating(entry)).reduce(|a, b| a * b)
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
    fn test_num_beating(){
        let table = parse_table(INPUT).unwrap();
        let test : Vec<u32> = table.iter().map(|entry| num_beating(entry)).collect();
        assert_eq!(test, [ 4, 8, 9 ]);
    }

    #[test]
    fn test_multiply_ways_to_win(){
        let result = multiply_ways_to_win(INPUT).unwrap();
        assert_eq!(result, 288);
    }

}