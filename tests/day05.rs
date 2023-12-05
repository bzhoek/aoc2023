#[cfg(test)]
mod tests {
  use std::ops::Range;
  use nom::bytes::complete::tag;
  use nom::character::complete;
  use nom::character::complete::multispace1;
  use nom::error::Error;
  use nom::multi::separated_list1;
  use nom::sequence::tuple;

  #[test]
  fn test_parse_sample() {
    let input = include_str!("day05.sample");
    let (input, (_, _, seeds)) = tuple((
      tag("seeds:"), multispace1, separated_list1(multispace1, complete::u16::<_, Error<_>>)))
      (input).unwrap();
    let (input, (_, _, _, s2s)) = tuple((
      multispace1, tag("seed-to-soil map:"), multispace1, separated_list1(multispace1, complete::u16::<_, Error<_>>)))
      (input).unwrap();
    let (input, (_, _, _, s2f)) = tuple((
      multispace1, tag("soil-to-fertilizer map:"), multispace1, separated_list1(multispace1, complete::u16::<_, Error<_>>)))
      (input).unwrap();
    let (input, (_, _, _, f2w)) = tuple((
      multispace1, tag("fertilizer-to-water map:"), multispace1, separated_list1(multispace1, complete::u16::<_, Error<_>>)))
      (input).unwrap();
    let (input, (_, _, _, w2l)) = tuple((
      multispace1, tag("water-to-light map:"), multispace1, separated_list1(multispace1, complete::u16::<_, Error<_>>)))
      (input).unwrap();
    let (input, (_, _, _, l2t)) = tuple((
      multispace1, tag("light-to-temperature map:"), multispace1, separated_list1(multispace1, complete::u16::<_, Error<_>>)))
      (input).unwrap();
    let (input, (_, _, _, t2h)) = tuple((
      multispace1, tag("temperature-to-humidity map:"), multispace1, separated_list1(multispace1, complete::u16::<_, Error<_>>)))
      (input).unwrap();
    let (_input, (_, _, _, h2l)) = tuple((
      multispace1, tag("humidity-to-location map:"), multispace1, separated_list1(multispace1, complete::u16::<_, Error<_>>)))
      (input).unwrap();

    assert_eq!(4, seeds.len());
    assert_eq!(6, s2s.len());
    assert_eq!(9, s2f.len());
    assert_eq!(12, f2w.len());
    assert_eq!(6, w2l.len());
    assert_eq!(9, l2t.len());
    assert_eq!(6, t2h.len());
    assert_eq!(6, h2l.len());

    let seed = seeds.first().unwrap();
    let seed_to_soil = map_to_ranges(s2s);
    assert_eq!(vec!((50, 98..100), (52, 50..98)), seed_to_soil);

    assert_eq!(81, map_through(*seed, &seed_to_soil));
    assert_eq!(14, map_through(14u16, &seed_to_soil));
    assert_eq!(57, map_through(55u16, &seed_to_soil));
    assert_eq!(13, map_through(13u16, &seed_to_soil));
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

  fn map_to_ranges(vector: Vec<u16>) -> Vec<(u16, Range<u16>)> {
    let s2s: Vec<_> = vector.chunks(3).map(|chunk| {
      (chunk[0], Range { start: chunk[1], end: chunk[1] + chunk[2] })
    }).collect();
    s2s
  }
}