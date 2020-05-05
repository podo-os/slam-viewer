use cv_core::FeatureMatch;
use nalgebra::Point3;
use rand_distr::{Distribution, StandardNormal};

fn main() {
    const NUM_POINTS: usize = 10_000;
    const SIGMA: f32 = 0.1;

    let mut rng = rand::thread_rng();
    let mut rng = StandardNormal.sample_iter(&mut rng);

    let matches = (0..NUM_POINTS)
        .map(|_| {
            let x = rng.next().unwrap();
            let y = rng.next().unwrap();
            let z = rng.next().unwrap();
            let p1 = Point3::new(x, y, z - 10.0);

            let x = x + rng.next().unwrap() * SIGMA;
            let y = y + rng.next().unwrap() * SIGMA;
            let z = z + rng.next().unwrap() * SIGMA;
            let p2 = Point3::new(x, y, z - 10.0);

            FeatureMatch(p1, p2)
        })
        .collect();

    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
    }

    // make a window with this thread
    slam_viewer::alloc_thread().add_matches(matches).run();
}
