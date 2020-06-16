// https://adventofcode.com/2018/day/2

use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let mut twice = 0;
    let mut thrice = 0;
    for line in contents.split_whitespace() {
        let result = check_box_id(line);
        twice += result.0 as i32;
        thrice += result.1 as i32;
    }

    let result = twice * thrice;

    println!("{}", result);

    Ok(())
}

fn check_box_id(box_id: &str) -> (bool, bool) {
    let mut letters = HashMap::new();

    for value in box_id.chars() {
        let counter = letters.entry(value).or_insert(0);
        *counter += 1;
    }

    let mut twice = false;
    let mut thrice = false;
    for value in letters.values() {
        match value {
            2 => twice = true,
            3 => thrice = true,
            _ => {}
        }
    }

    (twice, thrice)
}

#[cfg(test)]
mod test_check_box_id {
    use super::*;
    use rstest::rstest;

    #[rstest(input, expected,
        case("abcdef", (false, false)),
        case("bababc", (true, true)),
        case("abbcde", (true, false)),
        case("abcccd", (false, true)),
        case("aabcdd", (true, false)),
        case("abcdee", (true, false)),
        case("ababab", (false, true)),
    )]
    fn test_cases(input: &str, expected: (bool, bool)) {
        let result = check_box_id(input);
        assert_eq!(result, expected);
    }
}