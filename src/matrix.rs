use std::fs;

#[derive(Clone)]
pub struct Matrix<T> {
  pub get: Vec<Vec<T>>
}

pub fn get_matrix_from_file(file_path: String) -> Vec<Vec<i32>> {
  let mut contents = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");
  contents = contents.replace("\r", "");
  let strings: Vec<String> = contents.split("\n").into_iter().map(|a| a.to_string()).collect();
  let mut matrix: Vec<Vec<i32>> = Vec::new();
  for index_of_string in 0..strings.len() {
    let string: Vec<&str> = strings[index_of_string].split(" ").collect();
    let mut relation = Vec::new();
    for char in &string {
      relation.push(char.parse::<i32>().unwrap());
    }
    matrix.push(relation.clone());
  }

  matrix 
}
