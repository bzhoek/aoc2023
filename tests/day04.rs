#[cfg(test)]
mod tests {
  use std::collections::HashSet;
  use nom::bytes::complete::tag;
  use nom::character::complete;
  use nom::character::complete::multispace1;
  use nom::error::Error;
  use nom::multi::separated_list1;
  use nom::sequence::{separated_pair, tuple};

  #[test]
  fn test_parse_card() {
    let (input, (_, index, _)) = tuple((tag("Card "), complete::u16::<_, Error<_>>, tag(": ")))
      ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
      .unwrap();
    assert_eq!(1, index);
    let (_rest, (numbers, winning)) =
      separated_pair(
        separated_list1(multispace1, complete::u16::<_, Error<_>>),
        tag(" | "),
        separated_list1(multispace1, complete::u16::<_, Error<_>>))
        (input).unwrap();
    assert_eq!(5, numbers.len());
    assert_eq!(vec!(41, 48, 83, 86, 17), numbers);
    assert_eq!(8, winning.len());
    assert_eq!(vec!(83, 86, 6, 31, 17, 9, 48, 53), winning);
  }

  fn parse_line(input: &str) -> (u16, Vec<u16>, Vec<u16>) {
    println!("{}", input);
    let (input, (_, index, _, _)) =
      tuple((tag("Card "), complete::u16::<_, Error<_>>, tag(":"), multispace1))
        (input)
        .unwrap();
    let (_rest, (numbers, winning)) =
      separated_pair(
        separated_list1(multispace1, complete::u16::<_, Error<_>>),
        tag(" | "),
        separated_list1(multispace1, complete::u16::<_, Error<_>>))
        (input).unwrap();
    (index, numbers, winning)
  }

  #[test]
  fn test_parse_line() {
    let (index, numbers, winning) =
      parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
    assert_eq!(1, index);
    assert_eq!(5, numbers.len());
    assert_eq!(vec!(41, 48, 83, 86, 17), numbers);
    assert_eq!(8, winning.len());
    assert_eq!(vec!(83, 86, 6, 31, 17, 9, 48, 53), winning);
    let numbers = numbers.into_iter().collect::<HashSet<_>>();
    let winning = winning.into_iter().collect::<HashSet<_>>();
    let mut overlap: Vec<_> = numbers.intersection(&winning).collect();
    overlap.sort();
    assert_eq!(vec!(&17, &48, &83, &86), overlap);
    let score = if overlap.len() == 1 { 1 } else { 2_i32.pow((overlap.len() - 1) as u32) };
    assert_eq!(8, score);
  }

  #[test]
  fn it_solves_sample() {
    let input = include_str!("day04.sample");
    let lines: Vec<_> = input
      .lines()
      .map(parse_line)
      .collect();
    assert_eq!(6, lines.len());
    let winnings: Vec<_> = lines.iter().map(|(_index, numbers, winning)| {
      let numbers = numbers.into_iter().collect::<HashSet<_>>();
      let winning = winning.into_iter().collect::<HashSet<_>>();
      let mut overlap: Vec<_> = numbers.intersection(&winning).collect();
      overlap.sort();
      let score = if overlap.len() == 0 { 0 } else if overlap.len() == 1 { 1 } else { 2_u32.pow((overlap.len() - 1) as u32) };
      score
    }).collect();
    let sum: u32 = winnings.iter().sum();
    assert_eq!(13, sum);
  }
}