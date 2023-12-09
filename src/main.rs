mod almanac;
mod boatrace;
mod camel_cards;
mod cube_game;
mod engine_parts;
mod network;
mod oasis;
mod scratch_cards;
mod trebuchet;

fn main() {
    // read the arguments passed to the program as day and part, both are numbers.
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run <day> <part>");
        return;
    }
    let day = args[1]
        .parse::<usize>()
        .expect("Day must be a number. cargo run 1 1");
    let part = args[2]
        .parse::<usize>()
        .expect("Part must be a number. cargo run 3 1");

    match (day, part) {
        (1, 1) => {
            let input = include_str!("../datafiles/01.txt");
            println!("Day 1 Part 1: {}", trebuchet::trebuchet(input));
        }
        (1, 2) => {
            let input = include_str!("../datafiles/01.2.txt");
            println!("Day 1 Part 2: {}", trebuchet::trebuchet_words(input));
        }
        (2, 1) => {
            let input = include_str!("../datafiles/02.txt");
            println!("Day 2 Part 1: {}", cube_game::cube_game(input));
        }
        (2, 2) => {
            let input = include_str!("../datafiles/02.txt");
            println!("Day 2 Part 2: {}", cube_game::fewest_power(input));
        }
        (3, 1) => {
            let input = include_str!("../datafiles/03.txt");
            println!("Day 3 Part 1: {}", engine_parts::sum(input));
        }
        (3, 2) => {
            let input = include_str!("../datafiles/03.txt");
            println!("Day 3 Part 2: {}", engine_parts::gear_ratio(input));
        }
        (4, 1) => {
            let input = include_str!("../datafiles/04.txt");
            println!("Day 4 Part 1: {}", scratch_cards::scratch_cards(input));
        }
        (4, 2) => {
            todo!()
        }
        (5, 1) => {
            let input = include_str!("../datafiles/05.txt");
            println!("Day 5 Part 1: {}", almanac::lowest(input));
        }
        (5, 2) => {
            let input = include_str!("../datafiles/05.txt");
            // Attempt at brute force DOESNT WORK
            println!("Day 5 Part 2: {}", almanac::ranged(input));
        }
        (6, 1) => {
            let input = include_str!("../datafiles/06.1.txt");
            println!("Day 6 Part 1: {}", boatrace::possible_wins(input));
        }
        (6, 2) => {
            let input = include_str!("../datafiles/06.2.txt");
            println!("Day 6 Part 2: {}", boatrace::possible_wins(input));
        }
        (7, 1) => {
            let input = include_str!("../datafiles/07.txt");
            println!("Day 7 Part 1: {}", camel_cards::total_winnings(input));
        }
        (7, 2) => {
            todo!()
        }
        (8, 1) => {
            let input = include_str!("../datafiles/08.txt");
            println!("Day 8 Part 1: {}", network::count_steps(input));
        }
        (9, 1) => {
            let input = include_str!("../datafiles/09.txt");
            println!("Day 9 Part 1: {}", oasis::extrapolations_sum(input));
        }
        (9, 2) => {
            todo!()
        }
        _ => println!("Day {} Part {} not implemented yet", day, part),
    }
}
