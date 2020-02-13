use crate::card::{Card, deck};
use rand::{thread_rng, seq::{SliceRandom}};

#[derive(Eq, PartialEq, Debug)]
pub struct Hand {
    hand: [Card; 4],
}

impl Hand {
    pub fn new(c1: &str, c2: &str, c3: &str, c4: &str) -> Self {
        Hand{hand: [Card::new(c1), Card::new(c2), Card::new(c3), Card::new(c4)]}
    }

    pub fn rand() -> Self {
        let mut rng = thread_rng();
        let cs: Vec<Card> = deck().choose_multiple(&mut rng, 4).cloned().collect();
        Hand{hand: [cs[0], cs[1], cs[2], cs[3]]}
    }
}

pub fn hand(cs: [&str; 4]) -> Hand {
    Hand::new(cs[0], cs[1], cs[2], cs[3])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_constructor_1() {
        Hand::new("2♡", "3♡", "4♡", "5♡");
    }

    #[test]
    fn test_hand_equality_1() {
        assert_eq!(Hand::new("2♡", "3♡", "4♡", "5♡"), Hand::new("2♡", "3♡", "4♡", "5♡"));
    }

    #[test]
    fn test_hand_rand_1() {
        assert_eq!(Hand::rand().hand.len(), 4);
    }

    #[test]
    fn test_hand_fn_1() {
        assert_eq!(hand(["2♡", "3♡", "4♡", "5♡"]), Hand::new("2♡", "3♡", "4♡", "5♡"));
    }
}
