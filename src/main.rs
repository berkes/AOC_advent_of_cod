

fn main() {
    // open datafiles/01.txt read contents into a string
    let input = include_str!("../datafiles/01.txt");
    println!("Part 1: {}", aoc::trebuchet(input));
}
