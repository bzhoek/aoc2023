#[cfg(test)]
mod tests {
  fn extract_numbers(str: &str) -> Option<(u32, u32)> {
    let first = str.chars().find(|c: &char| c.is_numeric()).and_then(|c| c.to_digit(10));
    let last = str.chars().rev().find(|c: &char| c.is_numeric()).and_then(|c| c.to_digit(10));
    first.zip(last)
  }

  #[test]
  fn test_examples() {
    assert_eq!(Some((1, 2)), extract_numbers("1abc2"));
    assert_eq!(Some((7, 7)), extract_numbers("treb7uchet"));
  }

  #[test]
  fn it_solves_part1() {
    let input = include_str!("day01.input");

    let lines: Vec<_> = input.lines()
      .flat_map(extract_numbers)
      .map(|(first, last)| first * 10 + last)
      .collect();

    let total = lines.into_iter().sum::<u32>();
    assert_eq!(53334, total);
  }

  const NUMBER_WORDS: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
  ];

  fn convert_words(str: &str) -> Option<u32> {
    let first = find_first_number(str);
    let last = find_last_number(str);
    first.zip(last)
      .map(|(first, last)| first * 10 + last)
  }

  #[test]
  fn test_word_conversion() {
    assert_eq!(Some(29), convert_words("two1nine"));
    assert_eq!(Some(42), convert_words("4nineeightseven2"));
  }

  #[test]
  fn test_word_finding() {
    let str = "threefour";
    let result = NUMBER_WORDS.iter().enumerate()
      .find(|(index, word)| { str[0..word.len()].eq(**word) });
    assert_eq!(Some((2usize, &"three")), result)
  }

  fn is_word<'a>(str: &str) -> Option<(usize, &'a &str)> {
    NUMBER_WORDS.iter().enumerate()
      .find(|(index, word)| {
        str.len() >= word.len() &&
          str[0..word.len()].eq(**word)
      })
  }

  fn find_first_number(str: &str) -> Option<u32> {
    for (i, char) in str.chars().enumerate() {
      if char.is_numeric() {
        return char.to_digit(10);
      }
      if let Some((index, number)) = is_word(&str[i..]) {
        return Some((index + 1) as u32);
      }
    }
    None
  }

  fn find_last_number(str: &str) -> Option<u32> {
    for i in (0..str.len()).rev() {
      if let Some(char) = str.chars().nth(i) {
        if char.is_numeric() {
          return char.to_digit(10);
        }
      }
      if let Some((index, number)) = is_word(&str[i..]) {
        return Some((index + 1) as u32);
      }
    }
    None
  }

  #[test]
  fn test_last_number_word() {
    let str = "twosomefour";
    let result = find_last_number(str);
    assert_eq!(Some(4), result)
  }

  #[test]
  fn test_first_number_word() {
    let str = "three";
    let result = is_word(str);
    assert_eq!(Some((2usize, &"three")), result)
  }

  #[test]
  fn test_later_number_word() {
    let str = "somethree";
    let result = find_first_number(str);
    assert_eq!(Some(3), result)
  }

  #[test]
  fn it_solves_part2() {
    let input = include_str!("day01.input");

    let lines: Vec<_> = input.lines()
      .flat_map(convert_words)
      .collect();

    let total = lines.into_iter().sum::<u32>();
    assert_eq!(52834, total);
  }
}