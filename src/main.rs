mod util;
mod ui;

use three;

use ui::Ui;

use three::object::Object;

// White
const BACKGROUND: u32 = 0xFFFFFF;

fn main() {
    let mut window = three::Window::builder("Transit").multisampling(8).build();
    let camera = window.factory.orthographic_camera([0.0, 0.0], 1.0, -1.0 .. 1.0);

    let mut ui = Ui::new(&mut window);

    let mesh = util::create_quad(&mut window);
    let mesh2 = window.factory.mesh_instance(&mesh);

    mesh.set_scale(0.1);
    mesh2.set_scale(0.1);

    window.scene.add(&mesh2);

    window.scene.background = three::Background::Color(BACKGROUND);

    while window.update() && !window.input.hit(three::KEY_ESCAPE) {
        if window.input.keys_hit().contains(&three::controls::Key::F11) {
            window.toggle_fullscreen();
        }

        let delta = window.input.delta_time();
        ui.update(delta);


        window.render(&camera);
    }
}
