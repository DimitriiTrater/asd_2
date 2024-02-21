use crate::matrix::Matrix;
use std::collections::{HashMap, HashSet, VecDeque};
#[derive(Clone)]
pub struct Graph<Node, TypeOfRelation> {
    pub relations: HashMap<Node, TypeOfRelation>,
}

impl<Node, TypeOfRelation> Graph<Node, TypeOfRelation> {

  pub fn bfs(graph: &Graph<u32, Vec<u32>>, start: u32, visited: &mut Vec<u32>) {
    let mut queue: VecDeque<u32> = VecDeque::new();
    queue.push_back(start);
    visited.push(start);

    while !queue.is_empty() {
      let vertex = queue.pop_front().unwrap();
      for neg in &graph.relations[&vertex] {
        if !visited.contains(&neg) {
            visited.push(neg.clone());
            queue.push_back(neg.clone());
        }
      }
    }
  }
  
  pub fn bfs_with_dist(graph: &Graph<u32, Vec<u32>>, start: u32, dist: &mut Vec<u32>) {
    let mut queue: VecDeque<u32> = VecDeque::new();
    queue.push_back(start);
    dist.push(start);

    while !queue.is_empty() {
      let vertex = queue.pop_front().unwrap();
      for neg in &graph.relations[&vertex] {
        if !dist.contains(&neg) {
            dist.push(neg.clone());
            queue.push_back(neg.clone());
        }
      }
    }
  }

  pub fn bfs_from_to(graph: &Graph<u32, Vec<u32>>, from: u32, to: u32) -> Vec<u32> {
    let mut queue: VecDeque<u32> = VecDeque::new();
    let mut path:       Vec<u32>        = Vec::new();
    let mut visited:    Vec<u32>        = Vec::new();

    queue.push_back(from);
    visited.push(from);

    while !queue.is_empty() {
      let vertex = queue.pop_front().unwrap();

      if vertex == to {
        break;
      }

      for neg in &graph.relations[&vertex] {
        if !visited.contains(&neg) {
            visited.push(neg.clone());
            queue.push_back(neg.clone());
        }
      }
    }


    let mut p = to;

    path.push(p);
    while p != from {
      p = visited[p as usize];
      path.push(p)
    }
    path.reverse();
    return path;
  }

  pub fn find_components(graph: &Graph<u32, Vec<u32>>) -> Vec<u32> {
    let mut visited = Vec::new();
    let mut components = Vec::new();
    for (vertex, _) in &graph.relations {
      if ! visited.contains(&vertex.clone()) {
        Graph::<u32, Vec<u32>>::bfs(graph, vertex.clone(), &mut visited);
        components.push(vertex.clone());
      }
    }
    components
  }
}

pub fn get_graph_from_matrix(matrix: Matrix<u8>) -> Graph<u32, Vec<u32>> {
    let mut hm: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut index = 0;
    for raw in &matrix.get {
        let mut relation: Vec<u32> = Vec::new();
        let mut linked_vertex = 0;
        for elem in raw {
            if *elem == 1 {
                relation.push(linked_vertex);
            }
            linked_vertex += 1;
        }
        hm.insert(index, relation);
        index += 1;
    }
    Graph { relations: hm }
}
