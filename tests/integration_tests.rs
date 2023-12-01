use aoc::trebuchet;

#[test]
fn trebuchet_test() {
    // Use a multiline string as input
    let input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
    "#;

    assert_eq!(trebuchet::trebuchet(input), 142);
}

#[test]
fn trebuchet_words_test() {
    // Use a multiline string as input
    let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    assert_eq!(trebuchet::trebuchet_words(input), 281);
}
