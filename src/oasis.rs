#[derive(Debug, PartialEq)]
struct Report {
    histories: Vec<History>,
}
impl From<&str> for Report {
    fn from(input: &str) -> Self {
        let histories = input.lines().map(History::from).collect();
        Report { histories }
    }
}
impl Report {
    fn extrapolations(&self) -> Vec<isize> {
        self.histories.iter().map(|h| h.extrapolation()).collect()
    }

    fn extrapolations_past(&self) -> Vec<isize> {
        self.histories
            .iter()
            .map(|h| h.extrapolation_past())
            .collect()
    }
}

#[derive(Debug, PartialEq)]
struct History {
    numbers: Vec<isize>,
}

impl From<&str> for History {
    fn from(input: &str) -> Self {
        let numbers = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        History { numbers }
    }
}

impl History {
    fn extrapolation(&self) -> isize {
        let mut all_series = vec![];
        // take the numbers as current series
        all_series.push(self.numbers.clone());
        // calculate the difference between each number 1, 3, 7 -> 2, 4 add that to all_series
        // calculate the difference for the next series 2, 4 -> 2 add that to all_series
        // repeat until current series is uniform: consists of only one number store this as
        // uniform number
        while !is_uniform(all_series.last().unwrap()) {
            let last_series = all_series.last().unwrap();
            let mut next_series = vec![];
            for i in 0..last_series.len() - 1 {
                next_series.push(last_series[i + 1] - last_series[i]);
            }
            all_series.push(next_series);
        }

        // calculate the last number in the previous series by adding the uniform number to the
        // last element of the previous series
        // repeat until we are back at the original series
        all_series.iter().rev().fold(0, |acc, series| {
            let last = series.last().unwrap();
            acc + last
        })
    }

    fn extrapolation_past(&self) -> isize {
        let reverse_history = History {
            numbers: self.numbers.iter().rev().cloned().collect(),
        };
        reverse_history.extrapolation()
    }
}

fn is_uniform(unwrap: &[isize]) -> bool {
    let first = unwrap[0];
    unwrap.iter().all(|&n| n == first)
}

pub fn extrapolations_sum(input: &str) -> isize {
    let report = Report::from(input);
    report.extrapolations().iter().sum()
}

pub fn extrapolations_past_sum(input: &str) -> isize {
    let report = Report::from(input);
    report.extrapolations_past().iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_from() {
        let input = r#"1 3 2
3 4 5"#;
        let report = Report::from(input);

        assert_eq!(report.histories.len(), 2);
    }

    #[test]
    fn test_report_from_with_negative_numbers() {
        let input = r#"1 -3 2"#;
        let report = Report::from(input);
        assert_eq!(report.histories.len(), 1);
    }

    #[test]
    fn test_history_from() {
        let input = "1 3 2";
        assert_eq!(
            History::from(input),
            History {
                numbers: vec![1, 3, 2]
            }
        );
    }
}
