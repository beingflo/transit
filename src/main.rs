mod control;
mod map;
mod ui;
mod util;

use three;

use control::Control;
use map::Map;
use ui::Ui;

// White
const BACKGROUND: u32 = 0xFFFFFF;

fn main() {
    let mut window = three::Window::builder("Transit").multisampling(8).build();
    let camera = window.factory.perspective_camera(90.0, 0.01..);

    let mut control = Control::new(&camera);
    let mut ui = Ui::new(&mut window);

    window.scene.background = three::Background::Color(BACKGROUND);

    let mut map = Map::new(&mut window);

    while window.update() && !control.should_quit() {
        control.handle(&window);
        map.update();

        ui.update(&window);

        if control.toggle_fullscreen() {
            window.toggle_fullscreen();
        }

        window.render(&camera);
    }
}
