
/*----------------------------------------------------------------------------*/
/*------ Sergey Platonov --------------------- HEPIA, ITI 2018-2019 sem.1 ----*/
/*----------------------------------------------------------------------------*/
/*-------------------------- Traveling Salesman Problem ----------------------*/
/*----------------------------------------------------------------------------*/

extern crate rand;
use self::rand::Rng;
use population::rand::distributions::range::SampleRange;
use std::cmp::PartialOrd;

use individual::*;
use distance_matrix::*;
use globals::*;


#[derive(Clone,Debug)]
pub struct Population {
     pub inhabitants: Vec<Individual>,
     distances: DistanceMatrix,
     d_total: f64,
     globals: Globals
}

impl Population {
     pub fn new(d: DistanceMatrix, g: Globals) -> Self {
     
          let start_vector: Vec<u32> = create_start_vec(d.content.len());
          let permutations: Vec<Vec<u32>> = selective_permutation(start_vector, g.m);

          let mut host: Vec<Individual> = vec!();
          let mut total_d: f64 = 0.0;
          for i in 0..permutations.len() {
               let ind: Individual = Individual::new(permutations[i].clone(), d.clone());
               total_d += ind.fitness;
               host.push(ind);
          }
          Population {
               inhabitants: host,
               distances: d,
               d_total: total_d,
               globals: g,
          }
     }

     pub fn tournament_sort(&mut self) {
          let input: Vec<Individual> = self.inhabitants.clone();
          let mut output: Vec<Individual> = vec!();
          let len: usize = input.len();
          for _i in 0..self.globals.m {
               output.push(tournament(input.clone(), generate_pool(len, self.globals.t), self.globals.t));
          }
          self.inhabitants = output;
     }

     pub fn reproduce(&mut self) {
          let mut i: usize = 0;
          while i < self.inhabitants.len() {

               let p_1: Vec<u32> = self.inhabitants[i].path.clone();
               let p_2: Vec<u32> = self.inhabitants[i+1].path.clone();

               let max: u32 = self.inhabitants[i].path.len() as u32;

               let random_positions_1: Vec<u32> = k_random_indexes(max);
               let random_positions_2: Vec<u32> = k_random_indexes(max);

               let random_vals_1: Vec<u32> = index_2_vals(random_positions_1.clone(), p_1.clone());
               let random_vals_2: Vec<u32> = index_2_vals(random_positions_2.clone(), p_2.clone());

               let child_prototype_1: Vec<u32> = crossover(random_vals_1, p_1.clone(), p_2.clone());
               let child_prototype_2: Vec<u32> = crossover(random_vals_2, p_2.clone(), p_1.clone());

               let mut ind_1: Individual = Individual::new(child_prototype_1.clone(), self.distances.clone());
               let mut ind_2: Individual = Individual::new(child_prototype_2.clone(), self.distances.clone());

               ind_1.mutation(self.distances.clone(), self.globals.p);
               ind_2.mutation(self.distances.clone(), self.globals.p);

               self.inhabitants[i] = ind_1;
               self.inhabitants[i+1] = ind_2;

               i += 2;
          }
          self.update();
     }

     pub fn update(&mut self) {
          let mut new_total: f64 = 0.0; 
          for i in 0..self.inhabitants.len() {
               new_total += self.inhabitants[i].fitness;
          }
          self.d_total = new_total;
     }

     pub fn best_path(&self) -> &Individual {
          let mut best: &Individual = &self.inhabitants[0];
          for i in 1..self.inhabitants.len() {
               if self.inhabitants[i].fitness >= best.fitness {
                    best = &self.inhabitants[i];
               }
          }
          return best;
     }
}

fn generate_pool(len: usize, t: usize) -> Vec<usize> {
     let mut random_indexes: Vec<usize> = vec![gen(0, len)];
     let mut j: usize = 1;
     while j < t {
          let rdm: usize = gen(0, len);
          if !random_indexes.contains(&rdm) {
               random_indexes.push(rdm);
               j += 1;
          } else {
               continue;
          }
     }
     return random_indexes;
}

fn tournament(v: Vec<Individual>, w: Vec<usize>, t: usize) -> Individual {
     let mut best: Individual = v[w[0]].clone();
     for i in 1..t {
          if v[w[i]].fitness > best.fitness {
               best = v[w[i]].clone();
          }
     }
     return best;
}

