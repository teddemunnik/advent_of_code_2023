use std::{fs::File, io::BufReader};




pub struct EngineSchematic{
    pub rows: Vec<Vec<u8>>
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Number{
    pub value: u32,
    pub row: usize,
    pub start_column: usize,
    pub end_column: usize
}


pub fn read_schematic<R : std::io::BufRead>(reader: R) -> Result<EngineSchematic, std::io::Error>{
    let rows : Result<Vec<Vec<u8>>, std::io::Error> = reader
        .lines()
        .map(|row| row.map(|row| row.as_bytes().to_vec()))
        .collect();

    rows.map(|rows| EngineSchematic{ rows })
}

fn commit_number(schematic: &EngineSchematic, numbers: &mut Vec<Number>, pending: &mut Option<Number>, current_x: usize){
    if let Some(number) = pending.as_mut(){
        number.end_column = current_x;
        number.value = std::str::from_utf8(&schematic.rows[number.row][number.start_column..number.end_column]).unwrap().parse::<u32>().unwrap();
        numbers.push(number.clone());
        *pending = None;
    }
}

pub fn find_numbers(schematic: &EngineSchematic) -> Vec<Number>{
    let mut result = Vec::new();

    for y in 0..schematic.rows.len(){
        let mut current_number :Option<Number> =  None;

        for x in 0..schematic.rows[y].len(){
            // Start new number range
            if schematic.rows[y][x].is_ascii_digit() && current_number.is_none(){
                current_number = Some(Number { value: 0, row: y, start_column: x, end_column: x });
            }
            if !schematic.rows[y][x].is_ascii_digit(){
                commit_number(schematic, &mut result, &mut current_number, x);
            }
        }

        commit_number(schematic, &mut result, &mut current_number, schematic.rows[y].len());
    }

    result
}

pub fn is_symbol(schematic: &EngineSchematic, x: usize, y: usize) -> bool{
    let value = schematic.rows[y][x];
    !value.is_ascii_digit() && value != b'.'
}

fn test_adjacent_row(schematic: &EngineSchematic, number: &Number, y: usize) -> bool{
    let adjacent_x_start = ((number.start_column as isize) - 1).max(0) as usize;
    let adjacent_x_end = (number.end_column + 1).min(schematic.rows[y].len());
    for x in adjacent_x_start..adjacent_x_end{
        if is_symbol(schematic, x, y){
            return true;
        }
    }

    false
}

fn is_part(schematic: &EngineSchematic, number: &Number) -> bool{

    let mut any_symbol = false;

    // Test top of number
    if number.row > 0{
        any_symbol |= test_adjacent_row(schematic, number, number.row - 1);
    }

    // test left of number
    if number.start_column > 0{
        any_symbol |= is_symbol(schematic, number.start_column - 1, number.row);
    }

    // test right of number
    if number.end_column < schematic.rows[number.row].len(){
        any_symbol |= is_symbol(schematic, number.end_column, number.row);
    }

    // Test row under number
    if number.row < schematic.rows.len() - 1{
        any_symbol |= test_adjacent_row(schematic, number, number.row + 1);
    }

    any_symbol

}

pub fn find_part_numbers(schematic: &EngineSchematic) -> Vec<Number>{
    find_numbers(schematic).into_iter().filter(|number| is_part(schematic, number)).collect()
}

pub fn read_input() -> Result<EngineSchematic, std::io::Error>{
    File::open("input_day3.txt")
        .map(|file| BufReader::new(file))
        .and_then(|reader| read_schematic(reader))
}

#[cfg(test)]
mod tests{
    use crate::{read_schematic, find_numbers, Number};
    use indoc::indoc;

    #[test]
    fn test_find_numbers(){
        const INPUT : &[u8]= indoc!{"
            467..114..
            ...*......
            ..35..633.
        "}.as_bytes();

        let schematic = read_schematic(INPUT).unwrap();
        let numbers = find_numbers(&schematic);
        assert_eq!(numbers, [
            Number{ row: 0, start_column: 0, end_column: 3, value: 467 },
            Number{ row: 0, start_column: 5, end_column: 8, value: 114 },
            Number{ row: 2, start_column: 2, end_column: 4, value: 35 },
            Number{ row: 2, start_column: 6, end_column: 9, value: 633 },
        ]);
    }
}