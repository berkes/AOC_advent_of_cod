mod trebuchet;

fn main() {
    // open datafiles/01.txt read contents into a string
    let input = include_str!("../datafiles/01.txt");
    println!("Day 1 Part 1: {}", trebuchet::trebuchet(input));
}
