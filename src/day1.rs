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

fn parse_line(value: &String) -> Result<u32, ParseLineError>{
    let first_byte = value
        .find(|c:char| c.is_ascii_digit())
        .ok_or(ParseLineError::NotEnoughDigits)?;
    let last_byte = value
        .rfind(|c:char| c.is_ascii_digit())
        .ok_or(ParseLineError::NotEnoughDigits)?;

    let first_value = value[first_byte..].chars().next().unwrap().to_digit(10).unwrap();
    let last_value= value[last_byte..].chars().next().unwrap().to_digit(10).unwrap();
    Ok(first_value * 10 + last_value)
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
        let sample_input = String::from("pqr3stu8vwx");
        assert_eq!(parse_line(&sample_input).unwrap(), 38);
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
}
