use three;

const MOUSE_SENSITIVITY: f32 = 0.002;

pub struct Control<'a, T: three::Object> {
    camera: &'a T,
    camera_position: [f32; 3],

    mouse_pressed: bool,
    mouse_pressed_pos: [f32; 2],
    toggle_fullscreen: bool,
    quit: bool,
}


impl<'a, T: three::Object> Control<'a, T> {
    pub fn new(object: &'a T) -> Self {
        let camera_position = [0.0, 0.0, 0.5];
        object.set_position(camera_position);

        object.look_at([0.0, 0.0, 0.0],[0.0, 0.0, -1.0],None);

        Self { quit: false, toggle_fullscreen: false, camera: object, camera_position, mouse_pressed: false, mouse_pressed_pos: [0.0, 0.0] }
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

        if window.input.hit(three::controls::MouseButton::Left) && !self.mouse_pressed {
            self.mouse_pressed = true;
            self.mouse_pressed_pos = window.input.mouse_pos_ndc().into();
        }

        if !window.input.hit(three::controls::MouseButton::Left) && self.mouse_pressed {
            self.mouse_pressed = false;
        }

        if self.mouse_pressed {
            let new_pos: [f32; 2] = window.input.mouse_pos_ndc().into();
            let diff = [new_pos[0] - self.mouse_pressed_pos[0], new_pos[1] - self.mouse_pressed_pos[1]];
            self.mouse_pressed_pos = new_pos;

            let (width, height): (u32, u32) = match window.glutin_window().get_inner_size() {
                None => panic!("Window doesn't exist"),
                Some(size) => size.into(),
            };

            let aspect_ratio = width as f32 / height as f32;

            let multiplier = self.camera_position[2];

            self.camera_position = [self.camera_position[0] - diff[0]*multiplier*aspect_ratio, self.camera_position[1] - diff[1]*multiplier, self.camera_position[2]];
        }

        self.camera.look_at([0.0, 0.0, 0.0],[0.0, 1.0/self.camera_position[2], -1.0],None);
        self.camera_position[2] *= 1.0 - window.input.mouse_wheel() * MOUSE_SENSITIVITY;
        self.camera.set_position(self.camera_position);
    }
}
