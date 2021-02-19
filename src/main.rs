
fn main() {
    println!("Hello, world!");
}

fn heavy_work(est_duration_secs: u64) -> f64 {

    let num_iterations = 20_000_000 * est_duration_secs;

    let mut acum: f64 = 0.0;
    for i in 1..num_iterations {
        acum += ((i as f64)*0.01).ln().exp();
    }

    acum
}
