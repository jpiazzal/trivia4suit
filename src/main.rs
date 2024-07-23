fn main() {

}

fn is_palindrome(word: String) -> bool {
  let reversed: String = word.chars().rev().collect();
  return word == reversed;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
      assert_eq!(is_palindrome(String::from("kayak")), true);
  }

  #[test]
  fn it_not_works() {
      assert_eq!(is_palindrome(String::from("toto")), false);
  }
}

