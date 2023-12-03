#[cfg(test)]
mod tests {
  use std::cmp::max;
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
  fn test_max_quantities() {
    let (_, quantities) = parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    let green: Vec<_> = quantities.iter()
      .filter(|(_count, name)| { name.eq(&"green") }).collect();
    assert_eq!(2, green.len());
    let (red, green, blue) = max_colors(&quantities);
    assert_eq!(4, red);
    assert_eq!(2, green);
    assert_eq!(6, blue);
  }

  fn max_colors(quantities: &Vec<(u16, &str)>) -> (u16, u16, u16) {
    let red = max_color(&quantities, "red");
    let green = max_color(&quantities, "green");
    let blue = max_color(&quantities, "blue");
    (red, green, blue)
  }

  fn max_color(quantities: &Vec<(u16, &str)>, color: &str) -> u16 {
    quantities.iter()
      .filter(|(_count, name)| { name.eq(&color) })
      .fold(0, |acc, num| max(acc, num.0))
  }

  #[test]
  fn it_solves_part1() {
    let input = include_str!("day02.input");

    let lines: Vec<_> = input.lines()
      .map(parse_line)
      .map(|(index, quantities)| { (index, max_colors(&quantities)) })
      .collect();

    assert_eq!(100, lines.len());

    let possible: Vec<_> = lines.iter()
      .filter(|(index, (red, green, blue))|
        { red <= &12u16 && green <= &13u16 && blue <= &14u16 }).collect();

    assert_eq!(51, possible.len());

    let sum: u16 = possible.iter()
      .map(|(index, _)| index)
      .sum();

    assert_eq!(2593, sum);
  }
}