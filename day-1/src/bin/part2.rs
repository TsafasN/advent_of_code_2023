fn main() {

    let input_file = include_str!("../../input1.txt");
    let output = part1(input_file);

    println!("{:?}", output);

    dbg!(output.unwrap().unwrap());
    
}

fn part1(input: &str) -> Result<Option<u32>, String> {

    let output = input
        .lines()
        .map(process_line)
        .sum::<u32>();

    Ok(Some(output))

}

fn process_line(line: &str) -> u32{

    let mut index = 0;
    let line_iter = std::iter::from_fn(move || {
        let reduced_line = &line[index..];

        let result = 
        if reduced_line.starts_with("one"){
            Some('1')
        } else if reduced_line.starts_with("two"){
            Some('2')
        } else if reduced_line.starts_with("three"){
            Some('3')
        } else if reduced_line.starts_with("four"){
            Some('4')
        } else if reduced_line.starts_with("five"){
            Some('5')
        } else if reduced_line.starts_with("six"){
            Some('6')
        } else if reduced_line.starts_with("seven"){
            Some('7')
        } else if reduced_line.starts_with("eight"){
            Some('8')
        } else if reduced_line.starts_with("nine"){
            Some('9')
        } else if reduced_line.starts_with("zero"){
            Some('0')
        } else {
            let result = reduced_line.chars().next();
            result
        };
        index += 1;
        result
    });
    
    let mut iter = line_iter
        .filter_map(|character| { 
            character.to_digit(10)
        });

    let first = iter
        .next()
        .expect("Should be a number");
    
    let last  = iter
        .last();

    match last {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }.parse::<u32>().expect("should be a valid number")

}



#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    // this test is from the real input
    // it tests two overlapping numbers
    // where the second number should succeed
    #[case("five75pvngkvx9nnlttwotwonev", 51)]
    fn line_test(
        #[case] line: &str,
        #[case] expected: u32,
    ) {
        assert_eq!(expected, process_line(line))
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        
        let result = part1(input);

        assert_eq!(Ok(Some(281)), result);
    }
}