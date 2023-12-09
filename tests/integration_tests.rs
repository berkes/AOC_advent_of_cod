use aoc::almanac;
use aoc::boatrace;
use aoc::camel_cards;
use aoc::cube_game;
use aoc::engine_parts;
use aoc::network;
use aoc::oasis;
use aoc::scratch_cards;
use aoc::trebuchet;

#[test]
fn trebuchet_test() {
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
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    assert_eq!(cube_game::cube_game(input), 8);
}

#[test]
fn cube_game_fewest_power_test() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    assert_eq!(cube_game::fewest_power(input), 2286);
}

#[test]
fn engine_parts_test() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    assert_eq!(engine_parts::sum(input), 4361);
}

#[test]
fn engine_parts_gear_ratio_test() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    assert_eq!(engine_parts::gear_ratio(input), 467835);
}

#[test]
fn scratch_cards_test() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    assert_eq!(scratch_cards::scratch_cards(input), 13);
}

#[test]
fn almanac_test() {
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    assert_eq!(almanac::lowest(input), 35);
}

#[test]
fn almanac_ranged_test() {
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    assert_eq!(almanac::ranged(input), 46);
}

#[test]
fn boatrace_test() {
    let input = r#"Time: 7 15 30
Time:      7  15   30
Distance:  9  40  200"#;

    assert_eq!(boatrace::possible_wins(input), 288);
}

#[test]
fn boatrace_kerning_test() {
    let input = r#"Time:      71530
Distance:  940200"#;

    assert_eq!(boatrace::possible_wins(input), 71503);
}

#[test]
fn camel_cards_test() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    assert_eq!(camel_cards::total_winnings(input), 6440);
}

#[test]
fn network_one_pass_test() {
    let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    assert_eq!(network::count_steps(input), 2);
}

#[test]
fn network_repeat_path_test() {
    let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    assert_eq!(network::count_steps(input), 6);
}

#[test]
fn oasis_extrapolations_sum_test() {
    let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    assert_eq!(oasis::extrapolations_sum(input), 114);
}
