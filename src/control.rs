use three;

use cgmath;
use cgmath::Rotation3;
use cgmath::Rotation;

const MOUSEWHEEL_SENSITIVITY: f32 = 0.002;
const VIEWING_SENSITIVITY: f32 = 50.0;

pub struct Control<'a, T: three::Object> {
    camera: &'a T,
    camera_position: [f32; 3],
    camera_lookat: [f32; 3],

    mouse_pressed: [bool; 2],
    mouse_pressed_pos: [[f32; 2]; 2],

    toggle_fullscreen: bool,
    quit: bool,
}


impl<'a, T: three::Object> Control<'a, T> {
    pub fn new(object: &'a T) -> Self {
        let camera_position = [0.0, 0.0, 0.5];
        let camera_lookat = [0.0, 1.0, -1.0];

        object.set_position(camera_position);
        object.look_at([0.0, 0.0, 0.0], camera_lookat, None);

        Self { quit: false, toggle_fullscreen: false, camera: object, camera_position, camera_lookat, mouse_pressed: [false; 2], mouse_pressed_pos: [[0.0, 0.0]; 2] }
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    pub fn toggle_fullscreen(&self) -> bool {
        self.toggle_fullscreen
    }

    pub fn handle(&mut self, window: &three::Window) {
        self.quit = window.input.hit(three::KEY_ESCAPE) || self.quit;

        if window.input.keys_hit().contains(&three::controls::Key::F11) {
            self.toggle_fullscreen = true;
        } else {
            self.toggle_fullscreen = false;
        }

        if window.input.hit(three::controls::MouseButton::Left) && !self.mouse_pressed[0] {
            self.mouse_pressed[0] = true;
            self.mouse_pressed_pos[0] = window.input.mouse_pos_ndc().into();
        }

        if !window.input.hit(three::controls::MouseButton::Left) && self.mouse_pressed[0] {
            self.mouse_pressed[0] = false;
        }

        if window.input.hit(three::controls::MouseButton::Middle) && !self.mouse_pressed[1] {
            self.mouse_pressed[1] = true;
            self.mouse_pressed_pos[1] = window.input.mouse_pos_ndc().into();
        }

        if !window.input.hit(three::controls::MouseButton::Middle) && self.mouse_pressed[1] {
            self.mouse_pressed[1] = false;
        }

        if self.mouse_pressed[0] {
            let new_pos: [f32; 2] = window.input.mouse_pos_ndc().into();
            let diff = [new_pos[0] - self.mouse_pressed_pos[0][0], new_pos[1] - self.mouse_pressed_pos[0][1]];
            self.mouse_pressed_pos[0] = new_pos;

            let (width, height): (u32, u32) = match window.glutin_window().get_inner_size() {
                None => panic!("Window doesn't exist"),
                Some(size) => size.into(),
            };

            let aspect_ratio = width as f32 / height as f32;

            let multiplier = self.camera_position[2];

            self.camera_position = [self.camera_position[0] - diff[0]*multiplier*aspect_ratio, self.camera_position[1] - diff[1]*multiplier, self.camera_position[2]];
        }

        if self.mouse_pressed[1] {
            let new_pos: [f32; 2] = window.input.mouse_pos_ndc().into();
            let diff = [new_pos[0] - self.mouse_pressed_pos[1][0], new_pos[1] - self.mouse_pressed_pos[1][1]];
            self.mouse_pressed_pos[1] = new_pos;

            let up = cgmath::Vector3::new(0.0, 0.0, 1.0);
            let rotation_vertical: cgmath::Quaternion<f32> = cgmath::Quaternion::from_axis_angle(up.cross(self.camera_lookat.into()), cgmath::Deg(diff[1]*VIEWING_SENSITIVITY));
            let rotation_z: cgmath::Quaternion<f32> = cgmath::Quaternion::from_axis_angle(up, cgmath::Deg(diff[0]*VIEWING_SENSITIVITY));

            self.camera_lookat = rotation_vertical.rotate_vector(self.camera_lookat.into()).into();
            self.camera_lookat = rotation_z.rotate_vector(self.camera_lookat.into()).into();

            self.camera.look_at([0.0, 0.0, 0.0], self.camera_lookat, None);
        }

        self.camera.set_position(self.camera_position);
        self.camera_position[0] += self.camera_lookat[0] * MOUSEWHEEL_SENSITIVITY * window.input.mouse_wheel() * self.camera_position[2];
        self.camera_position[1] += self.camera_lookat[1] * MOUSEWHEEL_SENSITIVITY * window.input.mouse_wheel() * self.camera_position[2];
        self.camera_position[2] += self.camera_lookat[2] * MOUSEWHEEL_SENSITIVITY * window.input.mouse_wheel() * self.camera_position[2];
    }
}
