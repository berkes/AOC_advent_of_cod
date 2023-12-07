struct CamelCards {
    hands: Vec<Hand>,
}
impl CamelCards {
    fn ranked_bids(&self) -> Vec<(usize, usize)> {
        // loop through the hands, starting with the lowest value, and record the index + 1 of the hand
        // with the bid of that hand
        self.hands
            .iter()
            .enumerate()
            .map(|(index, hand)| (index + 1, hand.bid))
            .collect::<Vec<(usize, usize)>>()
    }
}

impl From<&str> for CamelCards {
    fn from(input: &str) -> Self {
        let mut hands = input
            .lines()
            .map(|line| Hand::from(line))
            .collect::<Vec<Hand>>();

        hands.sort();

        Self { hands }
    }
}

struct Hand {
    cards: Vec<Card>,
    bid: usize,
}
impl From<&str> for Hand {
    fn from(line: &str) -> Self {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let hand = parts[0];

        let cards = hand
            .chars()
            .map(|card| Card::from(&card))
            .collect::<Vec<Card>>();

        let bid = parts[1].parse::<usize>().unwrap();

        Self { cards, bid }
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // compare the hand types
        match self.hand_type().cmp(&other.hand_type()) {
            std::cmp::Ordering::Equal => self.value().cmp(&other.value()),
            ordering => ordering,
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn value(&self) -> usize {
        self.cards
            .iter()
            .fold(0, |acc, card| acc * 100 + card.value)
    }

    fn hand_type(&self) -> HandType {
        let mut counts = [0; 15];
        for card in &self.cards {
            counts[card.value] += 1;
        }

        let mut counts = counts.to_vec();
        counts.sort();

        let counts = counts.iter().rev().collect::<Vec<&usize>>();

        let mut hand_type = HandType::HighCard;

        if counts[0] == &5 {
            hand_type = HandType::FiveOfAKind;
        } else if counts[0] == &4 {
            hand_type = HandType::FourOfAKind;
        } else if counts[0] == &3 && counts[1] == &2 {
            hand_type = HandType::FullHouse;
        } else if counts[0] == &3 {
            hand_type = HandType::ThreeOfAKind;
        } else if counts[0] == &2 && counts[1] == &2 {
            hand_type = HandType::TwoPair;
        } else if counts[0] == &2 {
            hand_type = HandType::OnePair;
        }

        hand_type
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Card {
    value: usize,
}

impl From<&char> for Card {
    fn from(card: &char) -> Self {
        let value = match card {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => card.to_digit(10).unwrap() as usize,
        };

        Self { value }
    }
}

pub fn total_winnings(input: &str) -> usize {
    let hands = CamelCards::from(input).ranked_bids();
    hands.iter().map(|(hand, bid)| hand * bid).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hands_and_bids() {
        let input = r#"TJ 1"#;
        assert_eq!(CamelCards::from(input).hands[0].cards[0].value, 10);
    }

    #[test]
    fn test_ranked_bids() {
        let input = r#"33332 100
2AAAA 200"#;
        assert_eq!(
            CamelCards::from(input).ranked_bids(),
            vec![(1, 200), (2, 100)]
        );
    }

    #[test]
    fn test_hand_type() {
        assert_eq!(
            CamelCards::from("AAAAA 100").hands[0].hand_type(),
            HandType::FiveOfAKind
        );
        assert_eq!(
            CamelCards::from("33332 100").hands[0].hand_type(),
            HandType::FourOfAKind
        );

        assert_eq!(
            CamelCards::from("33322 100").hands[0].hand_type(),
            HandType::FullHouse
        );
        assert_eq!(
            CamelCards::from("23332 100").hands[0].hand_type(),
            HandType::FullHouse
        );

        assert_eq!(
            CamelCards::from("TTT98 100").hands[0].hand_type(),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            CamelCards::from("23432 100").hands[0].hand_type(),
            HandType::TwoPair
        );

        assert_eq!(
            CamelCards::from("A23A4 100").hands[0].hand_type(),
            HandType::OnePair
        );
        assert_eq!(
            CamelCards::from("23456 100").hands[0].hand_type(),
            HandType::HighCard
        );
    }

    #[test]
    fn test_hand_type_ordering() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);

        assert!(Hand::from("AAAAA 100") > Hand::from("33332 100"));
        assert!(Hand::from("33332 100") > Hand::from("33322 100"));
        assert!(Hand::from("33322 100") > Hand::from("23332 100"));

        assert!(Hand::from("23332 100") > Hand::from("TTT98 100"));
        assert!(Hand::from("TTT98 100") > Hand::from("23432 100"));
        assert!(Hand::from("23432 100") > Hand::from("A23A4 100"));
        assert!(Hand::from("A23A4 100") > Hand::from("23456 100"));
    }
}
