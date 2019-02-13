mod util;
mod ui;
mod control;

use three;

use ui::Ui;
use control::Control;

use three::object::Object;

// White
const BACKGROUND: u32 = 0xFFFFFF;

fn main() {
    let mut window = three::Window::builder("Transit").multisampling(8).build();
    let camera = window.factory.orthographic_camera([0.0, 0.0], 1.0, -1.0 .. 1.0);

    let mut control = Control::new();
    let mut ui = Ui::new(&mut window);

    window.scene.background = three::Background::Color(BACKGROUND);

    let mesh = util::create_quad(&mut window);
    let mesh2 = window.factory.mesh_instance(&mesh);

    mesh.set_scale(0.1);
    mesh2.set_scale(0.1);

    window.scene.add(&mesh2);

    loop {
        control.handle(&window.input);

        let delta = window.input.delta_time();
        ui.update(delta);

        if control.toggle_fullscreen() {
            window.toggle_fullscreen();
        }

        window.update();
        window.render(&camera);

        if control.should_quit() {
            break;
        }
    }
}
