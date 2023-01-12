// TODO: remove this when you're done with your implementation.
// #![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
  // Ignore all spaces. Reject number with less than two digits.
  // Reject entries that do not contain digits.
  let numbers_only: Vec<&str> = cc_number.rmatches(char::is_numeric).collect();
  if numbers_only.len() < 2 {
      return false;
  }
  println!("numbers only is: {:?}", numbers_only);

  // Moving from right to left, double every second digit: for the number 1234, we double 3 and 1.
  let mut doubled_second_digits: Vec<u32> = Vec::new();

  for (i, char) in numbers_only.iter().enumerate() {
      if i == 0 || i % 2 == 0 {
          doubled_second_digits.push(char.parse().unwrap());
      } else {
          doubled_second_digits.push(char.parse::<u32>().unwrap() * 2);
      }
  }
  println!("doubled second digits is: {:?}", &doubled_second_digits);

  // After doubling a digit, sum the digits. So doubling 7 becomes 14 which becomes 5.
  let mut summed_digits: Vec<u32> = Vec::new();
  for digit in doubled_second_digits {
      if digit > 9 {
          let sum_digit: u32 = digit
              .to_string()
              .chars()
              .map(|c| c.to_digit(10).unwrap())
              .sum();
          summed_digits.push(sum_digit);
      } else {
          summed_digits.push(digit);
      }
  }
  println!("summed digits is: {:?}", summed_digits);

  // Sum all the undoubled and doubled digits.
  let mut sum = 0;
  for digit in summed_digits {
      sum += digit;
  }

  println!("sum is: {:?} and sum % 10 is {:?}", sum, sum % 10);
  // The credit card number is valid if the sum ends with 0.
  if sum % 10 == 0 {
      return true;
  } else {
      return false;
  }
}

#[test]
fn test_non_digit_cc_number() {
  assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
  assert!(!luhn(""));
  assert!(!luhn(" "));
  assert!(!luhn("  "));
  assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
  assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
  assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
  assert!(luhn("4263 9826 4026 9299"));
  assert!(luhn("4539 3195 0343 6467"));
  assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
  assert!(!luhn("4223 9826 4026 9299"));
  assert!(!luhn("4539 3195 0343 6476"));
  assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
