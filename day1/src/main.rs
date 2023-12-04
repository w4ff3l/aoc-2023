use std::fs::read_to_string;

fn main() {
    println!("c1");
    println!("{}", day1_c1());
    println!("c2");
    println!("{}", day1_c2());
}

fn day1_c1() -> i32 {
    read_to_string("puzzle-input-c1")
        .unwrap()
        .lines()
        .map(String::from)
        .map(parse_number_c1)
        .sum()
}

fn day1_c2() -> i32 {
    read_to_string("puzzle-input-c2")
        .unwrap()
        .lines()
        .map(String::from)
        .map(parse_number_c2)
        .sum()
}

fn parse_number_c1(str: String) -> i32 {
    let index_first_number = str.find(|c: char| c.is_ascii_digit()).unwrap();
    let index_last_number = str.rfind(|c: char| c.is_ascii_digit()).unwrap();

    let number_str = format!(
        "{}{}",
        str.chars().nth(index_first_number).unwrap(),
        str.chars().nth(index_last_number).unwrap()
    );

    number_str.parse::<i32>().unwrap()
}

fn parse_number_c2(str: String) -> i32 {
    let number_vec = vec![
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

    let mut first_number = "";
    let mut matched = false;
    let mut index = 0;

    while !matched {
        let sub = &str[0..index];

        for num in &number_vec {
            if sub.contains(num) {
                first_number = &num;
                matched = true;
            }
        }

        index += 1;
    }

    let mut last_number = "";
    matched = false;
    index = str.len();

    while !matched {
        let sub = &str[index..str.len()];

        for num in &number_vec {
            if sub.contains(num) {
                last_number = &num;
                matched = true;
            }
        }

        if index > 0 {
            index -= 1;
        } else {
            matched = true;
        }
    }

    if first_number.len() > 1 {
        let index = number_vec.iter().position(|x| x == first_number).unwrap();
        first_number = &number_vec[index + 9];
    }

    if last_number.len() > 1 {
        let index = number_vec.iter().position(|x| x == last_number).unwrap();
        last_number = &number_vec[index + 9];
    }

    let number_str = format!("{}{}", first_number, last_number);

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
    let test_string_1 = "two1nine".to_string();
    let test_string_2 = "eightwothree".to_string();
    let test_string_3 = "abcone2threexyz".to_string();
    let test_string_4 = "xtwone3four".to_string();
    let test_string_5 = "4nineeightseven2".to_string();
    let test_string_6 = "zoneight234".to_string();
    let test_string_7 = "7pqrstsixteen".to_string();

    let number_1 = parse_number_c2(test_string_1);
    let number_2 = parse_number_c2(test_string_2);
    let number_3 = parse_number_c2(test_string_3);
    let number_4 = parse_number_c2(test_string_4);
    let number_5 = parse_number_c2(test_string_5);
    let number_6 = parse_number_c2(test_string_6);
    let number_7 = parse_number_c2(test_string_7);

    assert_eq!(number_1, 29);
    assert_eq!(number_2, 83);
    assert_eq!(number_3, 13);
    assert_eq!(number_4, 24);
    assert_eq!(number_5, 42);
    assert_eq!(number_6, 14);
    assert_eq!(number_7, 76);

    assert_eq!(number_1 + number_2 + number_3 + number_4 + number_5 + number_6 + number_7, 281);
}

#[test]
fn finds_numbers_in_string_c2_custom() {
    let test_string = "nine2rsotai98twosevenfive".to_string();
    let test_string_1 = "two1nine".to_string();
    let test_string_2 = "eightwothree".to_string();
    let test_string_3 = "abcone2threexyz".to_string();
    let test_string_4 = "xtwone3four".to_string();
    let test_string_5 = "4nineeightseven2".to_string();
    let test_string_6 = "zoneight234".to_string();
    let test_string_7 = "7pqrstsixteen".to_string();
    let test_string_8 = "eightwothree".to_string();
    let test_string_9 = "fourfourfour".to_string();
    let test_string_10 = "mbvxvl2".to_string();

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
    let number_10 = parse_number_c2(test_string_10);

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
    assert_eq!(number_10, 2);
}
