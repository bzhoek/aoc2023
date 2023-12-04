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
  fn test_nom_parse_pairs() {
    let mut numbers =
      many1(pair(take_till(AsChar::is_dec_digit), digit1::<_, Error<_>>)
      );

    let (_rest, result) = numbers("..35..633.").unwrap();
    assert_eq!(vec!(("..", "35"), ("..", "633")), result);

    let (_rest, result) = numbers("467..114..").unwrap();
    assert_eq!(vec!(("", "467"), ("..", "114")), result);

    let (digits, symbols) = parse_symbols("...*......");
    assert_eq!(vec!(3), symbols);

    let matches = match_numbers(result, symbols);
    assert_eq!(vec!("467"), matches);
  }

  #[test]
  fn test_symbol_matching() {
    let mut numbers =
      many1(pair(take_till(AsChar::is_dec_digit), digit1::<_, Error<_>>)
      );

    let (_rest, result) = numbers("......755.").unwrap();
    assert_eq!(vec!(("......", "755")), result);

    let (digits, symbols) = parse_symbols("...$.*....");
    assert_eq!(vec!(3, 5), symbols);

    let matches = match_numbers(result, symbols);
    assert_eq!(vec!("755"), matches);
  }

  fn match_numbers<'a>(result: Vec<(&'a str, &'a str)>, symbols: Vec<usize>) -> Vec<&'a str> {
    let mut matches: Vec<&str> = vec![];
    let mut offset = 0;
    for (prefix, number) in result.iter() {
      let start = offset + prefix.len();
      let end = start + number.len();
      if let Some(index) = symbols.iter().find(|index| {
        println!("{} {} {}", start, index, end);
        *index + 1 >= start && index <= &&end
      }) {
        matches.push(number);
      };
      offset = end;
    }
    matches
  }

  #[test]
  fn test_parse_chars() {
    let subject = "467..114..";
    let (digits, symbols) = parse_symbols(subject);
    assert_eq!(vec!(0, 1, 2, 5, 6, 7), digits);
    assert_eq!(Vec::<usize>::new(), symbols);
    let subject = "617*......";
    let (digits, symbols) = parse_symbols(subject);
    assert_eq!(vec!(0, 1, 2), digits);
    assert_eq!(vec!(3), symbols);
  }

  fn parse_symbols(subject: &str) -> (Vec<usize>, Vec<usize>) {
    let mut digits: Vec<usize> = vec![];
    let mut symbols: Vec<usize> = vec![];
    for (i, char) in subject.chars().enumerate() {
      if char == '.' { continue; } else if char.is_numeric() { digits.push(i) } else { symbols.push(i) }
    }
    (digits, symbols)
  }
}