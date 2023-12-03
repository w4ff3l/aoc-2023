use std::{fs::read_to_string, i32};

const SYMBOLS: [char; 10] = ['#', '$', '%', '&', '*', '+', '-', '/', '=', '@'];
const NUMBERS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

fn main() {
    let input = read_to_string("puzzle-input").unwrap();

    let accumulated_part_numbers = sum_all_part_numbers_c1(&input);
    println!("Accumulated part numbers: {}", accumulated_part_numbers);

    let accumulated_gear_ratios = sum_all_gear_ratios_c2(&input);
    println!("Accumulated gear ratios: {}", accumulated_gear_ratios);
}

fn sum_all_gear_ratios_c2(input: &str) -> i32 {
    // # Given Problem
    //
    // - Gear ratios are connected via gears (*)
    // - Connected gear ratios need to be multiplied
    // - Multiplied ratios musst me accumulated
    //
    // # Algorithm
    //
    // - Buffer current line and the two next lines
    // - Find numbers in current line
    // - Find gears in second line
    // - Connect gear with numbers from first line
    // - Find numbers in third line
    // - Connect gears with numbers from third line
    // - Mix and match?

    let lines = input.lines().peekable();
    let mut accumulated_gear_ratios = 0;

    let mut line_index = 0;
    let mut gears: Vec<Gear> = Vec::new();
    let mut grid_numbers: Vec<GridNumber> = Vec::new();

    for line in lines {
        let gears_in_current_line = find_gears_in_line(line_index, line);
        let numbers_in_current_line = find_numbers_in_line(line_index, line);

        gears.extend(gears_in_current_line);
        grid_numbers.extend(numbers_in_current_line);

        line_index += 1;
    }

    for gear in gears {
        let adjacent_grid_numbers = gear.find_adjacent_grid_numbers(line_index, &grid_numbers);

        if adjacent_grid_numbers.len() == 2 {
            accumulated_gear_ratios += adjacent_grid_numbers
                .iter()
                .map(|grid_number| grid_number.number)
                .product::<i32>()
        }
    }

    accumulated_gear_ratios
}

#[derive(Debug)]
struct Gear {
    row: usize,
    column: usize,
}

impl Gear {
    fn find_adjacent_grid_numbers(
        &self,
        lines_len: usize,
        grid_numbers: &Vec<GridNumber>,
    ) -> Vec<GridNumber> {
        let mut adjacent_grid_numbers: Vec<GridNumber> = Vec::new();

        for grid_number in grid_numbers {
            let possible_coords = grid_number.get_possible_coords();

            let is_start = self.row == 0;
            let is_end = self.row == lines_len;
            
            if !is_start && possible_coords.contains(&(self.row - 1, self.column - 1)) // NW
                || (!is_start && (possible_coords.contains(&(self.row - 1, self.column)))) // N
                || (!is_start && possible_coords.contains(&(self.row - 1, self.column + 1))) // NE
                || possible_coords.contains(&(self.row, self.column + 1)) // E
                || (!is_end && possible_coords.contains(&(self.row + 1, self.column + 1))) // SE
                || (!is_end && possible_coords.contains(&(self.row + 1, self.column))) // S
                || (!is_end && possible_coords.contains(&(self.row + 1, self.column - 1))) // SW
                || possible_coords.contains(&(self.row, self.column - 1)) // W
            {
                adjacent_grid_numbers.push(*grid_number);
            }
        }

        adjacent_grid_numbers
    }
}

#[derive(Debug, Clone, Copy)]
struct GridNumber {
    row: usize,
    column_start: usize,
    column_end: usize,
    number: i32,
}

impl GridNumber {
    fn get_possible_coords(self) -> Vec<(usize, usize)> {
        let mut possible_coords: Vec<(usize, usize)> = Vec::new();
        for i in self.column_start..self.column_end + 1 {
            possible_coords.push((self.row, i));
        }
        possible_coords
    }
}

fn find_gears_in_line(row: usize, line: &str) -> Vec<Gear> {
    let mut gears: Vec<Gear> = Vec::new();

    for (i, char) in line.char_indices() {
        if char == '*' {
            gears.push(Gear { row, column: i });
        }
    }

    gears
}

#[cfg(test)]
mod tests_c2 {
    use crate::{sum_all_gear_ratios_c2, GridNumber};

    #[test]
    fn returns_possible_coords() {
        let grid_number = GridNumber {
            row: 0,
            column_start: 0,
            column_end: 1,
            number: 10,
        };

        let result = grid_number.get_possible_coords();

        assert!(result.contains(&(0, 0)));
        assert!(result.contains(&(0, 1)));
    }

