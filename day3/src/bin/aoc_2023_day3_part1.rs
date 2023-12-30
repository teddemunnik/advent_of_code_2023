use std::{io::{BufReader, BufWriter}, thread::current, fs::File};
use aoc_2023_day3::{read_input, find_numbers, is_part, EngineSchematic};
use indoc::indoc;


fn sum_parts(schematic: &EngineSchematic) -> u32{
    find_numbers(schematic)
        .iter()
        .filter(|number| is_part(schematic, number))
        .map(|number| number.value)
        .sum()
}


fn main() {
    let result = read_input().map(|schematic| sum_parts(&schematic));
    match result{
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests{
    use crate::sum_parts;
    use aoc_2023_day3::read_schematic;
    use indoc::indoc;

    #[test]
    fn test_sum_parts(){
        const INPUT : &[u8]= indoc!{"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "}.as_bytes();


        let schematic = read_schematic(INPUT).unwrap();
        let sum = sum_parts(&schematic);
        assert_eq!(sum, 4361);
    }

}
