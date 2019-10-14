
/*----------------------------------------------------------------------------*/
/*------ Sergey Platonov --------------------- HEPIA, ITI 2018-2019 sem.1 ----*/
/*----------------------------------------------------------------------------*/
/*-------------------------- Traveling Salesman Problem ----------------------*/
/*----------------------------------------------------------------------------*/

/* Remarks: a lot of work went into the study of premature convergence, but i chose not to put the functions that allow for 
keeping the diversity of the collective genome into this project. The functions removed are: 
* Judgment Day (randomize all, leave one best) and 
* Social Disaster (leave distinct, randomize the rest). */
extern crate local_lib;


use local_lib::distance_matrix::*;
use local_lib::population::*;
use local_lib::globals::*;
use local_lib::*;

fn main() {
    // Allow the user to tune the program
    let globals: Globals = interact();
    // Extract data and create the distance matrix
    let distance_matrix: DistanceMatrix = DistanceMatrix::new("berlin52.tsp.raw");
    // Initialize the primitive population
    let mut population: Population = Population::new(distance_matrix.clone(), globals);
    
    // Actions applied to each generation
    population.tournament_sort();
    population.reproduce();

    // Initialisation analysis variables
    let mut all_time_best: f64 = population.best_path().fitness;
    let mut all_time_best_path = population.best_path().clone();
    let mut total: f64 = 0.0;
    let mut average: f64 = population.best_path().fitness;
    let mut worst: f64 = population.best_path().fitness;

    println!("\n\n     Approximating...\n");

    // Generations
    for i in 1..globals.n  {
        // Loading Bar
        loading_bar(i, globals.n);

        // Actions
        population.tournament_sort();
        population.reproduce();

        // Average results over the entire process
        total += population.best_path().fitness;
        average = total / i as f64;

        // Analysis of best/worst
        if population.best_path().fitness >= all_time_best {
            all_time_best = population.best_path().fitness;
            all_time_best_path = population.best_path().clone();
        }
        if population.best_path().fitness < worst {
            worst = population.best_path().fitness;
        }
    }

    // Printing relevant information onto the console
    println!("\n\n     BEST distance: {:?}", 1.0/ all_time_best_path.fitness);
    println!("     AVERAGE distance: {}", 1.0/average);
    println!("     WORST distance: {}\n", 1.0/worst);
    
    // Creation of a file with a graphical representation in tmp/ directory
    draw(create_tuple(population.best_path().clone(), distance_matrix));
    




}
