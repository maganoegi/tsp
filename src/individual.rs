
/*----------------------------------------------------------------------------*/
/*------ Sergey Platonov --------------------- HEPIA, ITI 2018-2019 sem.1 ----*/
/*----------------------------------------------------------------------------*/
/*-------------------------- Traveling Salesman Problem ----------------------*/
/*----------------------------------------------------------------------------*/

extern crate rand;
use self::rand::Rng;
use individual::rand::distributions::range::SampleRange;
use std::cmp::PartialOrd;
use std::fmt;
use distance_matrix::*;

#[derive(Clone,Debug)]
pub struct Individual {
     pub path: Vec<u32>,
     pub fitness: f64,
}

impl Individual {
     pub fn new(cities: Vec<u32>, d: DistanceMatrix) -> Self{
          let score: f64 = 1.0 / (d.get_distance(cities.clone()));
          Individual {
               path: cities,
               fitness: score,
          }
     }

     pub fn mutation(&mut self, d: DistanceMatrix, p: f64) {
          let mut w: Vec<u32> = self.path.clone();
          let rng: f64 = gen(0.0, 1.0);

          if rng <= p {
               let a: usize = gen(0, w.len());
               let mut b: usize = gen(0, w.len());
               while b == a {
                    b = gen(0, w.len());
               } 
               w = switch(w.clone(), a, b);
          }
          self.path = w; 
          let new_score: f64 = 1.0 / (d.get_distance(self.path.clone()));
          self.fitness = new_score;
     }
}


fn gen<T: SampleRange+PartialOrd+Copy>(min: T, max: T) -> T {
	rand::thread_rng().gen_range(min,max)
}

fn switch(v: Vec<u32>, a: usize, b: usize) -> Vec<u32> {
     let mut w: Vec<u32> = v.clone();
     let tmp: u32 = w[a];
     w[a] = w[b];
     w[b] = tmp;

     return w;
}

impl fmt::Display for Individual {
     fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "Length is: {} and the path is: {:?}",self.fitness, self.path)
     }
}

#[cfg(test)]
mod individual_tests {
     use super::*;
     /* Test Data contains a square of sides of length = 2u */
     #[test]
     pub fn mutation_and_switch() {
          let d: DistanceMatrix = DistanceMatrix::new("testData.raw");
          let mut w: Vec<u32> = vec![0, 1, 2, 3];

          let a: usize = 0;
          let mut b: usize = 3;

          let obtained = switch(w, a, b);
          let expected = vec![3, 1, 2, 0];
          assert_eq!(obtained, expected);
     }
}