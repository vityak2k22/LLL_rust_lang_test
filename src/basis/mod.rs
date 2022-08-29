pub mod input_and_output;

use std::fs;

// Basis structure is created for work with lattice basis
pub struct Basis {
  dims: (usize, usize),
  matrix: Vec<Vec<f64>>
}

// Type of initialization basis structure
#[allow(dead_code)]
pub enum InitType<'a> {
  DefaultInit,
  InputInit,
  FileInit(&'a mut fs::File)
}

impl Basis {
  // Creating a Basis instance according to a initialization type
  pub fn new (dims: (usize, usize), mut init_type: InitType) -> Basis {
    match &mut init_type {
      InitType::InputInit => Basis {
        dims,
        matrix: input_and_output::input_matrix(dims)
      },
      InitType::DefaultInit => Basis {
        dims,
        matrix: vec![vec![0f64; dims.1]; dims.0]
      },
      InitType::FileInit(file) => Basis {
        dims,
        matrix: input_and_output::input_matrix_from_file(file)
      }
    }
  }
  
  // --------------------CORE OF PROGRAM--------------------
  // LLL algorithm
  #[allow(non_snake_case)]
  pub fn LLL (&mut self, delta: f64) {
    let mut coeffs = Basis::new((self.dims.0, self.dims.0), InitType::DefaultInit);
    let mut Y = Basis::new((self.dims.0, self.dims.1), InitType::DefaultInit);
    Y.ortho_GS(&self, &mut coeffs);
    let mut k = 1;

    while k < self.dims.0 {
      for j in (0..k).rev() {
        if coeffs.matrix[k][j].abs() > 0.5 {
          self.substract_matrix_line_with_multiplier(coeffs.matrix[k][j].round(), k, j);
          Y.ortho_GS(&self, &mut coeffs);
        }
      }
      let a = (delta - coeffs.matrix[k][k-1] * coeffs.matrix[k][k-1]) * Y.scalar_product(k-1, k-1);
      let b = Y.scalar_product(k, k);
      if a <= b {
        k = k + 1;
      }
      else {
        self.swap(k, k-1);
        Y.ortho_GS(&self, &mut coeffs);
        if k - 1 > 1 { k = k - 1 } else { k = 1 }
      }
    }
  }
  
  // --------------------HELP METHODS--------------------
  
  // Gram-Schmidt orthogonalization
  #[allow(non_snake_case)]
  fn ortho_GS (&mut self, X: &Basis, coeffs: &mut Basis) {
    self.assign_matrix_line(X, 0);
    let mut B = Vec::new();
    B.push(self.scalar_product(0, 0));
    for i in 1..self.dims.0 {
      self.assign_matrix_line(X, i);
      for j in 0..i {
        coeffs.matrix[i][j] = self.scalar_product(i, j) / B[j];
        self.substract_matrix_line_with_multiplier(coeffs.matrix[i][j], i, j);
      }
      B.push(self.scalar_product(i, i));
    }
  }
  
  // Swaps basis vectors
  fn swap (&mut self, index_1: usize, index_2: usize) {
    let mut temp = vec![0f64; self.dims.1];
    for j in 0..self.dims.1 {
      temp[j] = self.matrix[index_1][j];
      self.matrix[index_1][j] = self.matrix[index_2][j];
      self.matrix[index_2][j] = temp[j];
    }
  }
  
  // Calculates scalar product basis vectors
  fn scalar_product (&self, index_1: usize, index_2: usize) -> f64 {
    let mut res = 0f64;
    for j in 0..self.dims.1 {
      res += self.matrix[index_1][j] * self.matrix[index_2][j];
    }
    res
  }
  
  // Assigns basis vector values of another basis vector
  fn assign_matrix_line (&mut self, matrix_two: &Basis, index: usize) {
    for j in 0..self.dims.1 {
      self.matrix[index][j] = matrix_two.matrix[index][j];
    }
  }
  
  // Assigns basis vector values according to the formula: [A = A - multiplier * B], where B is another basis vector
  fn substract_matrix_line_with_multiplier (&mut self, mult: f64, index_1: usize, index_2: usize) {
    for j in 0..self.dims.1 {
      self.matrix[index_1][j] -= mult * self.matrix[index_2][j];
    }
  }
}