pub mod graph;
pub mod matrix;
use std::{collections::{vec_deque, VecDeque}, ops::Index};

use crate::matrix::get_matrix_from_file;

const FILE_PATH: &str = "input/graph.txt";


fn trace(from: i32, to: i32, v: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut frontier:   VecDeque<i32>   = VecDeque::new();
    let mut path:       Vec<i32>        = Vec::new();
    let mut visited:    Vec<i32>        = Vec::new();

    visited.resize(v.len(), 0xffff);

    frontier.push_front(from);
    visited[from as usize] = from;

    while !frontier.is_empty() {
        let p = frontier.pop_front();

        if p.unwrap() == to {
            break;
        }

        let nbrs = &v[p.unwrap() as usize];

        for n in nbrs {
            if visited[*n as usize] == 0xffff {
                visited[*n as usize] = p.unwrap();
                frontier.push_back(*n);
            }
        }
    }

    let mut p = to;

    path.push(p);

    while p != from {
        p = visited[p as usize];
        path.push(p);
    }

    path.reverse();

    return path;
}

fn bfs(graph: Vec<Vec<i32>>, s: Option<i32>) -> Vec<i32>
{
  let mut queue = VecDeque::new();
  queue.push_front(s.unwrap_or(0));
  let mut dist: Vec<i32> = Vec::new();
  dist.resize(graph.len(), -1);
  dist[s.unwrap_or(0) as usize] = 0;
  while !queue.is_empty() {
    let vertex = queue.pop_front();
    for adj in &graph[vertex.unwrap() as usize] {
      if dist[*adj as usize] == -1 {
        queue.push_back(adj.clone());
        dist[*adj as usize] = dist[(vertex.unwrap()) as usize] + 1;
      }
    }
  }
  return dist;
}

fn flatten(v: Vec<Vec<i32>>) -> Vec<i32> {
  let mut res = Vec::new();
  for ls in v {
    for el in ls {
      res.push(el);
    }
  }
  res
}

fn components(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let mut com = Vec::new();
  let mut vis = Vec::new();
  let mut prev = bfs(graph.clone(), None);
  let mut temp = Vec::new();
  for (i, j) in prev.iter().enumerate() {
    if *j != -1 {
      temp.push(i as i32);
    }
  }
  com.push(temp.clone());
  temp.clear();
  while prev.contains(&(-1)) {  
    let mut nx = Option::None;
    loop {
      nx = prev.iter().position(|&r| r == -1);
      match nx {
        Some(nx) => {
          if vis.contains(&(nx as i32)) {
            prev[nx] = 0;
          } 
          else 
          {
            break;
          }
        },
        None => return com
      }
    }
    
    let n = match nx {
      Some(nx) => Some(nx as i32),
      None => None  
    };
    prev = bfs(graph.clone(), n);
    
    for (i, j) in prev.iter().enumerate() {
      if *j != -1 {
        temp.push(i as i32);
      }
    }
    com.push(temp.clone());
    temp.clear();
    vis = flatten(com.clone());
  }
  com

}

fn main() {
  let matrix = get_matrix_from_file(FILE_PATH.to_string());
  let mut re: Vec<Vec<i32>> = Vec::new();
  re.resize(matrix.len(), vec![]);
  for (i, row) in matrix.iter().enumerate() {
    for (j, el) in row.iter().enumerate() {
      if *el == 1 {
        re[i].push(j as i32);
      }
    }
  }

  println!("{:#?}", components(re));
}
