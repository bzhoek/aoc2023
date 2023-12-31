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

  fn parse_card(input: &str) -> (u16, Vec<u16>, Vec<u16>) {
    let (input, (_, _, index, _, _)) =
      tuple((tag("Card"), multispace1, complete::u16::<_, Error<_>>, tag(":"), multispace1))
        (input)
        .unwrap();
    let (_rest, (numbers, winning)) =
      separated_pair(
        separated_list1(multispace1, complete::u16::<_, Error<_>>),
        tuple((tag(" |"), multispace1)),
        separated_list1(multispace1, complete::u16::<_, Error<_>>))
        (input).unwrap();
    (index, numbers, winning)
  }

  #[test]
  fn test_parse_line() {
    let (index, numbers, winning) =
      parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
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
  fn it_solves_sample1() {
    let input = include_str!("day04.sample");
    let cards = parse_input(input);
    assert_eq!(6, cards.len());
    let winnings: Vec<_> = cards.iter().map(|(_index, numbers, winning)| {
      let numbers = numbers.iter().collect::<HashSet<_>>();
      let winning = winning.iter().collect::<HashSet<_>>();
      let overlap: Vec<_> = numbers.intersection(&winning).collect();
      if overlap.is_empty() { 0 } else if overlap.len() == 1 { 1 } else { 2_u32.pow((overlap.len() - 1) as u32) }
    }).collect();
    let sum: u32 = winnings.iter().sum();
    assert_eq!(13, sum);
  }

  fn parse_input(input: &str) -> Vec<(u16, Vec<u16>, Vec<u16>)> {
    let lines: Vec<_> = input
      .lines()
      .map(parse_card)
      .collect();
    lines
  }

  #[test]
  fn it_solves_part1() {
    let input = include_str!("day04.input");
    let cards = parse_input(input);
    assert_eq!(199, cards.len());
    let winnings: Vec<_> = cards.iter().map(|(_index, numbers, winning)| {
      let numbers = numbers.iter().collect::<HashSet<_>>();
      let winning = winning.iter().collect::<HashSet<_>>();
      let overlap: Vec<_> = numbers.intersection(&winning).collect();
      if overlap.is_empty() { 0 } else if overlap.len() == 1 { 1 } else { 2_u32.pow((overlap.len() - 1) as u32) }
    }).collect();
    let sum: u32 = winnings.iter().sum();
    assert_eq!(23847, sum);
  }
}