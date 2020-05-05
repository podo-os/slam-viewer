use nalgebra::Point3;
use rand_distr::{Distribution, StandardNormal};

fn main() {
    const NUM_POINTS: usize = 10_000;

    let mut rng = rand::thread_rng();
    let mut rng = StandardNormal.sample_iter(&mut rng);

    let points = (0..NUM_POINTS)
        .map(|_| {
            let x = rng.next().unwrap();
            let y = rng.next().unwrap();
            let z = rng.next().unwrap();
            Point3::new(x, y, z - 10.0)
        })
        .collect();

    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
    }

    // make a window with this thread
    slam_viewer::alloc_thread().add_points(points).run();
}
