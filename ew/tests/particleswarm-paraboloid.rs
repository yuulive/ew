use num::abs;

use ew::{
    tools::{logging, stopchecker},
    GoalFromFunction, Optimizer,
    particleswarm::{
        self,
        initializing,
        postmove,
        velocitycalc,
        PostMove,
    },
};

use ew_testfunc;

type Coordinate = f32;

#[test]
fn test_particleswarm_paraboloid() {
    // General parameters
    let minval: Coordinate = -100.0;
    let maxval: Coordinate = 100.0;
    let particles_count = 150;
    let dimension = 5;
    let intervals = vec![(minval, maxval); dimension];
    let phi_personal = 2.0;
    let phi_global = 6.0;
    let k = 0.2;

    // Goal function
    let goal = GoalFromFunction::new(ew_testfunc::paraboloid);

    // Particles initializers
    let coord_initializer = initializing::RandomCoordinatesInitializer::new(intervals.clone(), particles_count);
    let velocity_initializer = initializing::ZeroVelocityInitializer::new(dimension, particles_count);

    // PostMove
    let post_moves: Vec<Box<dyn PostMove<Coordinate>>> = vec![Box::new(postmove::MoveToBoundary::new(intervals.clone()))];

    // Velocity calculator
    let velocity_calculator = velocitycalc::CanonicalVelocityCalculator::new(phi_personal, phi_global, k);

    // Stop checker
    let change_max_iterations = 150;
    let change_delta = 1e-7;
    let stop_checker = stopchecker::CompositeAny::new(vec![
        Box::new(stopchecker::Threshold::new(1e-6)),
        Box::new(stopchecker::GoalNotChange::new(
            change_max_iterations,
            change_delta,
        )),
        Box::new(stopchecker::MaxIterations::new(3000)),
    ]);

    // Logger
    let loggers: Vec<Box<dyn logging::Logger<Vec<Coordinate>>>> = vec![];

    let mut optimizer = particleswarm::ParticleSwarmOptimizer::new(
        Box::new(goal),
        Box::new(stop_checker),
        Box::new(coord_initializer),
        Box::new(velocity_initializer),
        Box::new(velocity_calculator),
        );
    optimizer.set_loggers(loggers);
    optimizer.set_post_moves(post_moves);

    match optimizer.find_min() {
        None => assert!(false),
        Some((solution, goal_value)) => {
            for i in 0..dimension {
                assert!(abs(solution[i] - (i as f32 + 1.0)) < 0.3);
            }

            assert!(abs(goal_value) < 0.1);
        }
    }
}
