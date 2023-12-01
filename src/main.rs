mod trebuchet;

fn main() {
    let input = include_str!("../datafiles/01.txt");
    println!("Day 1 Part 1: {}", trebuchet::trebuchet(input));

    let input = include_str!("../datafiles/01.2.txt");
    println!("Day 1 Part 2: {}", trebuchet::trebuchet_words(input));
}
