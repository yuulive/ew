//! Example of optimizing the Schwefel function with genetic algorithm.
//!
//! y = f(x), where x = (x0, x1, ..., xi,... xn).
//! Global minimum is x' = (420.9687, 420.9687, ...) for any xi lying in [-500.0; 500.0].
//! f(x') = 0
//!
//! # Terms
//! * `Goal function` - the function for optimization. y = f(x).
//! * `Gene` - a single value of xi.
//! * `Chromosome` - a point in the search space. x = (x0, x1, x2, ..., xn).
//! * `Individual` - union of x and value of goal function.
//! * `Population` - set of the individuals.
//! * `Generation` - a number of iteration of genetic algorithm.

use num::abs;

use ew::genetic::{self, creation, cross, mutation, pairing, pre_birth, selection};
use ew::tools::logging;
use ew::tools::stopchecker;
use ew::{GoalFromFunction, Optimizer};
use ew_testfunc;

/// Gene type
type Gene = f32;

/// Chromosomes type
type Chromosomes = Vec<Gene>;

#[test]
fn genetic_schwefel() {
    // General parameters

    // Search space. Any xi lies in [-500.0; 500.0]
    let minval: Gene = -500.0;
    let maxval: Gene = 500.0;

    // Count individuals in initial population
    let population_size = 800;

    // Count of xi in the chromosomes
    let chromo_count = 5;

    let intervals = vec![(minval, maxval); chromo_count];

    // Make a trait object for goal function (Schwefel function)
    let goal = GoalFromFunction::new(ew_testfunc::schwefel);

    // Make the creator to create initial population.
    // RandomCreator will fill initial population with individuals with random chromosomes in a
    // given interval,
    let creator = creation::vec_float::RandomCreator::new(population_size, intervals.clone());

    // Make a trait object for the pairing.
    // Pairing is algorithm of selection individuals for crossbreeding.

    // Select random individuals from the population.
    // let pairing = pairing::RandomPairing::new();

    // Tournament method.
    let families_count = population_size / 2;
    let rounds_count = 5;
    let pairing = pairing::Tournament::new(families_count).rounds_count(rounds_count);

    // Crossbreeding algorithm.
    // Make a Cross trait object. The bitwise crossing for float genes.
    let single_cross = cross::FloatCrossExp::new();
    let cross = cross::VecCrossAllGenes::new(Box::new(single_cross));

    // Make a Mutation trait object.
    // Use bitwise mutation (change random bits with given probability).
    let mutation_probability = 15.0;
    let mutation_gene_count = 3;
    let single_mutation = mutation::BitwiseMutation::new(mutation_gene_count);
    let mutation = mutation::VecMutation::new(mutation_probability, Box::new(single_mutation));

    // Pre birth. Throw away new chlld chromosomes if their genes do not lies if given intervals.
    let pre_births: Vec<Box<dyn genetic::PreBirth<Chromosomes>>> = vec![Box::new(
        pre_birth::vec_float::CheckChromoInterval::new(intervals.clone()),
    )];

    // Stop checker. Stop criterion for genetic algorithm.
    let change_max_iterations = 200;
    let change_delta = 1e-7;

    // Stop algorithm if the value of goal function will become less of 1e-4 or
    // after 3000 generations (iterations).
    let stop_checker = stopchecker::CompositeAny::new(vec![
        Box::new(stopchecker::Threshold::new(1e-4)),
        Box::new(stopchecker::GoalNotChange::new(
            change_max_iterations,
            change_delta,
        )),
        Box::new(stopchecker::MaxIterations::new(3000)),
    ]);

    // Make a trait object for selection. Selection is killing the worst individuals.
    // Kill all individuals if the value of goal function is NaN or Inf.
    // Kill the worst individuals to population size remained unchanged.
    let selections: Vec<Box<dyn genetic::Selection<Chromosomes>>> = vec![
        Box::new(selection::KillFitnessNaN::new()),
        Box::new(selection::LimitPopulation::new(population_size)),
    ];

    // Make a loggers trait objects
    let loggers: Vec<Box<dyn logging::Logger<Chromosomes>>> = vec![];

    // Construct main optimizer struct
    let mut optimizer = genetic::GeneticOptimizer::new(
        Box::new(goal),
        Box::new(stop_checker),
        Box::new(creator),
        Box::new(pairing),
        Box::new(cross),
        Box::new(mutation),
        selections,
        pre_births,
    );
    optimizer.set_loggers(loggers);

    // Run genetic algorithm
    match optimizer.find_min() {
        None => assert!(false),
        Some((solution, goal_value)) => {
            for i in 0..chromo_count {
                assert!(abs(solution[i] - 421.0) < 0.1);
            }

            assert!(abs(goal_value) < 1e-3);
        }
    }
}
