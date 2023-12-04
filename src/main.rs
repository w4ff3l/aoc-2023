use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("puzzle-input").unwrap();

    let accumulated_points_c1 = accumulated_points_c1(&input);
    println!("Accumulated points challenge 1: {}", accumulated_points_c1);
    let accumulated_points_c2 = accumulated_cards_c2(&input);
    println!("Accumulated points challenge 2: {}", accumulated_points_c2);
}

fn accumulated_cards_c2(input: &str) -> u32 {
    let mut accumulator = 0;
    let mut copies: HashMap<u32, u32> = HashMap::new();

    let cards = parse_cards(input);
    let size = cards.len() as u32;

    for card in cards {
        copies
            .entry(card.card_number)
            .and_modify(|e| *e += 1)
            .or_insert(1);

        let mut matches = 0;
        let copies_of_card = *copies.get(&card.card_number).unwrap_or(&0);

        for placed_number in &card.placed_numbers {
            if card.winning_numbers.contains(placed_number) {
                matches += 1;
            }
        }

        let next_card_number = card.card_number + 1;
        let upto = next_card_number + matches;

        for n in next_card_number..upto {
            if n <= size {
                copies
                    .entry(n)
                    .and_modify(|e| *e += copies_of_card)
                    .or_insert(copies_of_card);
            }
        }
    }

    for (_, value) in copies.iter() {
        accumulator += value;
    }

    accumulator
}

fn accumulated_points_c1(input: &str) -> u32 {
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

        let card_number_str = card_number_split[0]
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()[1];

        let card_number = card_number_str.parse::<u32>().unwrap();

        let winning_placed_split = card_number_split[1].split(" | ").collect::<Vec<&str>>();
        let winning_numbers = winning_placed_split[0]
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|str| str.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let placed_numbers = winning_placed_split[1]
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|str| str.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let card = Card {
            card_number,
            winning_numbers,
            placed_numbers,
        };
        cards.push(card);
    }
    cards
}

#[derive(Debug)]
struct Card {
    card_number: u32,
    winning_numbers: Vec<u32>,
    placed_numbers: Vec<u32>,
}

#[cfg(test)]
mod tests_c2 {
    use crate::accumulated_cards_c2;

    #[test]
    fn aoc_example_test_c2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = accumulated_cards_c2(input);

        assert_eq!(result, 31);
    }
}

#[cfg(test)]
mod tests_c1 {
    use crate::{accumulated_points_c1, parse_cards};

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
