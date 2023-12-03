use std::fs::read_to_string;

const SYMBOLS: [char; 10] = ['#', '$', '%', '&', '*', '+', '-', '/', '=', '@'];
const NUMBERS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

fn main() {
    let input = read_to_string("puzzle-input").unwrap();

    let accumulated_part_numbers = sum_all_part_numbers(&input);
    println!("Accumulated part numbers: {}", accumulated_part_numbers);
}

fn sum_all_part_numbers(input: &str) -> i32 {
    // # Given Problem
    //
    // - All part numbers need to be found in the input.
    // - All part numbers need to be summed up.
    //
    // # Given Constraints
    //
    // - Numbers adjacent to a symbol are a part number - also diagonaly -.
    // - Periods do not count as a symbol.
    //
    // # Determine possible symbols
    //
    // Extract possible symbols from input via vim:
    //
    // | Action                               | Command     |
    // |--------------------------------------|-------------|
    // | Delete periods                       | %s/\.//g    |
    // | Delete numbers                       | %s/\d//g    |
    // | Join all symbols into one line       | ggVGJ       |
    // | Delete whitespaces                   | s/ //g      |
    // | Put each symbol onto a seperate line | s/./\0\r/g  |
    // | Sort and make unique                 | sort u      |
    //
    // Possible symbols are:
    // #, $, %, &, *, +, -, /, =, @,
    //
    // # Algorithm
    //
    // - Buffer three lines 0(upper), 1(middle) and 2(lower)
    // - Search for numbers in the middle line
    // - Use the index of the numbers to analyze adjacent indexes
    //      - On the upper and lower line generally
    //      - On the first and last line only consider upper or lower adjacent lines respectively
    // - Add the number if it is adjacent to a symbol

    let mut lines = input.lines().peekable();
    let mut accumulated_part_numbers = 0;
    let mut previous_line = lines.peek().unwrap().to_string();

    while let Some(line) = lines.next() {
        let current_line = line;
        let numbers_in_line = find_numbers_in_line(current_line);

        if previous_line == current_line {
            let next_line = lines.peek().unwrap().to_string().clone();

            for number_in_line in numbers_in_line {
                if has_adjacent_symbol(current_line, &next_line, &number_in_line) {
                    accumulated_part_numbers += number_in_line.number;
                }
            }
        } else if lines.peek().is_none() {
            for number_in_line in numbers_in_line {
                if has_adjacent_symbol(current_line, &previous_line, &number_in_line) {
                    accumulated_part_numbers += number_in_line.number;
                }
            }
        } else {
            let next_line = lines.peek().unwrap().to_string().clone();

            for number_in_line in numbers_in_line {
                if has_adjacent_symbol(current_line, &previous_line, &number_in_line)
                    || has_adjacent_symbol(current_line, &next_line, &number_in_line)
                {
                    accumulated_part_numbers += number_in_line.number;
                }
            }
        }

        previous_line = current_line.to_string().clone();
    }

    accumulated_part_numbers
}

fn has_adjacent_symbol(current_line: &str, other_line: &str, string_number: &StringNumber) -> bool {
    let chars_current = current_line.chars().collect::<Vec<char>>();
    let chars_other = other_line.chars().collect::<Vec<char>>();
    let line_len = other_line.len();

    let start_index_modifier = match string_number.start_index {
        0 => 0,
        _ => 1,
    };
    let end_index_modifier = match string_number.end_index {
        x if x == line_len => 0,
        _ => 2,
    };

    if start_index_modifier != 0
        && SYMBOLS.contains(&chars_current[string_number.start_index - start_index_modifier])
    {
        return true;
    }

    if end_index_modifier != 0
        && SYMBOLS.contains(&chars_current[string_number.end_index + end_index_modifier - 1])
    {
        return true;
    }

    let chars_slice = &chars_other[string_number.start_index - start_index_modifier
        ..string_number.end_index + end_index_modifier];

    for char in chars_slice {
        if SYMBOLS.contains(char) {
            return true;
        }
    }

    false
}

#[derive(Debug)]
struct StringNumber {
    start_index: usize,
    end_index: usize,
    number: i32,
}

