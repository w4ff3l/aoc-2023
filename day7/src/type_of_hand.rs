#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeOfHand {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl TypeOfHand {
    pub fn strength(&self) -> i32 {
        match self {
            TypeOfHand::FiveOfKind => 7,
            TypeOfHand::FourOfKind => 6,
            TypeOfHand::FullHouse => 5,
            TypeOfHand::ThreeOfKind => 4,
            TypeOfHand::TwoPair => 3,
            TypeOfHand::OnePair => 2,
            TypeOfHand::HighCard => 1,
        }
    }
}
