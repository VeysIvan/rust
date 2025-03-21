/// Определяет длину последовательности Коллатца для числа n.
fn collatz_length(mut n: i32) -> u32 {
  let mut length = 0;

  while n != 1 {
      length += 1;
      if n % 2 == 0 {
          n /= 2;
      } else {
          n = 3 * n + 1;
      }
  }

  length + 1
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

fn main() {
  println!("Длина: {}", collatz_length(11));
}