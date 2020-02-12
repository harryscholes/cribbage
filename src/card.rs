use std::{fmt, string, char, str};
use std::str::FromStr;

const SUITS: [char; 4] = ['♡', '♠', '♢', '♣'];

const RANKS: [char; 13] = ['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'];

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Card {
    pub mask: i32, // make private?
    pub suit: char,
    pub rank: char,
    pub value: i32, // make private?
}

impl Card {
    pub fn from_chars(rank: char, suit: char) -> Self {
        if !RANKS.contains(&rank) {
            panic!("Invalid `rank`");
        } else if !SUITS.contains(&suit) {
            panic!("Invalid `suit`");
        };
        let mask = RANKS.iter().position(|x| x == &rank).unwrap() as i32;
        let value = match mask > 10 {
            true => 10,
            _ => mask.clone(),
        };
        Self {
            mask,
            suit,
            rank,
            value,
        }
    }

    pub fn new(s: &str) -> Self {
        Self::from_str(&s).unwrap()
    }

    pub fn ace_high(&self) -> Self {
        if self.rank != 'A' {
            panic!("`rank` != 'A'")
        }
        Self {
            mask: 13,
            suit: self.suit,
            rank: self.rank,
            value: self.value,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl FromStr for Card {
    type Err = string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() != 2 {
            panic!("Number of characters in `s` != 2")
        };
        Ok(Card::from_chars(
            s.chars().nth(0).unwrap(),
            s.chars().nth(1).unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_equality_1() {
        assert_eq!(Card::from_chars('A', '♣'), Card::from_chars('A', '♣'))
    }

    #[test]
    fn test_card_equality_2() {
        assert_eq!(Card::from_chars('3', '♡'), Card::from_chars('3', '♡'))
    }

    #[test]
    fn test_card_equality_3() {
        assert_eq!(Card::from_chars('7', '♠'), Card::from_chars('7', '♠'))
    }

    #[test]
    fn test_card_equality_4() {
        assert_eq!(Card::from_chars('J', '♢'), Card::from_chars('J', '♢'))
    }

    #[test]
    fn test_card_inequality_1() {
        assert_ne!(Card::from_chars('A', '♣'), Card::from_chars('A', '♡'))
    }

    #[test]
    fn test_card_inequality_2() {
        assert_ne!(Card::from_chars('3', '♡'), Card::from_chars('A', '♠'))
    }

    #[test]
    fn test_card_inequality_3() {
        assert_ne!(Card::from_chars('7', '♠'), Card::from_chars('7', '♢'))
    }

    #[test]
    fn test_card_inequality_4() {
        assert_ne!(Card::from_chars('J', '♢'), Card::from_chars('J', '♣'))
    }

    #[test]
    fn test_card_rank() {
        assert_eq!(Card::from_chars('A', '♣').rank, 'A')
    }

    #[test]
    fn test_card_suit() {
        assert_eq!(Card::from_chars('A', '♣').suit, '♣')
    }

    #[test]
    fn test_card_mask_1() {
        assert_eq!(Card::from_chars('A', '♣').mask, 0)
    }

    #[test]
    fn test_card_mask_2() {
        assert_eq!(Card::from_chars('3', '♣').mask, 2)
    }

    #[test]
    fn test_card_mask_3() {
        assert_eq!(Card::from_chars('K', '♣').mask, 12)
    }

    #[test]
    fn test_card_mask_ace_high() {
        assert_eq!(Card::from_chars('A', '♣').ace_high().mask, 13)
    }

    #[test]
    fn test_card_ordering_1() {
        assert_eq!(Card::from_chars('A', '♣') < Card::from_chars('3', '♣'), true)
    }

    #[test]
    fn test_card_ordering_2() {
        assert_eq!(Card::from_chars('A', '♣') < Card::from_chars('3', '♡'), true)
    }

    #[test]
    fn test_card_ordering_3() {
        assert_eq!(Card::from_chars('A', '♡') < Card::from_chars('A', '♣'), true)
    }

    #[test]
    fn test_card_ordering_4() {
        assert_eq!(Card::from_chars('A', '♡') < Card::from_chars('A', '♢'), true)
    }

    #[test]
    fn test_card_ordering_5() {
        assert_eq!(Card::from_chars('A', '♢') < Card::from_chars('A', '♣'), true)
    }

    #[test]
    fn test_card_ordering_6() {
        assert_eq!(Card::from_chars('A', '♠') < Card::from_chars('A', '♢'), true)
    }

    #[test]
    fn test_card_ordering_ace_high_1() {
        assert_eq!(Card::from_chars('A', '♣') < Card::from_chars('A', '♣').ace_high(), true)
    }

    #[test]
    fn test_card_ordering_ace_high_2() {
        assert_eq!(Card::from_chars('A', '♣') < Card::from_chars('A', '♠').ace_high(), true)
    }

    #[test]
    fn test_card_ordering_ace_high_3() {
        assert_eq!(Card::from_chars('A', '♣').ace_high() > Card::from_chars('A', '♣'), true)
    }

    #[test]
    fn test_card_ordering_ace_high_4() {
        assert_eq!(Card::from_chars('A', '♠').ace_high() > Card::from_chars('A', '♣'), true)
    }

    #[test]
    fn test_card_to_string_1() {
        assert_eq!(Card::from_chars('A', '♠').to_string(), "A♠")
    }

    #[test]
    fn test_card_to_string_2() {
        assert_eq!(Card::from_chars('K', '♡').to_string(), "K♡")
    }

    #[test]
    fn test_card_ace_high_1() {
        Card::from_chars('A', '♡').ace_high();
    }

    #[test]
    #[should_panic]
    fn test_card_ace_high_2() {
        Card::from_chars('K', '♡').ace_high();
    }

    #[test]
    fn test_card_from_str_1() {
        assert_eq!(Card::from_str("A♡").unwrap(), Card::from_chars('A', '♡'))
    }

    #[test]
    #[should_panic]
    fn test_card_from_str_2() {
        Card::from_str("A♡X");
    }

    #[test]
    #[should_panic]
    fn test_card_from_str_3() {
        Card::from_str("A");
    }
}
