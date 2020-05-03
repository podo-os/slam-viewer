use nalgebra::{Isometry3, Point2, Point3, Quaternion, Translation3, Unit};

use rand_distr::{Distribution, StandardNormal};

#[derive(Clone)]
struct MyWorld {
    points: Vec<MyFeature>,
    frames: Vec<MyKeyFrame>,
}

#[derive(Clone)]
struct MyKeyFrame(Isometry3<f32>);

#[derive(Clone)]
struct MyFeature(Point3<f32>);

impl slam_cv::vo::World for MyWorld {
    type Number = f32;
    type KeyFrame = MyKeyFrame;
    type Landmark = MyFeature;

    fn for_landmarks<F>(&self, mut f: F)
    where
        F: FnMut(&Self::Landmark),
    {
        for feature in &self.points {
            f(feature);
        }
    }

    fn collect_landmarks<B, F>(&self, f: F) -> Vec<B>
    where
        F: FnMut(&Self::Landmark) -> B,
    {
        self.points.iter().map(f).collect()
    }

    fn collect_keyframes<B, F>(&self, f: F) -> Vec<B>
    where
        F: FnMut(&Self::KeyFrame) -> B,
    {
        self.frames.iter().map(f).collect()
    }

    fn load(&self) {}

    fn save(&self) {}
}

impl slam_cv::frame::KeyFrame for MyKeyFrame {
    type Number = f32;
    type Feature = MyFeature;

    fn for_landmarks<F>(&self, _f: F)
    where
        F: FnMut(&Self::Feature),
    {
        unimplemented!()
    }

    fn isometry(&self) -> Isometry3<f32> {
        self.0
    }
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
        unimplemented!()
    }
}

impl slam_cv::feature::Landmark for MyFeature {
    fn point_world(&self) -> Point3<Self::Number> {
        self.0
    }
}

fn main() {
    const NUM_POINTS: usize = 10_000;
    const NUM_FRAMES: usize = 20;

    let mut rng = rand::thread_rng();
    let mut rng = StandardNormal.sample_iter(&mut rng);

    let mut tx = 0.0;
    let mut ty = 0.0;
    let mut tz = 0.0;

    let mut rx = 0.0;
    let mut ry = 0.0;
    let mut rz = 0.0;
    let mut rw = std::f32::consts::FRAC_PI_2;

    let world = MyWorld {
        points: (0..NUM_POINTS)
            .map(|_| {
                let x = rng.next().unwrap();
                let y = rng.next().unwrap();
                let z = rng.next().unwrap();
                MyFeature(Point3::new(x, y, z - 10.0))
            })
            .collect(),
        frames: (0..NUM_FRAMES)
            .map(|_| {
                tx += rng.next().unwrap() * 0.1;
                ty += rng.next().unwrap() * 0.1;
                tz -= 0.5;
                let translation = Translation3::new(tx, ty, tz);

                rx += rng.next().unwrap() * 0.1;
                ry += rng.next().unwrap() * 0.1;
                rz += rng.next().unwrap() * 0.1;
                rw += rng.next().unwrap() * 0.1;
                let rotation = Unit::new_normalize(Quaternion::new(rx, ry, rz, rw));

                MyKeyFrame(Isometry3::from_parts(translation, rotation))
            })
            .collect(),
    };

    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
    }

    // make a window with this thread
    slam_viewer::alloc_thread().add(world).run();

    // make a window with a new thread
    // slam_viewer::alloc_thread().add(world).spawn().wait();
}
