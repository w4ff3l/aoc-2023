use crate::hand::Hand;

pub fn parse_hands(input: &str) -> Vec<Hand> {
    let mut hands = Vec::new();

    for line in input.lines() {
        let cards_bid_split = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let cards = cards_bid_split[0]
            .chars()
            .map(map_card_to_number)
            .collect::<Vec<u32>>();
        let bid = cards_bid_split[1].parse::<u32>().unwrap();

        let hand = Hand { cards, bid };
        hands.push(hand);
    }

    hands
}

fn map_card_to_number(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        number => number.to_digit(10).unwrap(),
    }
}
