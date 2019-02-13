use three;


pub struct Input {
    quit: bool,
    toggle_fullscreen: bool,
}


impl Input {
    pub fn new() -> Self {
        Input { quit: false, toggle_fullscreen: false }
    }

    pub fn handle(&mut self, input: &three::Input) {
        self.quit = input.hit(three::KEY_ESCAPE) || self.quit;

        if input.keys_hit().contains(&three::controls::Key::F11) {
            self.toggle_fullscreen = true;
        } else {
            self.toggle_fullscreen = false;
        }
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    pub fn toggle_fullscreen(&self) -> bool {
        self.toggle_fullscreen
    }
}
