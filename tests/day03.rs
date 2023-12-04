#[cfg(test)]
mod tests {
  use nom::AsChar;
  use nom::bytes::complete::take_till;
  use nom::character::complete::{digit1, u16};
  use nom::error::Error;
  use nom::multi::{many1, separated_list1};
  use nom::sequence::{pair, tuple};

  #[test]
  fn test_parse_digit_color() {
    let input = include_str!("day03.sample");
  }

  #[test]
  fn test_nom_parse_separated() {
    let mut numbers = tuple((
      take_till(AsChar::is_dec_digit),
      separated_list1(take_till(AsChar::is_dec_digit), u16::<_, Error<_>>))
    );
    let (_rest, (_, result)) = numbers("467..114..").unwrap();
    assert_eq!(vec!(467, 114), result);
    let (_rest, (_, result)) = numbers("..35..633.").unwrap();
    assert_eq!(vec!(35, 633), result);
  }

  #[test]
  fn test_nom_parse_pairs() {
    let mut numbers =
      many1(pair(take_till(AsChar::is_dec_digit), digit1::<_, Error<_>>)
      );
    let (_rest, result) = numbers("467..114..").unwrap();
    assert_eq!(vec!(("", "467"), ("..", "114")), result);
    let (_rest, result) = numbers("..35..633.").unwrap();
    assert_eq!(vec!(("..", "35"), ("..", "633")), result);
  }

  #[test]
  fn test_parse_chars() {
    let subject = "467..114..";
    let (digits, symbols) = parse_line(subject);
    assert_eq!(vec!(0, 1, 2, 5, 6, 7), digits);
    assert_eq!(Vec::<usize>::new(), symbols);
    let subject = "617*......";
    let (digits, symbols) = parse_line(subject);
    assert_eq!(vec!(0, 1, 2), digits);
    assert_eq!(vec!(3), symbols);
  }

  fn parse_line(subject: &str) -> (Vec<usize>, Vec<usize>) {
    let mut digits: Vec<usize> = vec![];
    let mut symbols: Vec<usize> = vec![];
    for (i, char) in subject.chars().enumerate() {
      if char == '.' { continue; } else if char.is_numeric() { digits.push(i) } else { symbols.push(i) }
    }
    (digits, symbols)
  }
}