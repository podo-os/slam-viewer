mod opt;

use nalgebra::Point3;

struct MyWorld(Vec<MyFeature>);
struct MyFeature(Point3<f32>);

impl slam_cv::World for MyWorld {
    type Feature = MyFeature;

    fn for_features<F>(&self, mut f: F)
    where
        F: FnMut(&Self::Feature),
    {
        for feature in &self.0 {
            f(feature);
        }
    }
}

impl slam_cv::Feature for MyFeature {
    type Number = f32;

    fn point_world(&self) -> Point3<Self::Number> {
        self.0
    }
}

// TODO: move to samples
fn main() {
    let options = opt::Options::default();

    let world = MyWorld(vec![
        MyFeature(Point3::new(-0.1, -0.1, 0.0)),
        MyFeature(Point3::new(0.0, 0.1, 0.0)),
        MyFeature(Point3::new(0.1, -0.1, 0.0)),
    ]);
    options.spawn_window(world);
}
