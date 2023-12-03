use std::{fs::read_to_string, i32};

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
    // Possible symbols are: #, $, &, +, /, =, @, 

    0
}

#[cfg(test)]
mod tests {
    use crate::sum_all_part_numbers;

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
}
