struct Pile {
    cards: Vec<Card>,
}
impl From<&str> for Pile {
    fn from(input: &str) -> Self {
        let cards = input.lines().map(Card::from).collect();

        Self { cards }
    }
}

struct Card {
    scratched: Vec<usize>,
    owned: Vec<usize>,
}

impl From<&str> for Card {
    fn from(input: &str) -> Self {
        let mut parts = input.split(':');
        let _ = parts.next();
        let mut card_parts = parts.next().unwrap().split('|');

        let scratched = card_parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let owned = card_parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        Self { scratched, owned }
    }
}

impl Card {
    fn wins(&self) -> usize {
        self.scratched
            .iter()
            .filter(|num| self.owned.contains(num))
            .count()
    }

    fn cumulative_wins(&self) -> u32 {
        if self.wins() == 0 {
            return 0;
        }
        2_u32.pow((self.wins() as u32) - 1)
    }
}

pub fn scratch_cards(input: &str) -> u32 {
    let pile = Pile::from(input);

    pile.cards.iter().map(|card| card.cumulative_wins()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pile_test() {
        let input = r#"Card 1: 1 2 | 3 4
Card 2: 5 6 | 7 8"#;

        assert_eq!(Pile::from(input).cards.len(), 2);
    }

    #[test]
    fn card_test() {
        let input = "Card 1: 1 2 | 3 4 5";

        assert_eq!(Card::from(input).scratched.len(), 2);
        assert_eq!(Card::from(input).owned.len(), 3);
    }

    #[test]
    fn card_test_double_spaced() {
        let input = "Card 1: 10  2 | 30 40  5";

        assert_eq!(Card::from(input).scratched.len(), 2);
        assert_eq!(Card::from(input).owned.len(), 3);
    }

    #[test]
    fn card_wins_test() {
        let input = "Card 1: 1 2 | 3 4";
        assert_eq!(Card::from(input).wins(), 0);
    }

    #[test]
    fn card_wins_one_test() {
        let input = "Card 1: 1 2 | 3 1";
        assert_eq!(Card::from(input).wins(), 1);
    }

    #[test]
    fn card_wins_four_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(Card::from(input).wins(), 4);
    }

    #[test]
    fn card_wins_cumulative_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(Card::from(input).cumulative_wins(), 8);
    }
}
