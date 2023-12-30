use aoc_2023_day3::{read_input, EngineSchematic, find_part_numbers};

fn sum_parts(schematic: &EngineSchematic) -> u32{
    find_part_numbers(schematic)
        .iter()
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
