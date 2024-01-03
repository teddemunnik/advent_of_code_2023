
fn parse_line(line: &str) -> Option<Vec<isize>>{
    line.split_ascii_whitespace().map(|entry| entry.parse::<isize>().ok()).collect()
}

fn parse_inputs<R: std::io::BufRead>(input: R) -> Option<Vec<Vec<isize>>>{
    input.lines().map(|line| line.ok().and_then(|line| parse_line(&line))).collect()
}

fn extrapolate_value_forward(input: &[isize]) -> isize{
    if input.iter().all(|input| *input == 0){
        // Base case: Extrapolation for all zero differences
        0
    } else{
        // Recursive case: Extrapolation for nonzero differences
        let mut differences = Vec::with_capacity(input.len() - 1);
        for i in 0..(input.len()-1){
            differences.push(input[i + 1] - input[i]);
        }

        let next_difference = extrapolate_value_forward(&differences);
        input.last().unwrap() + next_difference
    }
}

fn extrapolate_value_backward(input: &[isize]) -> isize{
    if input.iter().all(|input| *input == 0){
        // Base case: Extrapolation for all zero differences
        0
    } else{
        // Recursive case: Extrapolation for nonzero differences
        let mut differences = Vec::with_capacity(input.len() - 1);
        for i in 0..(input.len()-1){
            differences.push(input[i + 1] - input[i]);
        }

        let next_difference = extrapolate_value_backward(&differences);
        input.first().unwrap() - next_difference
    }
}

#[aoc_2023_markup::aoc_task(2023, 9, 1)]
fn part1<R: std::io::BufRead>(input: R) -> Option<isize>{
    let input = parse_inputs(input)?;
    Some(input.iter().map(|input| extrapolate_value_forward(input)).sum())
}

#[aoc_2023_markup::aoc_task(2023, 9, 2)]
fn part2<R: std::io::BufRead>(input: R) -> Option<isize>{
    let input = parse_inputs(input)?;
    Some(input.iter().map(|input| extrapolate_value_backward(input)).sum())
}

#[cfg(test)]
mod test{
    use indoc::indoc;
    use super::*;

    const INPUT :&[u8]= indoc!{"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "}.as_bytes();

    #[test]
    fn test_parse_inputs(){

        let output = parse_inputs(INPUT).unwrap();
        assert_eq!(output,[
            [ 0, 3, 6, 9, 12, 15 ],
            [ 1, 3, 6, 10, 15, 21 ],
            [ 10, 13, 16, 21, 30, 45 ]
        ]);
    }

    #[test]
    fn test_extrapolate_value_forward(){
        let input = parse_inputs(INPUT).unwrap();
        assert_eq!(extrapolate_value_forward(&input[0]), 18);
        assert_eq!(extrapolate_value_forward(&input[1]), 28);
        assert_eq!(extrapolate_value_forward(&input[2]), 68);
    }

    #[test]
    fn test_extrapolate_value_backward(){
        let input = parse_inputs(INPUT).unwrap();
        assert_eq!(extrapolate_value_backward(&input[0]), -3);
        assert_eq!(extrapolate_value_backward(&input[1]), 0);
        assert_eq!(extrapolate_value_backward(&input[2]), 5);
    }

}