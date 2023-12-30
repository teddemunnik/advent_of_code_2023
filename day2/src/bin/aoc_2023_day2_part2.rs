use aoc_2023_day2::{read_input, Game, DiceCount};

fn calculate_power(game: &Game) -> u32{
    game.records
        .iter()
        .copied()
        .reduce(|a, b|{
            DiceCount{
                red: a.red.max(b.red),
                green: a.green.max(b.green),
                blue: a.blue.max(b.blue),
            }
        })
        .map_or(0, |counts| counts.red * counts.green * counts.blue)
}

fn main(){
    let result = read_input()
    .map(|games| games.iter().map(calculate_power).sum::<u32>());

    match result{
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests{
    use aoc_2023_day2::parse_game;

    use crate::calculate_power;

    #[test]
    fn test_power(){
        const INPUT : &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse_game(INPUT).unwrap();
        let power = calculate_power(&game);
        assert_eq!(power, 48);
    }

}