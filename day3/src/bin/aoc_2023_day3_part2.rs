use aoc_2023_day3::{EngineSchematic, find_part_numbers, Number, read_input};

fn is_adjacent(number: &Number, x: usize, y: usize) -> bool{
    if number.row > 0 && y < number.row - 1{
        return false;
    }

    if y > number.row + 1{
        return false;
    }

    if number.start_column > 0 && x < number.start_column - 1{
        return false;
    }

    if x > number.end_column{
        return false;
    }

    if y == number.row && x >= number.start_column && x < number.end_column{
        return false;
    }

    true
}

fn find_sum_gear_ratios(schematic: &EngineSchematic) -> u32{

    let part_numbers = find_part_numbers(schematic);
    let mut sum = 0;

    for y in 0..schematic.rows.len(){
        for x in 0..schematic.rows[y].len(){
            let value = schematic.rows[y][x];
            if value == b'*'{
                let adjacent: Vec<Number> = part_numbers.iter().filter(|number| is_adjacent(number, x, y)).copied().collect();
                if adjacent.len() == 2{
                    sum += adjacent[0].value * adjacent[1].value;
                }
            }
        }
    }

    sum
}


fn main(){
    aoc_2023_shared::run(read_input().map(|schematic| find_sum_gear_ratios(&schematic)));
}

#[cfg(test)]
mod tests{
    use aoc_2023_day3::read_schematic;
    use indoc::indoc;

    use crate::find_sum_gear_ratios;


    #[test]
    fn test_find_sum_gear_ratios(){
        const INPUT : &[u8] = indoc! {"
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
        let sum = find_sum_gear_ratios(&schematic);
        assert_eq!(sum, 467835);
    }

}