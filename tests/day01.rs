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
}