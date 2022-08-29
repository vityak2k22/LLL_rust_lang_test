use std::io;
use std::io::{Write, BufRead, Seek};
use std::fs;
use std::str::FromStr;

use crate::basis::Basis;

// Realization of 'one by one' input
pub fn input_line<T: FromStr> (limit: usize) -> Vec<T> {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let mut input: Vec<T> = input.trim().split_whitespace().flat_map(str::parse).collect();
  let mut length = input.len();
  if length != 0 && length < limit {
    panic!("input line is small");
  }
  else if length > limit {
    while length > limit {
      input.pop();
      length -= 1;
    }
  }
  input
}

// Realization of matrix input
pub fn input_matrix (dims: (usize, usize)) -> Vec<Vec<f64>> {
  let m = dims.0;
  let n = dims.1;
  let mut matrix = vec![vec![0f64; n]; m];
  for i in 0..m {
    matrix[i] = input_line(n);
  }
  matrix
}

// Inputting basis dimensions from file
pub fn input_dimensions_from_file (file: &fs::File) -> (usize, usize) {
  let mut file_iter = io::BufReader::new(file).lines();
  let dims: Vec<usize> = file_iter.nth(0).unwrap().unwrap().trim().split_whitespace().flat_map(str::parse).collect();
  (dims[0], dims[1])
}

// Inputting basis vectors from file
pub fn input_matrix_from_file (file: &mut fs::File) -> Vec<Vec<f64>> {
  file.rewind().unwrap();
  let file_iter = io::BufReader::new(file).lines();
  let matrix: Vec<Vec<f64>> = file_iter.skip(1).map(|line| line.unwrap()
                        .split_whitespace()
                        .map(|element| element.parse().unwrap()).collect())
                        .collect();
  matrix
}


  // Console matrix output
  #[allow(dead_code)] 
  pub fn output_matrix (basis: &Basis) {
    for i in 0..basis.dims.0 {
      for j in 0..basis.dims.1 {
        print!("{} ", basis.matrix[i][j]);
      }
      print!("\n");
    }
  }

  // Outputting matrix of basis to file
  #[allow(dead_code)] 
  pub fn output_matrix_to_file (basis: &Basis, file: &mut fs::File) {
  let mut matrix = String::new();
  for i in 0..basis.dims.0 {
      for j in 0..basis.dims.1 {
        matrix += &format!("{} ", basis.matrix[i][j]);
      }
    matrix += &format!("\n");
    }
  file.write_all(matrix.as_bytes()).unwrap();
  }