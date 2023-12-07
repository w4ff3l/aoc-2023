use std::cmp::Ordering;

use crate::type_of_hand::TypeOfHand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    pub cards: Vec<u32>,
    pub bid: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RatedHand {
    pub tuples: Vec<(u32, u32)>,
    pub hand_type: TypeOfHand,
}

impl From<Hand> for RatedHand {
    fn from(mut hand: Hand) -> Self {
        hand.cards.sort();

        let mut last_card = hand.cards[0];
        let mut tuples: Vec<(u32, u32)> = Vec::new();

        let mut counter = 1;

        for (i, card) in hand.cards.iter().enumerate().skip(1) {
            if card == &last_card {
                counter += 1;
                if i == 4 {
                    tuples.push((*card, counter));
                }
            } else if card != &last_card {
                tuples.push((last_card, counter));
                counter = 1;
                if i == 4 {
                    tuples.push((*card, 1));
                }
            }
            last_card = *card;
        }

        if tuples.len() == 1 {
            RatedHand {
                tuples,
                hand_type: TypeOfHand::FiveOfKind,
            }
        } else if tuples.len() == 2 {
            if tuples[0].1 == 3 || tuples[1].1 == 3 {
                RatedHand {
                    tuples,
                    hand_type: TypeOfHand::FullHouse,
                }
            } else {
                RatedHand {
                    tuples,
                    hand_type: TypeOfHand::FourOfKind,
                }
            }
        } else if tuples.len() == 3 {
            if tuples[0].1 == 3 || tuples[1].1 == 3 || tuples[2].1 == 3 {
                RatedHand {
                    tuples,
                    hand_type: TypeOfHand::ThreeOfKind,
                }
            } else {
                RatedHand {
                    tuples,
                    hand_type: TypeOfHand::TwoPair,
                }
            }
        } else if tuples.len() == 4 {
            RatedHand {
                tuples,
                hand_type: TypeOfHand::OnePair,
            }
        } else {
            RatedHand {
                tuples,
                hand_type: TypeOfHand::HighCard,
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_rated = RatedHand::from(self.clone());
        let other_rated = RatedHand::from(other.clone());

        let self_strength = self_rated.hand_type.strength();
        let other_strength = other_rated.hand_type.strength();

        if self_strength == other_strength {
            for (self_card, other_card) in self.cards.iter().zip(&other.cards) {
                if self_card != other_card {
                    return self_card.cmp(other_card);
                }
            }
            Ordering::Equal
        } else {
            self_strength.cmp(&other_strength)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests_hand {
    use crate::{
        hand::{Hand, RatedHand},
        type_of_hand::TypeOfHand,
    };

    #[test]
    #[rustfmt::skip]
    fn sorts_correctly() {
        let five_of_kind = Hand { cards: vec![1, 1, 1, 1, 1], bid: 3, };
        let three_of_kind = Hand { cards: vec![1, 1, 1, 2, 3], bid: 1, };
        let four_of_kind = Hand { cards: vec![1, 1, 1, 1, 2], bid: 2, };

        let mut hands = vec![five_of_kind, three_of_kind, four_of_kind];
        hands.sort();

        assert_eq!(hands[0].bid, 1);
        assert_eq!(hands[1].bid, 2);
        assert_eq!(hands[2].bid, 3);
    }

    #[test]
    #[rustfmt::skip]
    fn determines_type() {
        let five_of_kind =  Hand { cards: vec![1, 1, 1, 1, 1], bid: 0, };
        let four_of_kind =  Hand { cards: vec![1, 1, 1, 1, 2], bid: 0, };
        let full_house =    Hand { cards: vec![1, 1, 1, 2, 2], bid: 0, };
        let three_of_kind = Hand { cards: vec![1, 1, 1, 2, 3], bid: 0, };

        assert_eq!(RatedHand::from(five_of_kind), RatedHand{ tuples: vec![(1, 5)], hand_type: TypeOfHand::FiveOfKind });
        assert_eq!(RatedHand::from(four_of_kind), RatedHand{ tuples: vec![(1, 4), (2, 1)], hand_type: TypeOfHand::FourOfKind });
        assert_eq!(RatedHand::from(full_house), RatedHand{ tuples: vec![(1, 3), (2, 2)], hand_type: TypeOfHand::FullHouse });
        assert_eq!(RatedHand::from(three_of_kind), RatedHand{ tuples: vec![(1, 3), (2, 1), (3, 1)], hand_type: TypeOfHand::ThreeOfKind });
    }
}
