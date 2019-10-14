
/*----------------------------------------------------------------------------*/
/*------ Sergey Platonov --------------------- HEPIA, ITI 2018-2019 sem.1 ----*/
/*----------------------------------------------------------------------------*/
/*-------------------------- Traveling Salesman Problem ----------------------*/
/*----------------------------------------------------------------------------*/
/*----------------- Partagé avec: Adriano ------------------------------------*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone,Debug)]
pub struct DistanceMatrix {
     pub content: Vec<Vec<f64>>,
     pub coordinates: Vec<Vec<f64>>,
}

impl DistanceMatrix {
     pub fn new(file_path: &str) -> Self{
          let file = File::open(&file_path).expect("Unable to open");
          let br = BufReader::new(file);
          let mut coordinate_matrix: Vec<Vec<f64>> = vec!();
          for line in br.lines() {
               let unsplit = line.unwrap();
               let split = unsplit.split(" ");
               let pieces: Vec<&str> = split.collect();
               let mut coordinates: Vec<f64> = vec!();
               coordinates.push(pieces[0].parse().unwrap());
               coordinates.push(pieces[1].parse().unwrap());
               coordinate_matrix.push(coordinates);
          }

          let qty_inputs: usize = coordinate_matrix.len();
          let mut distance_matrix: Vec<Vec<f64>> = vec![vec![0.0; qty_inputs]; qty_inputs];
          for i in 0..qty_inputs {
               for j in 0..qty_inputs {
                    if i > j {
                         distance_matrix[i][j] = distance_matrix[j][i];
                    } else if i == j {
                         continue;
                    } else {
                         let delta_x: f64 = coordinate_matrix[j][0] - coordinate_matrix[i][0];
                         let delta_y: f64 = coordinate_matrix[j][1] - coordinate_matrix[i][1];
                         distance_matrix[i][j] = ((delta_x * delta_x) + (delta_y * delta_y)).sqrt();
                    }
               }
          }
          DistanceMatrix {
               content: distance_matrix,
               coordinates: coordinate_matrix,
          }
     }

     pub fn distance_between(&self, a: usize, b: usize) -> f64 {
          let result: f64 = self.content[a][b];
          return result;
     }

     pub fn get_distance(&self, cities: Vec<u32>) -> f64 {
          let number_cities: usize = cities.len();
          let mut distance: f64 = self.distance_between(cities.clone()[0] as usize, *cities.clone().last().unwrap() as usize);//0.0;
          let mut i = 0;
          while i < (number_cities - 1) {
               distance += self.content[cities[i] as usize][cities[i+1] as usize]; 
               i += 1;
          }
          return distance;
     }
}

#[cfg(test)]
mod distance_tests {
     use super::*;
     /* Test Data contains a square of sides = 2u */
     #[test]
     pub fn distance_between_2pt() {
          let d: DistanceMatrix = DistanceMatrix::new("testData.raw");
          let a: usize = 0;
          let b: usize = 3;
          assert_eq!(2.0, d.distance_between(a, b)); 
     }
     
     #[test]
     pub fn total_distance_calculation() {
          let d: DistanceMatrix = DistanceMatrix::new("testData.raw");
          let v: Vec<u32> = vec![0, 1, 2, 3];
          assert_eq!(8.0, d.get_distance(v)); 
     }
}