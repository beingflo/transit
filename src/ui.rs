use three;

// Black
const FONT_COLOR: u32 = 0x000000;

const UPDATE_INTERVAL: f32 = 1.0;
const INITIAL_UPDATE: f32 = 0.1;
const FONT_SIZE: f32 = 17.0;

pub struct Ui {
    fps: three::Text,
    time: f32,
    last: f32,

    size: (f32, f32, f32),
}

impl Ui {
    pub fn new(window: &mut three::Window) -> Self {
        let font = window.factory.load_font(format!(
            "{}/assets/UbuntuMono-R.ttf",
            env!("CARGO_MANIFEST_DIR")
        ));

        let mut fps = window.factory.ui_text(&font, "00.00");

        let size = Ui::get_window_size(window);

        fps.set_font_size(FONT_SIZE * size.2);
        fps.set_layout(three::Layout::SingleLine(three::Align::Right));
        fps.set_color(FONT_COLOR);

        fps.set_pos([size.0, 0.0]);

        window.scene.add(&fps);

        Self { fps, time: 0.0, last: 0.0, size }
    }

    pub fn update(&mut self, window: &three::Window) {
        let delta = window.input.delta_time();
        let size = Ui::get_window_size(window);

        if size != self.size {
            self.fps.set_pos([size.0, 0.0]);
            self.size = size;
        }

        if self.time < INITIAL_UPDATE || self.time - self.last >= UPDATE_INTERVAL {
            self.fps.set_text(format!("{:.2}", 1.0 / delta));
            self.last = self.time;
        }

        self.time += delta;
    }

    fn get_window_size(window: &three::Window) -> (f32, f32, f32) {
        let (width, height): (u32, u32) = match window.glutin_window().get_inner_size() {
            None => panic!("Window doesn't exist"),
            Some(size) => size.into(),
        };

        let hidpi = window.glutin_window().get_hidpi_factor() as f32;

        (width as f32 * hidpi, height as f32 * hidpi, hidpi)
    }
}