fn find_numbers_in_line(line_str: &str) -> Vec<StringNumber> {
    let mut string_numbers: Vec<StringNumber> = Vec::new();
    let mut first_index_of_number: Option<usize> = None;
    let mut reading_number = false;

    for (i, char) in line_str.char_indices() {
        let is_line_end = line_str.len() - 1 == i;

        if !reading_number && NUMBERS.contains(&char) {
            reading_number = true;
            first_index_of_number = Some(i);
        } else if reading_number && is_line_end {
            if NUMBERS.contains(&char) {
                string_numbers.push(StringNumber {
                    start_index: first_index_of_number.unwrap(),
                    end_index: i - 1,
                    number: line_str[first_index_of_number.unwrap()..i + 1]
                        .parse::<i32>()
                        .unwrap(),
                });
            } else {
                string_numbers.push(StringNumber {
                    start_index: first_index_of_number.unwrap(),
                    end_index: i - 1,
                    number: line_str[first_index_of_number.unwrap()..i]
                        .parse::<i32>()
                        .unwrap(),
                });
            }
            first_index_of_number = None;
            reading_number = false;
        } else if reading_number && !NUMBERS.contains(&char) {
            string_numbers.push(StringNumber {
                start_index: first_index_of_number.unwrap(),
                end_index: i - 1,
                number: line_str[first_index_of_number.unwrap()..i]
                    .parse::<i32>()
                    .unwrap(),
            });
            first_index_of_number = None;
            reading_number = false;
        }
    }

    string_numbers
}

#[cfg(test)]
mod tests {
    use crate::{find_numbers_in_line, sum_all_part_numbers};

    #[test]
    fn add50() {
        let input = "11....12\n\
                     ..$..$..\n\
                     13....14";

        let result = sum_all_part_numbers(input);

        assert_eq!(result, 50);
    }

    #[test]
    fn add28() {
        let input = "........\n\
                     .24$-4..\n\
                     ......*.";

        let result = sum_all_part_numbers(input);

        assert_eq!(result, 28);
    }

    #[test]
    fn add4() {
        let input = "$......$\n\
                     .1....1.\n\
                     .1....1.\n\
                     $......$";

        let result = sum_all_part_numbers(input);

        assert_eq!(result, 4);
    }

    #[test]
    fn long() {
        let input = "...............776..............552........968..................589...26...........484..............958......186....546.........484.........\n\
                     .........*.........778....................*....124...................................*...............*........%..26.........................\n\
                     ......194..380....@....900..........639....467........478*..............582...........798.............326...........894.........#...........\n\
                     904...........*2.......#......259.....*..........801......464................597.569.............794+................$..218....502..........\n\
                     ...*.....................-...$.....%..431.........*...810.....840+.668..........*.......144=.............................../...........%627.\n\
                     ...890...497.........829.643....504..........465..502..............*........488...................787.184...601....215........-450..........";

        let result = sum_all_part_numbers(input);

        assert_eq!(
            result,
            968 + 484
                + 958
                + 186
                + 778
                + 194
                + 380
                + 900
                + 639
                + 467
                + 478
                + 798
                + 326
                + 894
                + 904
                + 2
                + 259
                + 801
                + 464
                + 597
                + 569
                + 794
                + 218
                + 502
                + 431
                + 840
                + 668
                + 144
                + 627
                + 890
                + 643
                + 504
                + 502
                + 450
        );
    }
    #[test]
    fn minimal_example() {
        let num1 = 467;
        let num2 = 35;
        let num3 = 633;
        let input = format!(
            "{}..114..\n\
            ...*......\n\
            ..{}..{}.\n\
            ......#...",
            num1, num2, num3
        );

        let result = sum_all_part_numbers(&input);

        assert_eq!(result, num1 + num2 + num3);
    }

    #[test]
    fn aoc_example_test_c1() {
        let input = "467..114..\n\
                     ...*......\n\
                     ..35..633.\n\
                     ......#...\n\
                     617*......\n\
                     .....+.58.\n\
                     ..592.....\n\
                     ......755.\n\
                     ...$.*....\n\
                     .664.598..";

        let result = sum_all_part_numbers(input);

        assert_eq!(result, 4361);
    }

    #[test]
    fn finds_numbers_in_line() {
        let number1 = 35;
        let number2 = 633;
        let input = format!("..{}..{}.", number1, number2);

        let result = find_numbers_in_line(&input);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].start_index, 2);
        assert_eq!(result[0].end_index, 3);
        assert_eq!(result[0].number, number1);
        assert_eq!(result[1].start_index, 6);
        assert_eq!(result[1].end_index, 8);
        assert_eq!(result[1].number, number2);
    }

    #[test]
    fn finds_numbers_in_line2() {
        let number1 = 11;
        let number2 = 12;
        let input = format!("{}..{}", number1, number2);

        let result = find_numbers_in_line(&input);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].number, number1);
        assert_eq!(result[1].number, number2);
    }
}
