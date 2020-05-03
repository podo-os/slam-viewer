//! Many of this code is from [kiss3d](https://github.com/sebcrozet/kiss3d)
//! https://github.com/sebcrozet/kiss3d/blob/master/src/camera/first_person.rs

use super::super::event::WindowEventState;
use super::base::Camera;

use nalgebra::Vector2;
use slam_cv::Number;
use winit::event::*;

pub struct CameraControllerConfig<N>
where
    N: Number,
{
    pub mouse_left_speed: N,
    pub mouse_right_speed: N,
    pub scroll_speed: N,
    pub keyboard_speed: N,
}

impl<N> Into<CameraController<N>> for CameraControllerConfig<N>
where
    N: Number,
{
    fn into(self) -> CameraController<N> {
        CameraController {
            config: self,

            window_size: Vector2::zeros(),

            cursor_d: Vector2::zeros(),
            cursor_pos: None,
            mouse_wheel_d: N::zero(),

            is_left_mouse_pressed: false,
            is_right_mouse_pressed: false,

            is_left_key_pressed: false,
            is_right_key_pressed: false,
            is_up_key_pressed: false,
            is_down_key_pressed: false,
        }
    }
}

pub struct CameraController<N>
where
    N: Number,
{
    config: CameraControllerConfig<N>,

    cursor_d: Vector2<N>,
    cursor_pos: Option<Vector2<N>>,
    mouse_wheel_d: N,

    pub(crate) window_size: Vector2<N>,

    is_left_mouse_pressed: bool,
    is_right_mouse_pressed: bool,

    is_left_key_pressed: bool,
    is_right_key_pressed: bool,
    is_up_key_pressed: bool,
    is_down_key_pressed: bool,
}

impl<N> CameraController<N>
where
    N: Number,
{
    pub fn process_events(&mut self, event: &WindowEvent) -> WindowEventState {
        match event {
            WindowEvent::Resized(size) => {
                self.window_size.x = N::from(size.width).unwrap();
                self.window_size.y = N::from(size.height).unwrap();
                WindowEventState::Unused
            }
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
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_key_pressed = is_pressed;
                        WindowEventState::Consumed
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_key_pressed = is_pressed;
                        WindowEventState::Consumed
                    }
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_up_key_pressed = is_pressed;
                        WindowEventState::Consumed
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_down_key_pressed = is_pressed;
                        WindowEventState::Consumed
                    }
                    _ => WindowEventState::Unused,
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.cursor_pos = None;

                let is_pressed = *state == ElementState::Pressed;
                match button {
                    MouseButton::Left => {
                        self.is_left_mouse_pressed = is_pressed;
                        WindowEventState::Consumed
                    }
                    MouseButton::Right => {
                        self.is_right_mouse_pressed = is_pressed;
                        WindowEventState::Consumed
                    }
                    _ => WindowEventState::Unused,
                }
            }
            // TODO: calibrate
            WindowEvent::CursorMoved { position, .. } => {
                if self.is_left_mouse_pressed || self.is_right_mouse_pressed {
                    let position =
                        Vector2::new(N::from(position.x).unwrap(), N::from(position.y).unwrap());

                    if let Some(cursor_pos) = self.cursor_pos {
                        let delta = position - cursor_pos;
                        self.cursor_d += delta;
                    }

                    self.cursor_pos = Some(position);
                    WindowEventState::Consumed
                } else {
                    WindowEventState::Unused
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.mouse_wheel_d += match delta {
                    MouseScrollDelta::LineDelta(_, ny) => N::from(*ny).unwrap(),
                    // TODO: calibrate
                    MouseScrollDelta::PixelDelta(dp) => N::from(dp.x.max(dp.y)).unwrap(),
                };
                WindowEventState::Consumed
            }
            _ => WindowEventState::Unused,
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera<N>) {
        self.cursor_d.x /= self.window_size.x;
        self.cursor_d.y /= self.window_size.y;

        // mouse movement
        if self.is_left_mouse_pressed {
            camera.rotate(self.cursor_d * self.config.mouse_left_speed);
        }
        if self.is_right_mouse_pressed {
            camera.move_to(self.cursor_d * self.config.mouse_right_speed);
        }
        self.cursor_d = Vector2::zeros();

        // scroll movement
        camera.scale(self.mouse_wheel_d * self.config.scroll_speed);
        self.mouse_wheel_d = N::zero();

        // keyboard input
        let mut dkey = Vector2::<N>::zeros();
        if self.is_left_key_pressed {
            dkey.x += N::one();
        }
        if self.is_right_key_pressed {
            dkey.x -= N::one();
        }
        if self.is_up_key_pressed {
            dkey.y += N::one();
        }
        if self.is_down_key_pressed {
            dkey.y -= N::one();
        }
        dkey *= self.config.keyboard_speed;

        camera.move_to(Vector2::new(dkey.x, N::zero()));
        camera.scale(dkey.y);
    }
}
