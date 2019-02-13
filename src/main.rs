use three;

use three::object::Object;

// White
const BACKGROUND: u32 = 0xFFFFFF;

fn main() {
    let mut window = three::Window::builder("Transit").multisampling(8).build();
    let center = [0.0, 0.0];
    let yextent = 1.0;
    let zrange = -1.0 .. 1.0;
    let camera = window.factory.orthographic_camera(center, yextent, zrange);

    let vertices = vec![
        [-0.5, -1.0, 0.0].into(),
        [0.5, -1.0, 0.0].into(),
        [0.5, 1.0, 0.0].into(),
        [-0.5, 1.0, 0.0].into(),
    ];

    let faces = vec![
        [0, 1, 2],
        [2, 3, 0],
    ];

    let quad = three::Geometry {
        faces,
        base: three::Shape {
            vertices,
            .. three::Shape::default()
        },
        .. three::Geometry::default()
    };

    let material = three::material::Basic {
        color: 0x000000,
        map: None,
    };

    let mesh = window.factory.mesh(quad, material);
    mesh.set_scale(0.1);
    window.scene.add(&mesh);

    let font = window.factory.load_font_karla();

    let mut fps_counter = window.factory.ui_text(&font, "FPS: 00");
    fps_counter.set_font_size(20.0);
    fps_counter.set_color(0x000000);

    window.scene.add(&fps_counter);

    for _ in 0..100000 {
        let mesh = window.factory.mesh_instance(&mesh);
        mesh.set_scale(0.1);
        window.scene.add(&mesh);
    }

    window.scene.background = three::Background::Color(BACKGROUND);

    while window.update() && !window.input.hit(three::KEY_ESCAPE) {
        if window.input.keys_hit().contains(&three::controls::Key::F11) {
            window.toggle_fullscreen();
        }
        let delta_time = window.input.delta_time();
        fps_counter.set_text(format!("FPS: {}", 1.0 / delta_time));
        println!("{}", 1.0 / delta_time);
        window.render(&camera);
    }
}
