use std::hint::black_box;
use std::time::{Duration, Instant};
use crate::cases::test_cases;
use crate::triangle::{mode_name, rasterize, Triangle, TriangleMode};

const REPETITIONS: usize = 7;

fn all_triangles() -> Vec<Triangle> {
    let mut triangles = Vec::new();
    for case in test_cases() {
        triangles.extend(case.triangles);
    }
    triangles
}

fn median(times: &mut Vec<Duration>) -> Duration {
    times.sort();
    times[times.len() / 2]
}

pub fn run_benchmark(iterations: usize) {
    let triangles = all_triangles();

    let modes = [
        TriangleMode::Wireframe,
        TriangleMode::FilledBarycentric,
        TriangleMode::FilledScanline,
    ];

    println!("Triangulos: {}", triangles.len());
    println!("Ejecuciones por medicion: {}", iterations);
    println!("Mediciones por modo: {}", REPETITIONS);

    for mode in modes {
        let fragment_count = rasterize(&triangles, mode).len();
        let mut times = Vec::new();

        for _ in 0..REPETITIONS {
            let start = Instant::now();

            for _ in 0..iterations {
                let fragments = rasterize(black_box(&triangles), mode);
                black_box(fragments);
            }

            times.push(start.elapsed());
        }

        let med = median(&mut times);
        let per_run = med.as_secs_f64() * 1000.0 / iterations as f64;

        println!(
            "{}: fragmentos {}, mediana {:?} para {} ejecuciones, {:.4} ms por ejecucion",
            mode_name(mode),
            fragment_count,
            med,
            iterations,
            per_run
        );
    }
}
