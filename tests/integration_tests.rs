use aoc::cube_game;
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

#[test]
fn cube_game_test() {
    // Use a multiline string as input
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    assert_eq!(cube_game::cube_game(input), 8);
}

#[test]
fn cube_game_fewest_power_test() {
    // Use a multiline string as input
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    assert_eq!(cube_game::fewest_power(input), 2286);
}
