use three;

// Black
const FONT_COLOR: u32 = 0x000000;

pub struct Ui {
    fps: three::Text,
}

impl Ui {
    pub fn new(window: &mut three::Window) -> Self {
        let font = window.factory.load_font_karla();
        let mut fps = window.factory.ui_text(&font, "FPS: 0.0");

        fps.set_font_size(25.0);
        fps.set_color(FONT_COLOR);

        window.scene.add(&fps);

        Ui { fps }
    }

    pub fn update(&mut self, delta: f32) {
        self.fps.set_text(format!("FPS: {:.2}", 1.0 / delta));
    }
}
