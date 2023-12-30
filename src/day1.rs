use thiserror::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Error, Debug)]
enum Error{
    #[error("failed to open input file ({inner:?}")]
    FailedToOpenInputFile{inner: std::io::Error},
    #[error("not enough digits on line {line}")]
    NotEnoughDigits{line: usize}
}

fn read_calibration_document()-> Result<u32, Error>{
    let lines = BufReader::new(File::open("input.txt").map_err(|e| Error::FailedToOpenInputFile { inner:e })?).lines().flatten();

    Ok(lines
        .enumerate()
        .map(|(line, value)|{
            let first_byte = value
                .find(|c:char| c.is_ascii_digit())
                .ok_or(Error::NotEnoughDigits {line})?;
            let last_byte = value
                .rfind(|c:char| c.is_ascii_digit())
                .ok_or(Error::NotEnoughDigits {line})?;

            let first_value = value[first_byte..].chars().next().unwrap().to_digit(10).unwrap();
            let last_value= value[last_byte..].chars().next().unwrap().to_digit(10).unwrap();
            Ok(first_value * 10 + last_value)
        })
        .try_fold(0, |acc, value| Ok(acc + value?))?)
}

fn main() {
    let result = read_calibration_document();
    match result{
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error occurred: {}", e),
    }
}
