use nalgebra::{Vector3};
use std::fs::{File};
use std::io::{BufRead, BufReader};

fn vector3_from_line(tokens: Vec<&str>) -> Vector3<f64> {
  Vector3::from_iterator(tokens.iter().map(|a| a.parse().unwrap()))
}

// https://en.wikipedia.org/wiki/Wavefront_.obj_file#Face_elements
pub struct Model {
  pub vertices: Vec<Vector3<f64>>,
  pub faces: Vec<Vec<i64>>, // i64s are indexes into the vertices column.
                        // Negative means "from the back of the array".
}

fn parse_face_index(s: &str) -> i64 {
  match s.split("/").next().unwrap().parse() {
    Ok(v) => v,
    Err(e) => panic!("error reading {}: {}", s, e),
  }
}

// TODO: handle parse() errors lol
impl Model {
  pub fn empty() -> Model {
    Model {
      vertices: Vec::new(),
      faces: Vec::new(),
    }
  }

  pub fn from_file(infile: File) -> Model {
    // panics everywhere.
    let mut this = Model::empty();

    for line in BufReader::new(infile).lines() {
      if let Ok(s) = line {
        this.add_line(s);
      }
    }

    this
  }

  fn add_line(self: &mut Self, line: String) -> bool {
    let mut tokens = line.split_whitespace();
    match tokens.next() {
      Some("v") => {
        let rest: Vec<&str> = tokens.collect();
        self.vertices.push(vector3_from_line(rest));
        true
      },

      Some("f") => {
        let rest: Vec<i64> =
          tokens.map(|x| parse_face_index(x)).collect();
        self.faces.push(rest);
        true
      }

      // Don't know what this type is
      Some(_) => {
        false
      },

      None => false
    }
  }
}
