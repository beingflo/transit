use three;

pub fn create_quad(window: &mut three::Window) -> three::Mesh {
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
    window.scene.add(&mesh);

    mesh
}
