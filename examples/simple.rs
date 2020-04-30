use nalgebra::{Point2, Point3};

use rand_distr::{Distribution, StandardNormal};
use slam_viewer::Viewer;

struct MyWorld(Vec<MyFeature>);
struct MyFeature(Point3<f32>);

impl slam_cv::vo::World for MyWorld {
    type Landmark = MyFeature;

    fn for_landmarks<F>(&self, mut f: F)
    where
        F: FnMut(&Self::Landmark),
    {
        for feature in &self.0 {
            f(feature);
        }
    }

    fn collect_landmarks<B, F>(&self, f: F) -> Vec<B>
    where
        F: FnMut(&Self::Landmark) -> B,
    {
        self.0.iter().map(f).collect()
    }

    fn load(&self) {}

    fn save(&self) {}
}

impl slam_cv::feature::Feature for MyFeature {
    type Number = f32;
}

impl slam_cv::feature::KeyPoint for MyFeature {
    fn point_image(&self) -> Point2<Self::Number> {
        self.0.xy()
    }
}

impl slam_cv::feature::Descriptor for MyFeature {
    type Distance = ();

    fn get_distance(&self, _other: &Self) -> Self::Distance {
        ()
    }
}

impl slam_cv::feature::Landmark for MyFeature {
    fn point_world(&self) -> Point3<Self::Number> {
        self.0
    }
}

fn main() {
    const NUM_POINTS: usize = 200;

    let mut rng = rand::thread_rng();
    let mut rng = StandardNormal.sample_iter(&mut rng);

    let world = MyWorld(
        (0..NUM_POINTS)
            .map(|_| {
                let x = rng.next().unwrap();
                let y = rng.next().unwrap();
                let z = rng.next().unwrap();
                MyFeature(Point3::new(x, y, z))
            })
            .collect(),
    );

    let viewer = Viewer::default();
    viewer.run(world);
}
