use std::ops::RangeInclusive;

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        let seeds = input
            .lines()
            .find(|line| line.starts_with("seeds:"))
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|seed| seed.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let maps = input
            .split("\n\n")
            .skip(1)
            .map(|block| {
                let rows = block
                    .lines()
                    .skip(1) // skip the first line which is the map name
                    .map(MapRow::from)
                    .collect::<Vec<MapRow>>();
                Map { rows }
            })
            .collect::<Vec<Map>>();

        Almanac { seeds, maps }
    }
}

impl Almanac {
    fn walk(&self, seed: usize) -> usize {
        let mut current = seed;
        for map in &self.maps {
            current = map.corresponding(current);
        }
        current
    }

    fn seeds_ranged(&self) -> Vec<usize> {
        self.seeds
            .chunks(2)
            .map(|pair| (pair[0]..pair[0] + pair[1]))
            .flat_map(|range| range.collect::<Vec<usize>>())
            .collect()
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    rows: Vec<MapRow>,
}

impl Map {
    fn corresponding(&self, source: usize) -> usize {
        match self.rows.iter().find(|row| row.source.contains(&source)) {
            Some(row) => {
                let index = source - row.source.start();
                row.destination.start() + index
            }
            None => source,
        }
    }
}

#[derive(Debug, PartialEq)]
struct MapRow {
    source: RangeInclusive<usize>,
    destination: RangeInclusive<usize>,
}

impl From<&str> for MapRow {
    fn from(input: &str) -> Self {
        let mut parts = input.split_whitespace();

        let destination = parts.next().unwrap().parse::<usize>().unwrap();
        let source = parts.next().unwrap().parse::<usize>().unwrap();
        let length = parts.next().unwrap().parse::<usize>().unwrap();

        if length == 0 {
            MapRow {
                source: (0..=0),
                destination: (0..=0),
            }
        } else {
            MapRow {
                source: (source..=source + length - 1),
                destination: (destination..=destination + length - 1),
            }
        }
    }
}

pub fn lowest(input: &str) -> usize {
    let almanac = Almanac::from(input);
    let outputs = almanac
        .seeds
        .iter()
        .map(|seed| almanac.walk(*seed))
        .collect::<Vec<usize>>();
    *outputs.iter().min().unwrap()
}

pub fn ranged(input: &str) -> usize {
    let almanac = Almanac::from(input);
    let seeds_ranged = almanac.seeds_ranged();
    println!("total seeds: {}", seeds_ranged.len());
    let unique_seeds = seeds_ranged
        .into_iter()
        .collect::<std::collections::HashSet<usize>>()
        .into_iter()
        .collect::<Vec<usize>>();
    println!("unique seeds: {}", unique_seeds.len());

    48
    // let outputs = almanac
    //     .seeds_ranged()
    //     .iter()
    //     .map(|seed| almanac.walk(*seed))
    //     .collect::<Vec<usize>>();
    // *outputs.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeds_test() {
        let input = r#"seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48"#;

        assert_eq!(Almanac::from(input).seeds, vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_seeds_ranged() {
        let input = r#"seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48"#;

        assert_eq!(Almanac::from(input).seeds_ranged().len(), 27);
    }

    #[test]
    fn maps_test() {
        let input = r#"seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        1 2 1
        1 2 1
        1 2 1"#;

        let maps = Almanac::from(input).maps;
        assert_eq!(maps.len(), 3);
        let expected = Map {
            rows: vec![
                MapRow {
                    source: (98..=99),
                    destination: (50..=51),
                },
                MapRow {
                    source: (50..=97),
                    destination: (52..=99),
                },
            ],
        };
        assert_eq!(maps[0], expected);
    }

    #[test]
    fn maps_overflow_test() {
        let input = r#"seeds: 79 14 55 13

        seed-to-soil map:
        50 98 0"#;

        assert_eq!(
            Almanac::from(input).maps[0].rows[0],
            MapRow {
                source: (0..=0),
                destination: (0..=0)
            }
        );
    }

    #[test]
    fn map_row_from_str_test() {
        let input = "50 98 2";
        let expected = MapRow {
            source: (98..=99),
            destination: (50..=51),
        };
        assert_eq!(MapRow::from(input), expected);
    }

    #[test]
    fn test_map_included() {
        let map = Map {
            rows: vec![
                MapRow {
                    source: (98..=99),
                    destination: (50..=51),
                },
                MapRow {
                    source: (50..=97),
                    destination: (52..=99),
                },
            ],
        };

        assert_eq!(map.corresponding(98), 50);
        assert_eq!(map.corresponding(99), 51);
    }

    #[test]
    fn test_map_not_included() {
        let map = Map {
            rows: vec![
                MapRow {
                    source: (98..=99),
                    destination: (50..=51),
                },
                MapRow {
                    source: (50..=97),
                    destination: (52..=99),
                },
            ],
        };

        assert_eq!(map.corresponding(10), 10);
        assert_eq!(map.corresponding(0), 0);
    }

    #[test]
    fn test_almanac_walk() {
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

        let almanac = Almanac::from(input);
        assert_eq!(almanac.walk(79), 82);
        assert_eq!(almanac.walk(14), 43);
        assert_eq!(almanac.walk(55), 86);
    }
}
