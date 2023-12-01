pub fn trebuchet(input: &str) -> u32 {
    extremeties(input).iter().sum()
}

fn extremeties(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            let numbers = line
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<char>>();

            // skip lines without numbers like empty lines
            if numbers.is_empty() {
                return 0;
            }

            // get first and last element of the vector.
            let extremeties = [numbers.first().unwrap(), numbers.last().unwrap()];

            extremeties
                .iter()
                .map(|c| c.to_digit(10).unwrap())
                .fold(0, |acc, c| acc * 10 + c)
        })
        .collect()
}

pub fn trebuchet_words(input: &str) -> u32 {
    input
        .lines()
        .map(|line| extremeties_words(line.to_string()))
        .sum()
}

fn extremeties_words(line: String) -> u32 {
    // loop over all number words and find them in the line and return the first number
    let mut first_opts = number_words()
        .iter()
        .filter_map(|word| {
            line.find(word).map(|idx| (idx, word_to_u32(word).unwrap()))
        })
        .collect::<Vec<(usize, u32)>>();

    first_opts.sort_by(|a, b| a.0.cmp(&b.0));
    let first = first_opts.first().unwrap().1;

    // loop over all number words and find them in the line and return the last number
    let mut last_opts = number_words()
        .iter()
        .filter_map(|word| {
            line.rfind(word).map(|idx| (idx, word_to_u32(word).unwrap()))
        })
        .collect::<Vec<(usize, u32)>>();
    last_opts.sort_by(|a, b| a.0.cmp(&b.0));
    let last = last_opts.last().unwrap().1;

    // get first and last element of the vector.
    let extremeties = [first, last];

    extremeties.into_iter().fold(0, |acc, c| acc * 10 + c)
}

fn number_words() -> [&'static str; 18] {
    [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ]
}

fn word_to_u32(input: &str) -> Option<u32> {
    match input {
        "one" | "1" => Some(1),
        "two" | "2" => Some(2),
        "three" | "3" => Some(3),
        "four" | "4" => Some(4),
        "five" | "5" => Some(5),
        "six" | "6" => Some(6),
        "seven" | "7" => Some(7),
        "eight" | "8" => Some(8),
        "nine" | "9" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words_contained() {
        let line = "abcone2threexyz";
        let mut numbers = number_words()
            .iter()
            .filter_map(|word| {
                let idx = line.find(word);
                if idx.is_some() {
                    Some((idx.unwrap(), word_to_u32(word).unwrap()))
                } else {
                    None
                }
            })
            .collect::<Vec<(usize, u32)>>();

        // sort by index
        numbers.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(numbers, vec![(3, 1), (6, 2), (7, 3)]);
    }

    #[test]
    fn test_eigththree() {
        let input = "eighthree";
        assert_eq!(extremeties_words(input.to_string()), 83);
    }

    #[test]
    fn test_sevenine() {
        let input = "sevenine";
        assert_eq!(extremeties_words(input.to_string()), 79);
    }

    #[test]
    fn test_eight_two() {
        let input = "jcb82eightwond";
        assert_eq!(extremeties_words(input.to_string()), 82);
    }

    #[test]
    fn test_regression() {
        let input = "9167ddtxjpxb6";
        assert_eq!(extremeties_words(input.to_string()), 96);
    }
}
