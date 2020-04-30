use kiss3d::light::Light;
use kiss3d::window::Window;
use nalgebra::Point3;
use slam_cv::prelude::*;

pub struct Options {
    pub title: String,
    pub framerate: Option<u64>,

    pub point_size: f32,
    pub point_color: Point3<f32>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            title: "SLAM Map Viewer".to_string(),
            framerate: Some(30),

            point_size: 10.0,
            point_color: Colors::red(),
        }
    }
}

impl Options {
    pub fn spawn_window<F, W>(self, world: W)
    where
        F: 'static + Landmark<Number = f32>,
        W: World<Landmark = F>,
    {
        let point_color = self.point_color;
        let mut window = self.make_window();

        while window.render() {
            world.map_landmarks(|f| {
                let p = f.point_world();
                window.draw_point(&p, &point_color);
            });
        }
    }

    fn make_window(self) -> Window {
        let mut window = Window::new(&self.title);

        window.set_light(Light::StickToCamera);
        window.set_point_size(self.point_size);
        window.set_framerate_limit(self.framerate);

        window
    }
}
