extern crate ndarray;
extern crate ndarray_linalg;
extern crate rand;
extern crate time;

use ndarray::prelude::*;
use ndarray_linalg::InverseInto;
use rand::Rng;
use time::Instant;

fn main() {
    let num_samples = 10;
    let num_features = 3;
    let num_regressions = 100000;

    // Generate random data
    let mut rng = rand::thread_rng();
    let x: Array2<f64> = Array::from_shape_fn((num_samples, num_features), |_| rng.gen());
    let y: Array1<f64> = Array::from_shape_fn(num_samples, |_| rng.gen());

    // Perform linear regressions using matrix stacking
    let start_time = Instant::now();

    let x_stacked = x
        .broadcast((num_regressions, num_samples, num_features))
        .unwrap();
    let y_stacked = y.broadcast((num_regressions, num_samples)).unwrap();

    let results = x_stacked
        .map_axis(Axis(1), |v| v.t().dot(&v).inv_into().unwrap())
        .dot(&x_stacked.map_axis(Axis(1), |v| v.t()))
        .dot(&y_stacked.insert_axis(Axis(2)));

    let matrix_time = start_time.elapsed();

    println!("Matrix time taken: {:?} seconds", matrix_time.as_secs_f64());

    println!("Results using ndarray and linear algebra:");
    println!("{:?}", results.slice(s![0, ..]));
}
