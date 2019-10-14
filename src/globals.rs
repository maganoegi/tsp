
/*----------------------------------------------------------------------------*/
/*------ Sergey Platonov --------------------- HEPIA, ITI 2018-2019 sem.1 ----*/
/*----------------------------------------------------------------------------*/
/*-------------------------- Traveling Salesman Problem ----------------------*/
/*----------------------------------------------------------------------------*/

// Data Structure that allows me to control my input parameters in a nice and efficient way
#[derive(Copy,Clone,Debug)]
pub struct Globals {
     pub p: f64,
     pub m: usize,
     pub t: usize,
     pub n: usize,
}

impl Globals {
     pub fn new(prob: f64, popsize: usize, toursize: usize, gen: usize) -> Self {
          Globals {
               p: prob,
               m: popsize,
               t: toursize,
               n: gen,
          }
     }
}