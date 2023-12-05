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

  #[test]
  fn test_parse_sample() {
    let input = include_str!("day05.sample");
    let (input, (_, _, seeds)) = tuple((
      tag("seeds:"), multispace1, separated_list1(multispace1, complete::u16::<_, Error<_>>))
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
    assert_eq!(14, map_through(14u16, &seed_to_soil));
    assert_eq!(57, map_through(55u16, &seed_to_soil));
    assert_eq!(13, map_through(13u16, &seed_to_soil));

    let locations: Vec<_> = seeds.iter()
      .map(|seed| map_through(*seed, &*seed_to_soil))
      .map(|seed| map_through(seed, &*soil_to_fertilizer))
      .map(|seed| map_through(seed, &*fertilizer_to_water))
      .map(|seed| map_through(seed, &*water_to_light))
      .map(|seed| map_through(seed, &*light_to_temperature))
      .map(|seed| map_through(seed, &*temperature_to_humidity))
      .map(|seed| map_through(seed, &*humidity_to_location))
      .collect();

    let result = locations.iter().min().copied();
    assert_eq!(Some(35), result);
  }

  fn parse_ranges<'a>(input: &'a str, heading: &str) -> (&'a str, Vec<(u16, Range<u16>)>) {
    let (input, (_, _, _, vector)) = tuple((
      multispace1,
      tag(heading),
      multispace1,
      map(
        separated_list1(multispace1, complete::u16::<_, Error<_>>), |list| {
          list.chunks(3).map(|chunk| {
            (chunk[0], Range { start: chunk[1], end: chunk[1] + chunk[2] })
          }).collect::<Vec<_>>()
        },
      )
    ))(input).unwrap();
    (input, vector)
  }

  fn map_through(seed: u16, s2s: &[(u16, Range<u16>)]) -> u16 {
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