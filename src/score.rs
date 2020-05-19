use crate::card::{Card};
use crate::hand::{Show};
use itertools::Itertools;

fn isfifteen(cs: &[Card]) -> bool {
    let mut t: i32 = 0;
    for c in cs.iter() {
        t += c.value
    };
    t == 15
}

pub trait Fifteen {
    fn all_cards(&self) -> Vec<Card>;

    fn score_fifteens(&self) -> i32 {
        let mut t: i32 = 0;
        let cards = self.all_cards();

        for n in 2..5 {
            for xs in cards.iter().cloned().combinations(n) {
                if isfifteen(&xs) {
                    t += 2
                }
            }
        }
        t
    }
}

impl Fifteen for Show {
    fn all_cards(&self) -> Vec<Card> {
        let mut cs = self.hand.hand.to_vec();
        cs.push(self.cut);
        cs
    }
}

fn ispair(c1: &Card, c2: &Card) -> bool {
    c1.rank == c2.rank
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_score_fifteens_1() {
        let h = Show::new(["2♡", "3♡", "5♡", "T♡"], "5♣");
        assert_eq!(h.score_fifteens(), 8)
    }

    #[test]
    fn test_ispair_1() {
        assert_eq!(
            ispair(
                &Card::from_chars('K', '♡'),
                &Card::from_chars('K', '♠'),
            ),
            true,
        )
    }

    #[test]
    fn test_ispair_2() {
        assert_eq!(
            ispair(
                &Card::from_chars('A', '♢'),
                &Card::from_chars('A', '♣'),
            ),
            true,
        )
    }

    #[test]
    fn test_ispair_3() {
        assert_eq!(
            ispair(
                &Card::from_chars('A', '♢'),
                &Card::from_chars('A', '♣').ace_high(),
            ),
            true,
        )
    }

    #[test]
    fn test_ispair_4() {
        assert_eq!(
            ispair(
                &Card::from_chars('K', '♡'),
                &Card::from_chars('A', '♣'),
            ),
            false,
        )
    }

    #[test]
    fn test_ispair_5() {
        assert_eq!(
            ispair(
                &Card::from_chars('K', '♡'),
                &Card::from_chars('A', '♣').ace_high(),
            ),
            false,
        )
    }

    fn card_vec(cs: Vec<(char,char)>) -> Vec<Card> {
        cs.iter()
        .map(|(x,y)| Card::from_chars(*x, *y))
        .collect::<Vec<_>>()
    }

    fn test_fifteen(cs: Vec<(char,char)>, b: bool) {
        assert_eq!(
            isfifteen(&card_vec(cs)),
            b,
        )
    }

    #[test]
    fn test_isfifteen_1() {
        test_fifteen(vec![('7', '♡'), ('8', '♡')], true)
    }

    #[test]
    fn test_isfifteen_2() {
        test_fifteen(vec![('5', '♡'), ('5', '♠'), ('5', '♣')], true)
    }

    #[test]
    fn test_isfifteen_3() {
        test_fifteen(vec![('2', '♡'), ('3', '♠'), ('T', '♣')], true)
    }

    #[test]
    fn test_isfifteen_4() {
        test_fifteen(vec![('A', '♡'), ('A', '♠'), ('3', '♠'), ('T', '♣')], true)
    }

    #[test]
    fn test_isfifteen_5() {
        test_fifteen(vec![('A', '♡'), ('3', '♠'), ('4', '♠'), ('4', '♡'), ('3', '♢')], true)
    }

    #[test]
    fn test_isfifteen_6() {
        test_fifteen(vec![('3', '♠'), ('4', '♡'), ('T', '♡')], false)
    }

    #[test]
    fn test_isfifteen_7() {
        println!("{}", Card::from_chars('A', '♣').value);
        assert_eq!(
            isfifteen(
                &vec![
                    Card::from_chars('A', '♡').ace_high(),
                    Card::from_chars('K', '♣'),
                    Card::from_chars('4', '♢'),
                ],
            ),
            true,
        )
    }

    #[test]
    fn test_Show_Fifteen_all_cards() {
        let s = Show::new(["2♡", "3♡", "4♡", "5♡"], "6♡");
        assert_eq!(
            s.all_cards(),
            vec!["2♡", "3♡", "4♡", "5♡", "6♡"]
                .iter()
                .map(|x| Card::from_str(x).unwrap())
                .collect::<Vec<_>>(),
        )
    }


}
