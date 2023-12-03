#[cfg(test)]
mod tests {
  use nom::branch::alt;
  use nom::bytes::complete::tag;
  use nom::character::complete::{one_of, u16};
  use nom::error::Error;
  use nom::multi::separated_list1;
  use nom::sequence::tuple;

  #[test]
  fn test_parse_digit_color() {
    let color = alt((tag("blue"), tag("red"), tag("green")));
    let mut quantity = tuple((tag(" "), u16::<_, Error<_>>, tag(" "), color));
    let (_rest, (_, count, _, color)) = quantity(" 1 blue").unwrap();
    assert_eq!(1, count);
    assert_eq!("blue", color);
    let (_rest, list) = separated_list1(one_of(",;"), quantity)(" 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
    assert_eq!(6, list.len());
  }

  #[test]
  fn test_parse_prefix() {
    let mut prefix = tuple((tag("Game "), u16::<_, Error<_>>, tag(":")));
    let (_rest, (_, index, _)) = prefix("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
    assert_eq!(1, index)
  }

  fn parse_line(str: &str) -> (u16, Vec<(u16, &str)>) {
    let prefix = tuple((tag("Game "), u16::<_, Error<_>>, tag(":")));
    let color = alt((tag("blue"), tag("red"), tag("green")));
    let quantity = tuple((tag(" "), u16::<_, Error<_>>, tag(" "), color));
    let quantities = separated_list1(one_of(",;"), quantity);
    let (_, ((_, index, _), quantities)) = tuple((prefix, quantities))(str).unwrap();
    let quantities: Vec<_> = quantities.into_iter().map(|(_, count, _, color)| (count, color)).collect();
    return (index, quantities);
  }

  #[test]
  fn test_parse_line() {
    let prefix = tuple((tag("Game "), u16::<_, Error<_>>, tag(":")));
    let color = alt((tag("blue"), tag("red"), tag("green")));
    let quantity = tuple((tag(" "), u16::<_, Error<_>>, tag(" "), color));
    let quantities = separated_list1(one_of(",;"), quantity);
    let (_rest, ((_, index, _), quantities)) = tuple((prefix, quantities))("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
    assert_eq!(1, index);
    assert_eq!(6, quantities.len())
  }

  #[test]
  fn it_solves_part1() {
    let input = include_str!("day02.input");

    let lines: Vec<_> = input.lines()
      .map(parse_line)
      .collect();

    assert_eq!(100, lines.len());
  }
}