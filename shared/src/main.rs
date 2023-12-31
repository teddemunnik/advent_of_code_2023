
use std::{io::{BufRead, Cursor, BufReader}, fs::File};
pub use linkme;

mod day1;
mod day2;
mod day3;
mod day4;

pub trait AocTask{
    fn year(&self) -> u32;
    fn day(&self) -> u32;
    fn part(&self) -> u32;
    fn invoke(&self, reader: &mut dyn BufRead);
}

#[linkme::distributed_slice]
pub static AOC_ENTRIES: [&(dyn AocTask + Sync)];

pub fn run<U: std::fmt::Display, E: std::fmt::Display>(input: Result<U, E>){
    match input{
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
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
        entry.invoke(&mut reader);
    }
}