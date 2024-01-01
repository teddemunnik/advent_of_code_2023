
use std::{io::{BufRead, Cursor, BufReader}, fs::File, fmt::Display};
pub use linkme;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub trait AocTask{
    fn year(&self) -> u32;
    fn day(&self) -> u32;
    fn part(&self) -> u32;
    fn invoke(&self, reader: &mut dyn BufRead);
}

#[linkme::distributed_slice]
pub static AOC_ENTRIES: [&(dyn AocTask + Sync)];


pub trait AocResult{
    fn write(&self, write: &mut dyn std::io::Write) -> std::io::Result<()>;
}

impl<T: std::fmt::Display, E: std::fmt::Display> AocResult for Result<T, E>{
    fn write(&self, write: &mut dyn std::io::Write) -> std::io::Result<()>{
        match self{
            Ok(value) => writeln!(write, "Result: {}", value),
            Err(error) => writeln!(write, "Error: {}", error),
        }
    }
}

impl<T: std::fmt::Display> AocResult for Option<T>{
    fn write(&self, write: &mut dyn std::io::Write) -> std::io::Result<()>{
        match self{
            Some(value) => writeln!(write, "Result: {}", value),
            None => writeln!(write, "Error"),
        }
    }
}

macro_rules! aoc_result_display {
    ($name:ident) => {
        impl AocResult for $name{
            fn write(&self, write: &mut dyn std::io::Write) -> std::io::Result<()>{
                writeln!(write, "Result: {}", self)
            }
        }
    };
}

aoc_result_display!(u32);
aoc_result_display!(usize);


pub fn run<R: AocResult>(input: R){
    input.write(&mut std::io::stdout());
}

fn main(){
    // Order registered tasks by year, then day, then part
    let mut entries = AOC_ENTRIES.to_vec();
    entries.sort_by(|a, b| {
        if a.year() == b.year(){
            if a.day() == b.day(){
                a.part().cmp(&b.part())
            }
            else{
                a.day().cmp(&b.day())
            }
        }
        else{
            a.year().cmp(&b.year())
        }
    });

    for entry in entries{
        println!("{} day {} part {}", entry.year(), entry.day(), entry.part());

        let path = format!("inputs/{}/{}.txt", entry.year(), entry.day());
        let mut reader =  File::open(path).map(|file| BufReader::new(file)).unwrap();

        let start = std::time::Instant::now();
        entry.invoke(&mut reader);
        let end = std::time::Instant::now();

        println!("Took {}mcs", (end - start).as_micros());
    }
}