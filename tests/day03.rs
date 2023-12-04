#[cfg(test)]
mod tests {
  use nom::AsChar;
  use nom::bytes::complete::take_till;
  use nom::character::complete::digit1;
  use nom::error::Error;
  use nom::multi::{many0, many1};
  use nom::sequence::pair;

  fn parse_digits(str: &str) -> Vec<(&str, &str)> {
    many0(
      pair(take_till(AsChar::is_dec_digit), digit1::<_, Error<_>>)
    )(str).unwrap().1
  }

  #[test]
  fn it_solves_sample() {
    let input = include_str!("day03.sample");
    let numbers: Vec<_> = input.lines()
      .map(parse_digits)
      .collect();
    assert_eq!(10, numbers.len());

    let mut symbols: Vec<_> = input.lines()
      .map(parse_symbols)
      .map(|(_digits, symbols)| symbols)
      .collect();

    symbols.insert(0, Vec::<usize>::new());
    symbols.push(Vec::<usize>::new());
    assert_eq!(12, symbols.len());

    let mut matches: Vec<&str> = vec![];
    for (i, v) in numbers.iter().enumerate() {
      matches.append(&mut match_numbers(v, &symbols[i..=i + 2]))
    }

    let matches: Vec<u16> = matches.iter().map(|str| str.parse().unwrap()).collect();
    assert_eq!(8, matches.len());
    assert_eq!(vec!(467, 35, 633, 617, 592, 755, 664, 598), matches);
    let sum: u16 = matches.iter().sum();
    assert_eq!(4361, sum);
  }

  #[test]
  fn it_solves_part1() {
    let input = include_str!("day03.input");
    let numbers: Vec<_> = input.lines()
      .map(parse_digits)
      .collect();
    assert_eq!(140, numbers.len());

    let mut symbols: Vec<_> = input.lines()
      .map(parse_symbols)
      .map(|(_digits, symbols)| symbols)
      .collect();

    symbols.insert(0, Vec::<usize>::new());
    symbols.push(Vec::<usize>::new());
    assert_eq!(142, symbols.len());

    let mut matches: Vec<&str> = vec![];
    for (i, v) in numbers.iter().enumerate() {
      matches.append(&mut match_numbers(v, &symbols[i..=i + 2]))
    }

    let matches: Vec<u32> = matches.iter().map(|str| str.parse().unwrap()).collect();
    assert_eq!(1048, matches.len());
    let sum: u32 = matches.iter().sum();
    assert_eq!(531561, sum);
  }

  #[test]
  fn test_nom_parse_pairs() {
    let result = parse_digits("..35..633.");
    assert_eq!(vec!(("..", "35"), ("..", "633")), result);

    let result = parse_digits("467..114..");
    assert_eq!(vec!(("", "467"), ("..", "114")), result);

    let (_digits, symbols) = parse_symbols("...*......");
    assert_eq!(vec!(3), symbols);

    let matches = match_numbers(&result, &[symbols]);
    assert_eq!(vec!("467"), matches);
  }

  #[test]
  fn test_symbol_matching() {
    let mut numbers =
      many1(pair(take_till(AsChar::is_dec_digit), digit1::<_, Error<_>>)
      );

    let (_rest, result) = numbers("......755.").unwrap();
    assert_eq!(vec!(("......", "755")), result);

    let (_digits, symbols) = parse_symbols("...$.*....");
    assert_eq!(vec!(3, 5), symbols);

    let matches = match_numbers(&result, &[symbols]);
    assert_eq!(vec!("755"), matches);
  }

  fn match_numbers<'a>(result: &[(&str, &'a str)], symbols: &[Vec<usize>]) -> Vec<&'a str> {
    let mut matches: Vec<&str> = vec![];
    let mut offset = 0;
    for (prefix, number) in result.iter() {
      let start = offset + prefix.len();
      let end = start + number.len();
      for line in symbols.iter() {
        if let Some(_index) = line.iter().find(|index| {
          // println!("{:?} {} {} {}", number, start, index, end);
          *index + 1 >= start && index <= &&end
        }) {
          matches.push(number);
          break;
        };
      }
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