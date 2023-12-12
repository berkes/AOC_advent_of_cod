pub fn arrangements_total(input: &str) -> usize {
    let map = SpringMap::from(input);
    map.arrangements_total()
}

struct SpringMap {
    records: Vec<ConditionRecord>,
}

impl SpringMap {
    fn arrangements_total(&self) -> usize {
        self.records
            .iter()
            .map(|record| record.arrangements())
            .sum()
    }
}

impl From<&str> for SpringMap {
    fn from(input: &str) -> Self {
        let records = input
            .lines()
            .map(ConditionRecord::from)
            .collect();
        SpringMap { records }
    }
}

#[derive(Debug)]
struct ConditionRecord {
    row: Row,
    check: Vec<usize>,
}

impl ConditionRecord {
    fn arrangements(&self) -> usize {
        let mut all_variations = Vec::new();
        generate_variations(self.row.clone(), &mut all_variations);

        // remove any that still have unknowns
        all_variations.retain(|v| !v.0.contains(&Condition::Unknown));

        // check each variation against the check
        let matching = all_variations.iter().filter(|v| v.matches(&self.check));

        matching.count()
    }
}

impl From<&str> for ConditionRecord {
    fn from(input: &str) -> Self {
        let mut row = Row(Vec::new());
        let mut check = Vec::new();

        let mut parts = input.split_whitespace();
        // TODO: refactor to use iterators
        for c in parts.next().unwrap().chars() {
            match c {
                '.' => row.0.push(Condition::Operational),
                '#' => row.0.push(Condition::Damaged),
                '?' => {
                    row.0.push(Condition::Unknown);
                }
                _ => panic!("Invalid input"),
            }
        }

        for c in parts.next().unwrap().split(',') {
            check.push(c.parse().unwrap());
        }
        ConditionRecord { row, check }
    }
}

fn generate_variations(mut row: Row, variations: &mut Vec<Row>) {
    // return if there are no more unknowns
    if !row.0.contains(&Condition::Unknown) {
        return;
    }

    // find the first unknown and call generate_variations for Operational and Damaged
    let first_unknown = row.0.iter().position(|c| c == &Condition::Unknown).unwrap();

    row.0[first_unknown] = Condition::Operational;
    variations.push(row.clone());
    generate_variations(row.clone(), variations);

    row.0[first_unknown] = Condition::Damaged;
    variations.push(row.clone());
    generate_variations(row.clone(), variations);
}

#[derive(Debug, Clone)]
struct Row(Vec<Condition>);
impl Row {
    fn matches(&self, check: &[usize]) -> bool {
        // Split the row on all operational entries
        let lengths = self
            .0
            .split(|c| c == &Condition::Operational)
            .map(|segment| segment.len())
            .filter(|&length| length > 0)
            .collect::<Vec<usize>>();

        lengths == check
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_arrangement() {
        let input = "???.### 1,1,3";
        assert_eq!(SpringMap::from(input).arrangements_total(), 1);
    }

    #[test]
    fn test_10_arrangement() {
        let input = "?###???????? 3,2,1";
        assert_eq!(SpringMap::from(input).arrangements_total(), 10);
    }

    #[test]
    fn test_generate_variations() {
        let input = "???.###";

        let mut variations = Vec::new();
        let row = Row(input
            .chars()
            .map(|c| match c {
                '?' => Condition::Unknown,
                '.' => Condition::Operational,
                '#' => Condition::Damaged,
                _ => panic!("Invalid input"),
            })
            .collect::<Vec<Condition>>());
        generate_variations(row, &mut variations);

        assert_eq!(variations.len(), 14);
    }
}
