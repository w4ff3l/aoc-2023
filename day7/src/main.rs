mod hand;
mod parser;
mod type_of_hand;

use std::{fs::read_to_string, u32};

use crate::parser::parse_hands;

fn main() {
    let input = read_to_string("puzzle-input").unwrap();

    let result_challenge_1 = calculate_result_challenge_1(&input);
    println!("Result for challenge 1: {}", result_challenge_1);
}

fn calculate_result_challenge_1(input: &str) -> u32 {
    let mut hands = parse_hands(input);
    hands.sort();

    let mut accumulator = 0;

    for (i, hand) in hands.iter().enumerate() {
        accumulator += (i as u32 + 1) * hand.bid;
    }

    accumulator
}

#[cfg(test)]
mod tests_c1 {
    use crate::calculate_result_challenge_1;

    #[test]
    fn c1_small() {
        let input = "KTJJT 34\n\
                     KK677 7";

        let result = calculate_result_challenge_1(input);

        assert_eq!(result, 6592);
    }

    #[test]
    fn c1_complc() {
        let input = "2345A 1\n\
                     Q2KJJ 13\n\
                     Q2Q2Q 19\n\
                     T3T3J 17\n\
                     T3Q33 11\n\
                     2345J 3\n\
                     J345A 2\n\
                     32T3K 5\n\
                     T55J5 29\n\
                     KK677 7\n\
                     KTJJT 34\n\
                     QQQJA 31\n\
                     JJJJJ 37\n\
                     JAAAA 43\n\
                     AAAAJ 59\n\
                     AAAAA 61\n\
                     2AAAA 23\n\
                     2JJJJ 53\n\
                     JJJJ2 41";

        let result = calculate_result_challenge_1(input);

        assert_eq!(result, 6592);
    }
    #[test]
    fn challenge_1_example_test() {
        let input = "32T3K 765\n\
                     T55J5 684\n\
                     KK677 28\n\
                     KTJJT 220\n\
                     QQQJA 483";

        let result = calculate_result_challenge_1(input);

        assert_eq!(result, 6440);
    }
}
