
/*----------------------------------------------------------------------------*/
/*------ Sergey Platonov --------------------- HEPIA, ITI 2018-2019 sem.1 ----*/
/*----------------------------------------------------------------------------*/
/*-------------------------- Traveling Salesman Problem ----------------------*/
/*----------------------------------------------------------------------------*/

extern crate plotlib;
use plotlib::style::Line;


pub mod individual;
pub mod population;
pub mod distance_matrix;
pub mod globals;

use std::env;

use distance_matrix::*;
use individual::*;
use globals::*;

pub fn match_int_val(val_2_parse: &String) -> Option<i64> {
    match val_2_parse.parse() {
        Ok(n) => {
            Some(n)
        },
        Err(_) => {
            None
        },
    }
}

pub fn interact() -> Globals {
     let args: Vec<String> = env::args().collect();
     match &args.len() {
          1 => {
               println!("\nLaunching Program With Default (Tested) Parameters");
               println!("P = 70; M = 2000; T = 5; N = 800");
               Globals::new(0.7, 2000, 5, 800)
          },
          _ => {
               let v: Vec<i64> = parse_coordinates(args);
               if v.len() == 4 {
                    if v[1]%2 == 0 {
                         if v[2] <= v[1] {
                              if v[0] >= 0 && v[1] > 0 && v[2] > 0 && v[3] > 0 {
                                   Globals::new(v[0] as f64 / 100.0, v[1] as usize, v[2] as usize, v[3] as usize)
                              } else {
                                   println!("Invalid values - default parameters taken!");
                                   Globals::new(0.7, 2000, 5, 800)
                              }
                         } else {
                              panic!("Tournament size cannot be larger than population size!")
                         }
                    } else {
                         panic!("WARNING: Population size must be an even number")
                    }
               } else {
                    println!("PLEASE RESPECT THE FOLLOWING FORMAT:");
                    println!("Probability of mutation in %,    Population Size,    Tournament Size,    Number Of Generations");
                    println!("Without parameters: the following values are chosen: 70, 2000, 5, 800");
                    panic!("WARNING: Invalid Input Format!")
               }
          },
     }
}

fn parse_coordinates(args: Vec<String>) -> Vec<i64> {
     let mut v: Vec<i64> = vec![];
     for i in 1..args.len() {
          if let Some(x) = match_int_val(&args[i]) {
               v.push(x);
          }
     }
     return v;
}


pub fn loading_bar(i: usize, t: usize) {
        let percentage: usize = 1 + ((i as f64 / t as f64) * 100.0) as usize;
        let mut display_string = "     [".to_owned();
        let space_string: &str = " ";
        let charge_string: &str = "=";
        for _j in 0..percentage {
            display_string += charge_string;
        } 
        //display_string += ">" as &str;
        for _k in percentage..100 {
            display_string += space_string;
        }
        display_string += "]" as &str;
        print!("\r{}  {}%", display_string, percentage);
    }

pub fn create_tuple(ind: Individual, d: DistanceMatrix) -> Vec<(f64, f64)> {
     let v: Vec<u32> = ind.path.clone();
     let mut result: Vec<(f64, f64)> = vec!();
     for i in 0..v.len() {
          result.push((d.coordinates[i][0], d.coordinates[i][1]));
     }
     return result;
}

pub fn draw(points: Vec<(f64, f64)>) {

     let l1 = plotlib::line::Line::new(&points)
          .style(plotlib::line::Style::new().colour("black"));
     let v = plotlib::view::ContinuousView::new().add(&l1);
     plotlib::page::Page::single(&v)
     .save("./tmp/line".to_owned()
               /* +&format!("{:0>8}", i) */
               +&".svg".to_owned())
     .expect("saving svg");

}
