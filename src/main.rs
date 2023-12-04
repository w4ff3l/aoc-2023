use std::{fs::read_to_string, i32};

fn main() {
    let input = read_to_string("puzzle-input").unwrap();

    let accumulated_points = accumulated_points_c1(&input);
    println!("Accumulated points challenge 1: {}", accumulated_points);
}

fn accumulated_points_c1(input: &str) -> i32 {
    let mut accumulator = 0;

    let cards = parse_cards(input);

    for card in cards {
        let mut points = 0;
        for placed_number in card.placed_numbers {
            if card.winning_numbers.contains(&placed_number) {
                if points == 0 {
                    points += 1;
                } else {
                    points *= 2;
                }
            }
        }
        accumulator += points;
    }

    accumulator
}

fn parse_cards(input: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for line in input.lines() {
        let card_number_split = line.split(": ").collect::<Vec<&str>>();

        let winning_placed_split = card_number_split[1].split(" | ").collect::<Vec<&str>>();
        let winning_numbers = winning_placed_split[0]
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|str| str.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let placed_numbers = winning_placed_split[1]
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|str| str.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let card = Card {
            winning_numbers,
            placed_numbers,
        };
        cards.push(card);
    }
    cards
}

struct Card {
    winning_numbers: Vec<i32>,
    placed_numbers: Vec<i32>,
}

#[cfg(test)]
mod tests_c1 {
    use crate::{parse_cards, accumulated_points_c1};

    #[test]
    fn aoc_example_test_c1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = accumulated_points_c1(input);

        assert_eq!(result, 13);
    }

    #[test]
    fn parses_cards() {
        let input = "Card 1: 11 12 | 21 22\n\
                     Card 2: 31 32 | 41 42";

        let results = parse_cards(input);
        let result_card_1 = &results[0];
        let result_card_2 = &results[1];

        assert_eq!(result_card_1.winning_numbers.len(), 2);
        assert_eq!(result_card_1.placed_numbers.len(), 2);

        assert_eq!(result_card_2.winning_numbers.len(), 2);
        assert_eq!(result_card_2.placed_numbers.len(), 2);
    }
}
