mod almanac;
mod boatrace;
mod camel_cards;
mod cube_game;
mod engine_parts;
mod scratch_cards;
mod trebuchet;

fn main() {
    let input = include_str!("../datafiles/01.txt");
    println!("Day 1 Part 1: {}", trebuchet::trebuchet(input));

    let input = include_str!("../datafiles/01.2.txt");
    println!("Day 1 Part 2: {}", trebuchet::trebuchet_words(input));

    let input = include_str!("../datafiles/02.txt");
    println!("Day 2 Part 1: {}", cube_game::cube_game(input));
    println!("Day 2 Part 2: {}", cube_game::fewest_power(input));

    let input = include_str!("../datafiles/03.txt");
    println!("Day 3 Part 1: {}", engine_parts::sum(input));
    println!("Day 3 Part 2: {}", engine_parts::gear_ratio(input));

    let input = include_str!("../datafiles/04.txt");
    println!("Day 4 Part 1: {}", scratch_cards::scratch_cards(input));

    let input = include_str!("../datafiles/05.txt");
    println!("Day 5 Part 1: {}", almanac::lowest(input));
    // Attempt at brute force
    // println!("Day 5 Part 2: {}", almanac::ranged(input));

    let input = include_str!("../datafiles/06.1.txt");
    println!("Day 6 Part 1: {}", boatrace::possible_wins(input));
    let input = include_str!("../datafiles/06.2.txt");
    println!("Day 6 Part 2: {}", boatrace::possible_wins(input));

    let input = include_str!("../datafiles/07.txt");
    println!("Day 7 Part 1: {}", camel_cards::total_winnings(input));
}
