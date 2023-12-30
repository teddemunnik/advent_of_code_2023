use std::fs::File;
use thiserror::Error;
use std::io::{BufRead, BufReader};

#[derive(Error, Debug)]
enum ParseLineError{
    #[error("No digits were found")]
    NotEnoughDigits
}

#[derive(Error, Debug)]
enum Error{
    #[error("failed to open input file ({inner:?}")]
    FailedToOpenInputFile{inner: std::io::Error},
    #[error("failed to parse line {line}: {inner}")]
    FailedToParseLine{line: usize, inner: ParseLineError}
}

const NUMBERS : [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn parse_line(value: &str) -> Result<u32, ParseLineError>{
    // Find all symbols in the line
    let symbols = value.char_indices().filter_map(|(index, char)|{
        if char.is_ascii_digit(){
            return Some(char.to_digit(10).unwrap());
        }

        let remaining_value= &value[index..];
        for (index, word) in NUMBERS.iter().enumerate(){
            if remaining_value.starts_with(word){
                return Some(index as u32 + 1);
            }
        }

        None
    });

    let first = symbols.clone().next().ok_or(ParseLineError::NotEnoughDigits)?;
    let last = symbols.clone().last().ok_or(ParseLineError::NotEnoughDigits)?;
    Ok(first * 10 + last)
}

fn parse_calibration_document<R: BufRead>(input: R)-> Result<u32, Error>{
    let lines = input.lines().flatten();

    Ok(lines
        .enumerate()
        .map(|(line, value)| parse_line(&value).map_err(|e| Error::FailedToParseLine {line, inner: e}))
        .try_fold(0, |acc, value| Ok(acc + value?))?)
}

fn main() {
    let result = match File::open("input.txt") {
        Ok(file) => Ok(BufReader::new(file)),
        Err(e) => Err(Error::FailedToOpenInputFile { inner: e }),
    }.and_then(parse_calibration_document);

    match result{
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error occurred: {}", e),
    }
}

#[cfg(test)]
mod tests{
    use crate::{parse_calibration_document, parse_line};

    #[test]
    fn test_single_line(){
        const SAMPLE_INPUT : &str= "pqr3stu8vwx";
        assert_eq!(parse_line(SAMPLE_INPUT).unwrap(), 38);
    }

    #[test]
    fn test_single_line_spelled(){
        assert_eq!(parse_line("two1nine").unwrap(), 29);
    }

    #[test]
    fn test_result(){
        const SAMPLE_INPUT : &[u8]= "1abc2
                                     pqr3stu8vwx
                                     a1b2c3d4e5f
                                     treb7uchet".as_bytes();

        let output = parse_calibration_document(SAMPLE_INPUT).unwrap();
        assert_eq!(output, 142);
    }

    #[test]
    fn test_list_with_spelled(){
        const SAMPLE_INPUT : &[u8] = "two1nine
                                      eightwothree
                                      abcone2threexyz
                                      xtwone3four
                                      4nineeightseven2
                                      zoneight234
                                      7pqrstsixteen".as_bytes();
        let output = parse_calibration_document(SAMPLE_INPUT).unwrap();
        assert_eq!(output, 281);
    }
}
