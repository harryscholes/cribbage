use crate::card::{Card, deck};
use rand::{thread_rng, seq::{SliceRandom}};

#[derive(Eq, PartialEq, Debug)]
pub struct Hand {
    hand: [Card; 4],
}

impl Hand {
    pub fn new(cs: [&str; 4]) -> Self {
        Hand {
            hand: [Card::new(cs[0]), Card::new(cs[1]), Card::new(cs[2]), Card::new(cs[3])]
        }
    }

    pub fn rand() -> Self {
        let mut rng = thread_rng();
        let cs: Vec<Card> = deck().choose_multiple(&mut rng, 4).cloned().collect();
        Hand {
            hand: [cs[0], cs[1], cs[2], cs[3]]
        }
    }
}

pub fn hand(cs: [&str; 4]) -> Hand {
    Hand::new(cs)
}

#[derive(Eq, PartialEq, Debug)]
pub struct Show {
    hand: Hand,
    cut: Card,
}

impl Show {
    pub fn new(hand: [&str; 4], cut: &str) -> Self {
        Show {
            hand: Hand::new(hand),
            cut: Card::new(cut),
        }
    }

    pub fn rand() -> Self {
        let mut rng = thread_rng();
        let cs: Vec<Card> = deck().choose_multiple(&mut rng, 5).cloned().collect();
        Show {
            hand: Hand {
                hand: [cs[0], cs[1], cs[2], cs[3]]
            },
            cut: cs[4],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_constructor_1() {
        Hand::new(["2♡", "3♡", "4♡", "5♡"]);
    }

    #[test]
    fn test_hand_equality_1() {
        assert_eq!(Hand::new(["2♡", "3♡", "4♡", "5♡"]), Hand::new(["2♡", "3♡", "4♡", "5♡"]));
    }

    #[test]
    fn test_hand_rand_1() {
        assert_eq!(Hand::rand().hand.len(), 4);
    }

    #[test]
    fn test_hand_fn_1() {
        assert_eq!(hand(["2♡", "3♡", "4♡", "5♡"]), Hand::new(["2♡", "3♡", "4♡", "5♡"]));
    }

    #[test]
    fn test_show_constructor_1() {
        Show::new(["2♡", "3♡", "4♡", "5♡"], "6♡");
    }

    #[test]
    fn test_show_constructor_2() {
        let s = Show::new(["2♡", "3♡", "4♡", "5♡"], "6♡");
        assert_eq!(s.hand, Hand::new(["2♡", "3♡", "4♡", "5♡"]))
    }

    #[test]
    fn test_show_constructor_3() {
        let s = Show::new(["2♡", "3♡", "4♡", "5♡"], "6♡");
        assert_eq!(s.cut, Card::new("6♡"))
    }
}
