
pub fn run<U: std::fmt::Display, E: std::fmt::Display>(input: Result<U, E>){
    match input{
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}