fn crossover(random_vals_ex: Vec<u32>, p_1: Vec<u32>, p_2: Vec<u32>) -> Vec<u32> {
     let mut _random_vals: Vec<u32> = random_vals_ex.clone();
     let mut _child_prototype: Vec<u32> = vec!();
     let mut valid_values: Vec<u32> = vec!();

     for i in 0..p_2.len() {
          if !random_vals_ex.contains(&p_2[i]) {
               valid_values.push(p_2[i]);
          }
     }
     _child_prototype = p_1.clone();
     for i in 0.._child_prototype.len() {
          if valid_values.contains(&_child_prototype[i]) {
               let tmp = valid_values.remove(0);
               _child_prototype[i] = tmp;
               valid_values.push(tmp);
          }
     }
     return _child_prototype;
}

fn k_random_indexes(max: u32) -> Vec<u32> {
     let mut random_positions: Vec<u32> = vec![gen(0, max)];
     let mut j: usize = 1;
     let k: usize = gen(0, max as usize);
     while j < k {
          let rdm: u32 = gen(0, max);
          if !random_positions.contains(&rdm) {
               random_positions.push(rdm);
               j += 1;
          } else {
               continue;
          }
     }
     return random_positions;
}

// Finds values that correspond to certain indexes
fn index_2_vals(v: Vec<u32>, w: Vec<u32>) -> Vec<u32> {
     let mut res: Vec<u32> = vec!();
     for i in 0..v.len() {
          res.push(w[v[i] as usize]);
     }
     return res;
}

// I chose to create the population by randomizing each individual in it. Better for larger data sets
fn selective_permutation(v: Vec<u32>, m: usize) -> Vec<Vec<u32>> {
     let length: usize = v.len();
     let mut res: Vec<Vec<u32>> = vec![generate_random_vector(length)];
     for _i in 1..m {
          let mut tmp: Vec<u32> = generate_random_vector(length);
          while res.contains(&tmp) {
               tmp = generate_random_vector(length);
          }
          res.push(tmp);
     }
     return res;
}

fn generate_random_vector(length: usize) -> Vec<u32> {
     let mut tmp: Vec<u32> = vec![gen(0, length as u32)];

     while tmp.len() < length {
          let rng: u32 = gen(0, length as u32);
          if !tmp.contains(&rng) {
               tmp.push(rng);
          } else {
               continue;
          }
     }
     return tmp;
}

fn gen<T: SampleRange+PartialOrd+Copy>(min: T, max: T) -> T {
	rand::thread_rng().gen_range(min,max)
}

fn create_start_vec(length: usize) -> Vec<u32> {
     let mut sample: Vec<u32> = vec!();
     for i in 0 .. length {
          sample.push(i as u32);
     }
     return sample;
}

#[cfg(test)]
mod population_tests {
     use super::*;
     /* Test Data contains a square of sides of length = 2u */

     // The functions that contains random number generation have been opened up here for testing purposes!!
     #[test]
     pub fn crossing_over() {
          let p_1: Vec<u32> = vec![0, 1, 2, 3, 4, 5];
          let p_2: Vec<u32> = vec![5, 4, 3, 2, 1, 0];

          let vals: Vec<u32> = vec![1, 2, 5];

          let obtained = crossover(vals, p_1, p_2);
          let expected = vec![4, 1, 2, 3, 0, 5];
          assert_eq!(obtained, expected);
     }

     #[test]
     pub fn tournament_test() {
          let d: DistanceMatrix = DistanceMatrix::new("testData.raw");
          let mut input: Vec<Individual> = vec![
               Individual::new(vec![0, 1, 3, 2], d.clone()),
               Individual::new(vec![1, 0, 2, 3], d.clone()),
               Individual::new(vec![2, 3, 1, 0], d.clone()),
               Individual::new(vec![0, 1, 2, 3], d.clone())
          ];
          let mut obtained: Vec<Individual> = vec!();
          let expected: Vec<Individual> = vec![
               Individual::new(vec![0, 1, 2, 3], d.clone()),
               Individual::new(vec![0, 1, 2, 3], d.clone()),
               Individual::new(vec![0, 1, 2, 3], d.clone()),
               Individual::new(vec![0, 1, 2, 3], d.clone())
          ];
          let mut result: bool = true;
          for i in 0..4 {
               let mut random_indexes: Vec<usize> = vec![2, 3, 1, 0];
               
               let mut best: Individual = input[random_indexes[0]].clone();
               let mut counter: usize = 0;
               for i in 1..4 {
                    if input[random_indexes[i]].fitness > best.fitness {
                         best = input[random_indexes[i]].clone();
                         counter = i;
                    }
               }
               obtained.push(best);
               if obtained[i].path != expected[i].path {
                    result = false;
               }
          }
          assert_eq!(result, true);
     }
}