    #[test]
    fn add_200() {
        let input = "10*10..5.\n\
                     .......*.\n\
                     100..20..\n\
                     ..*......\n\
                     .2.......";

        let result = sum_all_gear_ratios_c2(input);

        assert_eq!(result, 400);
    }
}

fn sum_all_part_numbers_c1(input: &str) -> i32 {
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
        let numbers_in_line = find_numbers_in_line(0, current_line);

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

fn has_adjacent_symbol(current_line: &str, other_line: &str, string_number: &GridNumber) -> bool {
    let chars_current = current_line.chars().collect::<Vec<char>>();
    let chars_other = other_line.chars().collect::<Vec<char>>();
    let line_len = other_line.len();

    let start_index_modifier = match string_number.column_start {
        0 => 0,
        _ => 1,
    };
    let end_index_modifier = match string_number.column_end {
        x if x == line_len => 0,
        _ => 2,
    };

    if start_index_modifier != 0
        && SYMBOLS.contains(&chars_current[string_number.column_start - start_index_modifier])
    {
        return true;
    }

    if end_index_modifier != 0
        && SYMBOLS.contains(&chars_current[string_number.column_end + end_index_modifier - 1])
    {
        return true;
    }

    let chars_slice = &chars_other[string_number.column_start - start_index_modifier
        ..string_number.column_end + end_index_modifier];

    for char in chars_slice {
        if SYMBOLS.contains(char) {
            return true;
        }
    }

    false
}

fn find_numbers_in_line(line_index: usize, line_str: &str) -> Vec<GridNumber> {
    let mut string_numbers: Vec<GridNumber> = Vec::new();
    let mut first_index_of_number: Option<usize> = None;
    let mut reading_number = false;

    for (i, char) in line_str.char_indices() {
        let is_line_end = line_str.len() - 1 == i;

        if !reading_number && NUMBERS.contains(&char) {
            reading_number = true;
            first_index_of_number = Some(i);
        } else if reading_number && is_line_end {
            if NUMBERS.contains(&char) {
                string_numbers.push(GridNumber {
                    column_start: first_index_of_number.unwrap(),
                    column_end: i - 1,
                    number: line_str[first_index_of_number.unwrap()..i + 1]
                        .parse::<i32>()
                        .unwrap(),
                    row: line_index,
                });
            } else {
                string_numbers.push(GridNumber {
                    column_start: first_index_of_number.unwrap(),
                    column_end: i - 1,
                    number: line_str[first_index_of_number.unwrap()..i]
                        .parse::<i32>()
                        .unwrap(),
                    row: line_index,
                });
            }
            first_index_of_number = None;
            reading_number = false;
        } else if reading_number && !NUMBERS.contains(&char) {
            string_numbers.push(GridNumber {
                column_start: first_index_of_number.unwrap(),
                column_end: i - 1,
                number: line_str[first_index_of_number.unwrap()..i]
                    .parse::<i32>()
                    .unwrap(),
                row: line_index,
            });
            first_index_of_number = None;
            reading_number = false;
        }
    }

    string_numbers
}

#[cfg(test)]
mod tests_c1 {
    use crate::{find_numbers_in_line, sum_all_part_numbers_c1};

    #[test]
    fn add50() {
        let input = "11....12\n\
                     ..$..$..\n\
                     13....14";

        let result = sum_all_part_numbers_c1(input);

        assert_eq!(result, 50);
    }

    #[test]
    fn add28() {
        let input = "........\n\
                     .24$-4..\n\
                     ......*.";

        let result = sum_all_part_numbers_c1(input);

        assert_eq!(result, 28);
    }

    #[test]
    fn add4() {
        let input = "$......$\n\
                     .1....1.\n\
                     .1....1.\n\
                     $......$";

        let result = sum_all_part_numbers_c1(input);

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

        let result = sum_all_part_numbers_c1(input);

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

        let result = sum_all_part_numbers_c1(&input);

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

        let result = sum_all_part_numbers_c1(input);

        assert_eq!(result, 4361);
    }

    #[test]
    fn finds_numbers_in_line() {
        let number1 = 35;
        let number2 = 633;
        let input = format!("..{}..{}.", number1, number2);

        let result = find_numbers_in_line(1, &input);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].column_start, 2);
        assert_eq!(result[0].column_end, 3);
        assert_eq!(result[0].number, number1);
        assert_eq!(result[1].column_start, 6);
        assert_eq!(result[1].column_end, 8);
        assert_eq!(result[1].number, number2);
    }

    #[test]
    fn finds_numbers_in_line2() {
        let number1 = 11;
        let number2 = 12;
        let input = format!("{}..{}", number1, number2);

        let result = find_numbers_in_line(1, &input);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].number, number1);
        assert_eq!(result[1].number, number2);
    }
}
