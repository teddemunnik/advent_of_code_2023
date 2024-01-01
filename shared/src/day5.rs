use std::thread::current;

#[derive(PartialEq, Eq, Debug)]
struct MappingRange{
    source_start: usize,
    destination_start: usize,
    count: usize 
}

impl MappingRange{
    fn try_map(&self, source: usize) -> Option<usize>{
        if source >= self.source_start && source < self.source_start + self.count{
            Some(source - self.source_start + self.destination_start)
        }else{
            None
        }
    }
}

struct Mapping{
    ranges: Vec<MappingRange>
}

impl Mapping{
    fn lookup(&self, value: usize) -> usize{
        self.ranges.iter().filter_map(|range| range.try_map(value)).next().unwrap_or(value)
    }
}

struct SeedMappings{
    seeds: Vec<usize>,
    mappings: Vec<Mapping>
}

fn parse_mapping_line(line: &str) -> Option<MappingRange>{
    let mut split = line.split_ascii_whitespace();
    let destination_start = split.next()?.parse::<usize>().ok()?;
    let source_start = split.next()?.parse::<usize>().ok()?;
    let count = split.next()?.parse::<usize>().ok()?;
    Some(MappingRange{
        destination_start,
        source_start,
        count
    })
}

fn parse_seed_mapping<R: std::io::BufRead>(input: R) -> Option<SeedMappings>{
    let mut lines = input.lines();

    // Parse the seeds list on the first line
    let seeds = lines.next()?.ok()?.split(':').nth(1)?.split_ascii_whitespace().map(|number| number.parse::<usize>().ok()).collect::<Option<Vec<usize>>>()?;

    let mut mappings = Vec::new();
    let mut current_mappings= Vec::new();

    while let Some(line) = lines.next(){
        let line = line.ok()?;

        if line.contains(':'){
            if !current_mappings.is_empty(){
                mappings.push(Mapping{ ranges: current_mappings });
                current_mappings = Vec::new();
            }
            continue;
        }

        if let Some(mapping) = parse_mapping_line(&line){
            current_mappings.push(mapping);
        }
    }

    if !current_mappings.is_empty(){
        mappings.push(Mapping{ ranges: current_mappings});
    }

    Some(SeedMappings{
        seeds,
        mappings
    })
}

#[aoc_2023_markup::aoc_task(2023, 5, 1)]
fn lowest_location_with_seed<R: std::io::BufRead>(input: R) -> Option<usize>{
    let mappings = parse_seed_mapping(input)?;

    let locations = mappings.seeds.iter().map(|seed| {
        mappings.mappings.iter().fold(*seed, |a, b| b.lookup(a))
    });

    locations.min()
}

#[cfg(test)]
mod tests{
    use super::*;
    use indoc::indoc;

    const SAMPLE_INPUT: &[u8] = indoc!{"
        seeds: 79 14 55 13

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
        56 93 4
    "}.as_bytes();

    #[test]
    fn test_mapping_lookup(){
        let MAPPING = Mapping{
            ranges: vec![
                MappingRange{ source_start: 98, destination_start: 50, count: 2 },
                MappingRange{ source_start: 50, destination_start: 52, count: 48 },
            ]
        };

        assert_eq!(MAPPING.lookup(98), 50);
        assert_eq!(MAPPING.lookup(99), 51);
        assert_eq!(MAPPING.lookup(53), 55);
        assert_eq!(MAPPING.lookup(10), 10);
    }

    #[test]
    fn test_parse_seed_mappings(){
        let mappings = parse_seed_mapping(SAMPLE_INPUT).unwrap();
        assert_eq!(mappings.seeds, [ 79, 14, 55, 13]);
        assert_eq!(mappings.mappings.len(), 7);
        assert_eq!(mappings.mappings[0].ranges, [
            MappingRange{ destination_start: 50, source_start: 98, count: 2},
            MappingRange{ destination_start: 52, source_start: 50, count: 48},
        ]);
    }

    #[test]
    fn test_lowest_location_with_seeds(){
        let lowest_location = lowest_location_with_seed(SAMPLE_INPUT).unwrap();
        assert_eq!(lowest_location, 35);
    }
}