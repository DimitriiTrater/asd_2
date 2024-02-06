use std::fs;

pub struct Matrix<T> {
  pub get: Vec<Vec<T>>
}

pub fn get_matrix_from_file(file_path: String) -> Matrix<u8> {
  let mut contents = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");
  contents = contents.replace("\r", "");
  let strings: Vec<String> = contents.split("\n").into_iter().map(|a| a.to_string()).collect();
  let mut matrix: Vec<Vec<u8>> = Vec::new();
  for index_of_string in 0..strings.len() {
    let string: Vec<&str> = strings[index_of_string].split(" ").collect();
    let mut relation = Vec::new();
    for char in &string {
      relation.push(char.parse::<u8>().unwrap());
    }
    matrix.push(relation.clone());
  }

  Matrix { get: matrix }
}
