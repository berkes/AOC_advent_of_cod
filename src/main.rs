mod cube_game;
mod trebuchet;

fn main() {
    let input = include_str!("../datafiles/01.txt");
    println!("Day 1 Part 1: {}", trebuchet::trebuchet(input));

    let input = include_str!("../datafiles/01.2.txt");
    println!("Day 1 Part 2: {}", trebuchet::trebuchet_words(input));

    let input = include_str!("../datafiles/02.txt");
    println!("Day 2 Part 1: {}", cube_game::cube_game(input));
    println!("Day 2 Part 2: {}", cube_game::fewest_power(input));
}
