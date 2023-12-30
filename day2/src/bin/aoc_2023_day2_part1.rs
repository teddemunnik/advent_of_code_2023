use aoc_2023_day2::{Game, DiceCount, has_enough_dice, read_input};

fn add_possible_games(games: &[Game], available_dice: &DiceCount) -> u32{
    let possible_games = games.iter().filter(|game| game.records.iter().all(|roll| has_enough_dice(available_dice, roll)));
    possible_games.map(|game| game.id).reduce(|a, b| a + b).unwrap_or(0)
}

fn main(){
    let available_dice = DiceCount{
        red: 12,
        green: 13,
        blue: 14
    };

    aoc_2023_shared::run(read_input().map(|games| add_possible_games(&games, &available_dice)));
}

#[cfg(test)]
mod tests{
    use aoc_2023_day2::parse_games;

    use super::*;

    #[test]
    fn add_possible_games(){
        const INPUT_GAMES : &[u8] =
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
             Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
             Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
             Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
             Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".as_bytes();

        const INPUT_DICE : DiceCount = DiceCount{
            red: 12,
            green: 13,
            blue: 14
        };

        let games = parse_games(INPUT_GAMES).unwrap();
        let count = super::add_possible_games(&games, &INPUT_DICE);
        assert_eq!(count, 8);
    }
}