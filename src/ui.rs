use three;

// Black
const FONT_COLOR: u32 = 0x000000;

pub struct Ui {
    fps: three::Text,
}

impl Ui {
    pub fn new(window: &mut three::Window) -> Self {
        let font = window.factory.load_font(format!(
            "{}/assets/UbuntuMono-R.ttf",
            env!("CARGO_MANIFEST_DIR")
        ));

        let mut fps = window.factory.ui_text(&font, "FPS: 0.0");

        fps.set_font_size(25.0);
        fps.set_layout(three::Layout::SingleLine(three::Align::Left));
        fps.set_color(FONT_COLOR);

        let (width, height): (u32, u32) = match window.glutin_window().get_inner_size() {
            None => panic!("Window doesn't exist"),
            Some(size) => size.into(),
        };

        window.scene.add(&fps);

        Self { fps }
    }

    pub fn update(&mut self, delta: f32) {
        self.fps.set_text(format!("FPS: {:.2}", 1.0 / delta));
    }
}
