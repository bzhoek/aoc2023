#[cfg(test)]
mod tests {
  use std::ops::Range;

  use nom::bytes::complete::tag;
  use nom::character::complete;
  use nom::character::complete::multispace1;
  use nom::combinator::map;
  use nom::error::Error;
  use nom::multi::separated_list1;
  use nom::sequence::tuple;
  use rayon::iter::IntoParallelIterator;
  use rayon::iter::ParallelIterator;

  #[test]
  fn test_solve_sample1() {
    let input = include_str!("day05.sample");
    let (input, (_, _, seeds)) = tuple((
      tag("seeds:"), multispace1, separated_list1(multispace1, complete::u64::<_, Error<_>>))
    )(input).unwrap();
    let (input, seed_to_soil) = parse_ranges(input, "seed-to-soil map:");
    let (input, soil_to_fertilizer) = parse_ranges(input, "soil-to-fertilizer map:");
    let (input, fertilizer_to_water) = parse_ranges(input, "fertilizer-to-water map:");
    let (input, water_to_light) = parse_ranges(input, "water-to-light map:");
    let (input, light_to_temperature) = parse_ranges(input, "light-to-temperature map:");
    let (input, temperature_to_humidity) = parse_ranges(input, "temperature-to-humidity map:");
    let (_, humidity_to_location) = parse_ranges(input, "humidity-to-location map:");

    assert_eq!(4, seeds.len());
    assert_eq!(2, seed_to_soil.len());
    assert_eq!(vec!((50, 98..100), (52, 50..98)), seed_to_soil);

    assert_eq!(3, soil_to_fertilizer.len());
    assert_eq!(4, fertilizer_to_water.len());
    assert_eq!(2, water_to_light.len());
    assert_eq!(3, light_to_temperature.len());
    assert_eq!(2, temperature_to_humidity.len());
    assert_eq!(2, humidity_to_location.len());

    let seed = seeds.first().unwrap();
    assert_eq!(81, map_through(*seed, &seed_to_soil));
    assert_eq!(14, map_through(14, &seed_to_soil));
    assert_eq!(57, map_through(55, &seed_to_soil));
    assert_eq!(13, map_through(13, &seed_to_soil));

    let locations: Vec<_> = seeds.iter()
      .map(|seed| map_through(*seed, &seed_to_soil))
      .map(|seed| map_through(seed, &soil_to_fertilizer))
      .map(|seed| map_through(seed, &fertilizer_to_water))
      .map(|seed| map_through(seed, &water_to_light))
      .map(|seed| map_through(seed, &light_to_temperature))
      .map(|seed| map_through(seed, &temperature_to_humidity))
      .map(|seed| map_through(seed, &humidity_to_location))
      .collect();

    let result = locations.iter().min().copied();
    assert_eq!(Some(35), result);
  }

  #[test]
  fn test_solve_sample2() {
    let input = include_str!("day05.sample");
    let (seeds, maps) = parse_input(input);
    let seeds: Vec<_> = seeds.chunks(2).map(|chunk| Range { start: chunk[0], end: chunk[0] + chunk[1] }).collect();
    assert_eq!(2, seeds.len());

    let result = seeds
      .iter()
      .flat_map(|range| {
        range.clone()
          .map(|seed| maps.iter().fold(seed, |seed, map| map_through(seed, map)))
          .min()
      }).min();
    assert_eq!(Some(46), result);
  }

  #[test]
  fn it_solves_part1() {
    let input = include_str!("day05.input");
    let (seeds, maps) = parse_input(input);

    assert_eq!(20, seeds.len());

    let locations: Vec<_> = seeds
      .into_iter()
      .map(|seed| maps.iter().fold(seed, |seed, map| map_through(seed, map)))
      .collect();

    let result = locations.iter().min().copied();
    assert_eq!(Some(111627841), result);
  }

  #[test]
  fn it_solves_part2() {
    let input = include_str!("day05.input");
    let (seeds, maps) = parse_input(input);
    let seeds: Vec<_> = seeds.chunks(2).map(|chunk| Range { start: chunk[0], end: chunk[0] + chunk[1] }).collect();
    assert_eq!(10, seeds.len());

    let seeds = seeds
      .into_par_iter()
      .flat_map(|range| range.clone())
      .collect::<Vec<u64>>();
    assert_eq!(1_945_168_946, seeds.len());

    let result = seeds
      .into_par_iter()
      .map(|seed| maps.iter().fold(seed, |seed, map| map_through(seed, map)))
      .min();
    assert_eq!(Some(69323688), result);
  }

  fn parse_input(input: &str) -> (Vec<u64>, Vec<Vec<(u64, Range<u64>)>>) {
    let (input, (_, _, seeds)) = tuple((
      tag("seeds:"), multispace1, separated_list1(multispace1, complete::u64::<_, Error<_>>))
    )(input).unwrap();
    let (input, seed_to_soil) = parse_ranges(input, "seed-to-soil map:");
    let (input, soil_to_fertilizer) = parse_ranges(input, "soil-to-fertilizer map:");
    let (input, fertilizer_to_water) = parse_ranges(input, "fertilizer-to-water map:");
    let (input, water_to_light) = parse_ranges(input, "water-to-light map:");
    let (input, light_to_temperature) = parse_ranges(input, "light-to-temperature map:");
    let (input, temperature_to_humidity) = parse_ranges(input, "temperature-to-humidity map:");
    let (_, humidity_to_location) = parse_ranges(input, "humidity-to-location map:");

    let maps = vec![seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location];

    (seeds, maps)
  }

  fn parse_ranges<'a>(input: &'a str, heading: &str) -> (&'a str, Vec<(u64, Range<u64>)>) {
    let (input, (_, _, _, vector)) = tuple((
      multispace1,
      tag(heading),
      multispace1,
      map(
        separated_list1(multispace1, complete::u64::<_, Error<_>>), |list| {
          list.chunks(3).map(|chunk| {
            (chunk[0], Range { start: chunk[1], end: chunk[1] + chunk[2] })
          }).collect::<Vec<_>>()
        },
      )
    ))(input).unwrap();
    (input, vector)
  }

  fn map_through(seed: u64, s2s: &[(u64, Range<u64>)]) -> u64 {
    let soil = s2s.iter()
      .find(|(_destination, range)| {
        range.contains(&seed)
      })
      .map(|(destination, range)| {
        destination + seed - range.start
      }).unwrap_or(seed);
    soil
  }
}