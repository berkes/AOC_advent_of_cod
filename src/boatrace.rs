struct RaceDoc {
    races: Vec<Race>,
}

impl From<&str> for RaceDoc {
    fn from(input: &str) -> Self {
        // Find the line with Time: remove the time, split on space, parse each as usize
        let times = input
            .lines()
            .find(|line| line.starts_with("Time:"))
            .unwrap()
            .trim_start_matches("Time:")
            .split_whitespace()
            .map(|time| time.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let distances = input
            .lines()
            .find(|line| line.starts_with("Distance:"))
            .unwrap()
            .trim_start_matches("Distance:")
            .split_whitespace()
            .map(|distance| distance.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let races = times
            .iter()
            .zip(distances.iter())
            .map(|(time, distance)| Race::new(*time, *distance))
            .collect::<Vec<Race>>();

        // Get line with Distance:, split on space, parse each as usize
        Self { races }
    }
}
impl RaceDoc {
    fn race_button_holds(&self) -> Vec<std::ops::RangeInclusive<usize>> {
        self.races.iter().map(|race| race.hold_options()).collect()
    }
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn new(time: usize, distance: usize) -> Self {
        Self { time, distance }
    }

    fn hold_options(&self) -> std::ops::RangeInclusive<usize> {
        let options = (0..self.time)
            .filter(|button_pressed| {
                let speed = button_pressed;
                let time_left_to_cover_distance = self.time - button_pressed;
                let distance_covered = speed * time_left_to_cover_distance;

                distance_covered > self.distance
            })
            .collect::<Vec<_>>();

        options.iter().min().unwrap().clone()..=options.iter().max().unwrap().clone()
    }
}

pub fn possible_wins(input: &str) -> usize {
    let race_doc = RaceDoc::from(input);

    race_doc
        .race_button_holds()
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hold_options_test() {
        let times = &[7, 15, 30];
        let distances = &[9, 40, 200];


        let options = times
            .iter()
            .zip(distances.iter())
            .map(|(time, distance)| Race::new(*time, *distance).hold_options())
            .collect::<Vec<_>>();

        assert_eq!(options, vec![2..=5, 4..=11, 11..=19]);
    }
}
