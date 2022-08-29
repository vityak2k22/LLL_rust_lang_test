mod basis;

use std::io::Write;
use std::fs::File;
use basis::input_and_output;

#[allow(non_snake_case)]
fn main () {
  let mut input = File::open("input.txt").unwrap();
  let mut output = File::create("output.txt").unwrap();
  
  output.write_all("LLL algorithm (my code describing <f64>)\n\n".as_bytes()).unwrap();

  let dims = input_and_output::input_dimensions_from_file(&input);
  output.write_all(format!("A matrix {}x{}\n", dims.0, dims.1).as_bytes()).unwrap();

  let mut X = basis::Basis::new(dims, basis::InitType::FileInit(&mut input));
  output.write_all("Input matrix:\n".as_bytes()).unwrap();
  input_and_output::output_matrix_to_file(&X, &mut output);

  X.LLL(0.75);
  output.write_all("\nReduced matrix:\n".as_bytes()).unwrap();
  input_and_output::output_matrix_to_file(&X, &mut output);
}