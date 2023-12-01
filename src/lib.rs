pub fn trebuchet(input: &str) -> u32 {
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
        .sum()
}
