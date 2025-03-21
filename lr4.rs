fn magnitude(v: &[f64]) -> f64 {
  let sum_of_squares: f64 = v.iter().map(|x| x * x).sum();
  sum_of_squares.sqrt()
}

fn normalize(v: &mut [f64]) {
  let magnitude = magnitude(v);
  for x in v {
      *x /= magnitude;
  }
}

fn main() {
  println!("Модуль единичного вектора: {}", magnitude(&[0.0, 1.0, 0.0]));

  let mut v = [1.0, 2.0, 9.0];
  println!("Модуль {v:?}: {}", magnitude(&v));
  normalize(&mut v);
  println!("Модуль {v:?} после нормализации: {}", magnitude(&v));
}