use nalgebra::{Matrix4, Point3, Vector3};
use slam_cv::Number;
use winit::event::*;

#[derive(Debug)]
pub struct Camera<N>
where
    N: 'static + Number,
{
    pub eye: Point3<N>,
    pub target: Point3<N>,
    pub up: Vector3<N>,
    pub aspect: N,
    pub fovy: N,
    pub znear: N,
    pub zfar: N,
}

impl<N> Camera<N>
where
    N: 'static + Number,
{
    pub fn build_view_projection_matrix(&self) -> Matrix4<N> {
        let view = Matrix4::look_at_rh(&self.eye, &self.target, &self.up);

        let proj = Matrix4::new_perspective(self.aspect, self.fovy, self.znear, self.zfar);

        proj * view
    }
}

pub struct CameraController<N>
where
    N: 'static + Number,
{
    speed: N,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl<N> CameraController<N>
where
    N: 'static + Number,
{
    pub fn new(speed: N) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::Space => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::LShift => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera<N>) {
        let forward = (camera.target - camera.eye).normalize();

        if self.is_forward_pressed {
            camera.eye += forward * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward * self.speed;
        }

        let right = forward.cross(&camera.up);

        if self.is_right_pressed {
            camera.eye += right * self.speed;
        }
        if self.is_left_pressed {
            camera.eye -= right * self.speed;
        }
    }
}
