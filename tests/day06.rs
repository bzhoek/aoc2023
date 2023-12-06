#[cfg(test)]
mod tests {
  use std::ops::Range;

  use nom::bytes::complete::tag;
  use nom::character::complete;
  use nom::character::complete::{multispace0, multispace1};
  use nom::error::Error;
  use nom::multi::separated_list1;
  use nom::sequence::tuple;

  #[test]
  fn it_parses_sample() {
    let input = "\
      Time:      7  15   30\n\
      Distance:  9  40  200\n\
      ";
    let pairs = parse_input(input);
    assert_eq!(vec!((7, 9), (15, 40), (30, 200)), pairs);

    let (duration, record) = pairs.first().unwrap();
    let range = Range { start: 1, end: *duration };
    let result = range.into_iter().map(|hold| (duration - hold) * hold).collect::<Vec<_>>();
    assert_eq!(vec!(6, 10, 12, 12, 10, 6), result);

    let result = result.into_iter().filter(|time| time > record).collect::<Vec<_>>();
    assert_eq!(vec!(10, 12, 12, 10), result);
  }

  #[test]
  fn it_solves_sample() {
    let input = "\
      Time:      7  15   30\n\
      Distance:  9  40  200\n\
      ";
    let pairs = parse_input(input);
    let (duration, record) = pairs.first().unwrap();
    let solutions = find_solutions(duration, record);
    assert_eq!(vec!(10, 12, 12, 10), solutions);

    let solutions = pairs.iter().map(|(duration, record)| find_solutions(duration, record)).collect::<Vec<_>>();
    assert_eq!(&vec!(10, 12, 12, 10), solutions.first().unwrap());

    let solutions = solutions.iter().map(|solution| solution.len()).collect::<Vec<_>>();
    let solution = solutions.iter().product::<usize>();
    assert_eq!(288, solution);
  }

  #[test]
  fn it_solves_sample2() {
    let input = "\
      Time:      71530\n\
      Distance:  940200\n\
      ";
    let pairs = parse_input(input);
    let (duration, record) = pairs.first().unwrap();

    let solutions = find_solutions(duration, record);
    assert_eq!(71503, solutions.len());
  }

  #[test]
  fn it_solves_part2() {
    let input = "\
      Time:        40929790\n\
      Distance:   215106415051100\n\
      ";
    let pairs = parse_input(input);
    let (duration, record) = pairs.first().unwrap();

    let solutions = find_solutions(duration, record);
    assert_eq!(28545089, solutions.len());
  }

  fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let (input, (_, _, durations)) = tuple((
      tag("Time:"), multispace1, separated_list1(multispace1, complete::u64::<_, Error<_>>))
    )(input).unwrap();
    let (_input, (_, _, _, records)) = tuple((
      multispace0, tag("Distance:"), multispace1, separated_list1(multispace1, complete::u64::<_, Error<_>>))
    )(input).unwrap();
    durations.into_iter().zip(records).collect::<Vec<_>>()
  }

  fn find_solutions(duration: &u64, record: &u64) -> Vec<u64> {
    let range = Range { start: 1, end: *duration };
    let result = range.into_iter().map(|hold| (duration - hold) * hold).collect::<Vec<_>>();
    result.into_iter().filter(|time| time > record).collect::<Vec<_>>()
  }
}