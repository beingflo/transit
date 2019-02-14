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
    let camera = window.factory.perspective_camera(60.0, 0.01 .. 100.0);

    let mut control = Control::new(&camera);
    let mut ui = Ui::new(&mut window);

    window.scene.background = three::Background::Color(BACKGROUND);

    let mesh = util::create_quad(&mut window);
    let mesh2 = window.factory.mesh_instance(&mesh);

    mesh.set_scale(0.1);
    mesh2.set_scale(0.1);

    window.scene.add(&mesh2);

    while window.update() && !control.should_quit() {
        control.handle(&window);

        ui.update(&window);

        if control.toggle_fullscreen() {
            window.toggle_fullscreen();
        }

        window.render(&camera);
    }
}
