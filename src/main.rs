pub mod graph;
pub mod matrix;

use crate::matrix::get_matrix_from_file;
use crate::graph::{get_graph_from_matrix, Graph};

const FILE_PATH: &str = "input/graph.txt";

fn main() {
  let matrix = get_matrix_from_file(FILE_PATH.to_string());
  let graph = get_graph_from_matrix(matrix);
  let mut visited = Vec::new();
  Graph::<u32, Vec<u32>>::bfs(&graph, 3, &mut visited);
  println!("{:#?}", visited);

}
