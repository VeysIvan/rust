// #![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
  [
      [matrix[0][0], matrix[1][0], matrix[2][0]],
      [matrix[0][1], matrix[1][1], matrix[2][1]],
      [matrix[0][2], matrix[1][2], matrix[2][2]],
  ]
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn main() {
  let matrix = [
      [1, 2, 3], //
      [4, 5, 6],
      [7, 8, 9],
  ];

  println!("матрица: {:#?}", matrix);
  let transposed = transpose(matrix);
  println!("транспонированная матрица: {:#?}", transposed);
}