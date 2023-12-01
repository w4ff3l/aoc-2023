use std::{collections::HashMap, fs::read_to_string};

fn main() {
    println!("c1");
    println!("{}", day1_c1());
    println!("c2");
    println!("{}", day1_c2());
}

fn day1_c2() -> i32 {
    read_to_string("puzzle-input-c1")
        .unwrap()
        .lines()
        .map(String::from)
        .map(parse_number_c2)
        .sum()
}

fn day1_c1() -> i32 {
    read_to_string("puzzle-input-c1")
        .unwrap()
        .lines()
        .map(String::from)
        .map(parse_number_c1)
        .sum()
}

fn parse_number_c2(str: String) -> i32 {
    let str_nums = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string(),
        "7".to_string(),
        "8".to_string(),
        "9".to_string(),
    ];

    let mut findings: HashMap<usize, usize> = HashMap::new();

    for (i_str_num, str_num) in str_nums.iter().enumerate() {
        match str.find(str_num) {
            Some(i) => findings.insert(i, i_str_num),
            None => None,
        };
    }

    let min_index_in_string = findings.keys().min().unwrap();
    let max_index_in_string = findings.keys().max().unwrap();

    let min_index_in_nums = findings.get(min_index_in_string).unwrap();
    let max_index_in_nums = findings.get(max_index_in_string).unwrap();

    let mut min_string = &str_nums[*min_index_in_nums];
    let mut max_string = &str_nums[*max_index_in_nums];

    if min_string.len() > 1 {
        min_string = &str_nums[*min_index_in_nums + 9];
    }

    if max_string.len() > 1 {
        max_string = &str_nums[*max_index_in_nums + 9];
    }

    let number_str = format!("{}{}", min_string.clone(), max_string.clone());

    number_str.parse::<i32>().unwrap()
}

fn parse_number_c1(str: String) -> i32 {
    let index_first_number = str.find(|c: char| c.is_digit(10)).unwrap();
    let index_last_number = str.rfind(|c: char| c.is_digit(10)).unwrap();

    let number_str = format!(
        "{}{}",
        str.chars().nth(index_first_number).unwrap(),
        str.chars().nth(index_last_number).unwrap()
    );

    number_str.parse::<i32>().unwrap()
}

#[test]
fn finds_numbers_in_string_c1() {
    let test_string = "asw2rati1rat9".to_string();

    let number = parse_number_c1(test_string);

    assert_eq!(number, 29);
}

#[test]
fn finds_numbers_in_string_c2() {
    let test_string = "nine2rsotai98twosevenfive".to_string();
    let test_string_1 = "two1nine".to_string();
    let test_string_2 = "eightwothree".to_string();
    let test_string_3 = "abcone2threexyz".to_string();
    let test_string_4 = "xtwone3four".to_string();
    let test_string_5 = "4nineeightseven2".to_string();
    let test_string_6 = "zoneight234".to_string();
    let test_string_7 = "7pqrstsixteen".to_string();
    let test_string_8 = "eighthree".to_string();
    let test_string_9 = "fourfourfour".to_string();

    let number = parse_number_c2(test_string);
    let number_1 = parse_number_c2(test_string_1);
    let number_2 = parse_number_c2(test_string_2);
    let number_3 = parse_number_c2(test_string_3);
    let number_4 = parse_number_c2(test_string_4);
    let number_5 = parse_number_c2(test_string_5);
    let number_6 = parse_number_c2(test_string_6);
    let number_7 = parse_number_c2(test_string_7);
    let number_8 = parse_number_c2(test_string_8);
    let number_9 = parse_number_c2(test_string_9);

    assert_eq!(number, 95);
    assert_eq!(number_1, 29);
    assert_eq!(number_2, 83);
    assert_eq!(number_3, 13);
    assert_eq!(number_4, 24);
    assert_eq!(number_5, 42);
    assert_eq!(number_6, 14);
    assert_eq!(number_7, 76);
    assert_eq!(number_8, 83);
    assert_eq!(number_9, 44);
}
