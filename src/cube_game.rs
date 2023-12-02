struct Game {
    reveals: Vec<Reveal>,
    id: u32,
}

impl Game {
    fn is_possible(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        self.reveals
            .iter()
            .all(|reveal| reveal.is_possible(max_red, max_green, max_blue))
    }

    fn fewest_colors(&self) -> Reveal {
        let mut max = Reveal {
            red: 0,
            green: 0,
            blue: 0,
        };
        self.reveals.iter().for_each(|reveal| {
            if reveal.red > max.red {
                max.red = reveal.red;
            }
            if reveal.green > max.green {
                max.green = reveal.green;
            }
            if reveal.blue > max.blue {
                max.blue = reveal.blue;
            }
        });

        max
    }
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let mut parts = line.split(':');
        let id = parts
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let reveals = parts
            .next()
            .unwrap()
            .split(';')
            .map(Reveal::from)
            .collect::<Vec<Reveal>>();

        Game { id, reveals }
    }
}

struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

impl Reveal {
    fn is_possible(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        self.red <= max_red && self.green <= max_green && self.blue <= max_blue
    }
}

impl From<&str> for Reveal {
    fn from(line: &str) -> Self {
        let parts = line.split(',');

        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        parts.for_each(|part| {
            let part = part.trim();

            if part.contains("red") {
                red = part
                    .split(' ')
                    .next()
                    .unwrap_or("0")
                    .parse::<u32>()
                    .unwrap_or(0);
            }
            if part.contains("green") {
                green = part
                    .split(' ')
                    .next()
                    .unwrap_or("0")
                    .parse::<u32>()
                    .unwrap_or(0);
            }
            if part.contains("blue") {
                blue = part
                    .split(' ')
                    .next()
                    .unwrap_or("0")
                    .parse::<u32>()
                    .unwrap_or(0);
            }
        });

        Self { red, green, blue }
    }
}

pub fn cube_game(input: &str) -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let games: Vec<Game> = input.lines().map(Game::from).collect();

    let possible_games = games
        .iter()
        .filter(|game| game.is_possible(MAX_RED, MAX_GREEN, MAX_BLUE))
        .collect::<Vec<&Game>>();

    possible_games.iter().map(|game| game.id).sum()
}

pub fn fewest_power(input: &str) -> u32 {
    let games: Vec<Game> = input.lines().map(Game::from).collect();
    let fewest_colors: Vec<Reveal> = games.iter().map(|game| game.fewest_colors()).collect();

    fewest_colors
        .iter()
        .map(|reveal| reveal.red * reveal.green * reveal.blue)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_from_line() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::from(line);

        assert_eq!(game.id, 1);
        assert_eq!(game.reveals.len(), 3);
    }

    #[test]
    fn reveal_from_line() {
        let line = "1 red, 2 green, 6 blue";
        let reveal = Reveal::from(line);

        assert_eq!(reveal.red, 1);
        assert_eq!(reveal.green, 2);
        assert_eq!(reveal.blue, 6);
    }

    #[test]
    fn reveal_from_partial_line() {
        let line = "1 red, 2 green";
        let reveal = Reveal::from(line);

        assert_eq!(reveal.red, 1);
        assert_eq!(reveal.green, 2);
        assert_eq!(reveal.blue, 0);
    }

    #[test]
    fn game_fewest_colors() {
        let game = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let reveal = game.fewest_colors();
        assert_eq!(reveal.red, 4);
        assert_eq!(reveal.green, 2);
        assert_eq!(reveal.blue, 6);
    }
}
