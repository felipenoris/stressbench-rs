
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

fn main() {
    println!("Hello, world!");

    let duration = 10;
    let total_jobs = 32;

    for num_jobs in 1..(total_jobs+1) {
        run_benchmark(num_jobs, duration);
    }

    println!("Acabou o benchmark!");
}

fn run_benchmark(num_jobs: usize, est_duration_secs: usize) {
    let start = Instant::now();
    let handles = create_heavy_work_tasks(num_jobs, est_duration_secs);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Benchmark com {} jobs durou {:.3} segundos", num_jobs, (start.elapsed().as_millis() as f64) / 1000.0);
}

fn heavy_work(est_duration_secs: usize) -> f64 {

    let num_iterations = 20_000_000 * est_duration_secs;

    let mut acum: f64 = 0.0;
    for i in 1..(num_iterations+1) {
        acum += ((i as f64)*0.01).ln().exp();
    }

    acum
}

fn create_heavy_work_tasks(num_jobs: usize, est_duration_secs: usize) -> Vec<JoinHandle<()>> {
    let mut jobs = vec![];

    for _i in 0..num_jobs {
        jobs.push(
            thread::spawn(move || {
                let result = heavy_work(est_duration_secs);
                
                if result > 499999995000000.0 {
                    println!("{}", result);
                }
            })
        );
    }

    jobs
}
