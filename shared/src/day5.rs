use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
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

    fn try_map_range(&self, range: &std::ops::Range<usize>) -> Option<(std::ops::Range<usize>, std::ops::Range<usize>)>{
        let max_start = self.source_start.max(range.start);
        let min_end = (self.source_start + self.count).min(range.end);
        if min_end > max_start{
            let destination_start = (max_start - self.source_start) + self.destination_start;
            let destination_end = destination_start + (min_end - max_start);
            Some((max_start..min_end, destination_start..destination_end))
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

    fn lookup_ranges(&self, ranges: &[std::ops::Range<usize>]) -> Vec<std::ops::Range<usize>>{
        
        let mut test = self.ranges.clone();
        test.sort_by(|a, b| a.source_start.cmp(&b.source_start));

        let mut result = Vec::new();
        for range in ranges.iter(){

            let mut last_end = range.start;

            // Go over all mapped subranges
            for (source_range, destination_range) in test.iter().filter_map(|map| map.try_map_range(range)){
                // If there was any space between the last end and the current start, add an identity range
                if source_range.start > last_end{
                    result.push(last_end..source_range.start);
                }

                last_end = source_range.end;
                result.push(destination_range);
            }

            // Any remainder at the end
            if last_end < range.end{
                result.push(last_end..range.end);
            }
        }

        result
    }
}

struct SeedMappings{
    seeds: Vec<usize>,
    mappings: Vec<Mapping>
}

struct SeedRangeMappings{
    seed_ranges: Vec<std::ops::Range<usize>>,
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

fn parse_mappings(lines: &mut impl Iterator<Item = std::io::Result<String>>) -> Option<Vec<Mapping>>{
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

    Some(mappings)
}

fn parse_seed_mapping<R: std::io::BufRead>(input: R) -> Option<SeedMappings>{
    let mut lines = input.lines();
    let seeds = lines.next()?.ok()?.split(':').nth(1)?.split_ascii_whitespace().map(|number| number.parse::<usize>().ok()).collect::<Option<Vec<usize>>>()?;
    let mappings = parse_mappings(&mut lines)?;
    Some(SeedMappings{
        seeds,
        mappings
    })
}

fn parse_seed_range_mappings<R: std::io::BufRead>(input: R) -> Option<SeedRangeMappings>{
    let mut lines = input.lines();

    let seed_ranges = lines
        .next()?
        .ok()?
        .split(':')
        .nth(1)?
        .split_ascii_whitespace()
        .chunks(2)
        .into_iter()
        .map(|range| {
            let mut range_iter = range;
            let start = range_iter.next()?.parse::<usize>().ok()?;
            let count = range_iter.next()?.parse::<usize>().ok()?;
            Some(start..(start + count))
        })
        .collect::<Option<Vec<std::ops::Range<usize>>>>()?;

    let mappings = parse_mappings(&mut lines)?;
    Some(SeedRangeMappings{
        seed_ranges,
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

#[aoc_2023_markup::aoc_task(2023, 5, 2)]
fn lowest_location_with_seed_ranges<R: std::io::BufRead>(input: R) -> Option<usize>{
    let mappings = parse_seed_range_mappings(input)?;
    let locations : Vec<std::ops::Range<usize>> = mappings.mappings.iter().fold(mappings.seed_ranges, |a, b| b.lookup_ranges(&a));
    locations.iter().map(|range| range.start).min()
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
    fn test_parse_seed_ranges(){
        let mappings = parse_seed_range_mappings(SAMPLE_INPUT).unwrap();
        assert_eq!(mappings.seed_ranges, [
            79..93,
            55..68,
        ]);
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

    #[test]
    fn test_lowest_location_with_seed_ranges(){
        let lowest_location = lowest_location_with_seed_ranges(SAMPLE_INPUT).unwrap();
        assert_eq!(lowest_location, 46);
    }
}