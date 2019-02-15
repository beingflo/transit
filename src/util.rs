use three;

pub fn create_quad(window: &mut three::Window, scale: [f32; 2]) -> three::Mesh {
    let vertices = vec![
        [-1.0, -1.0, 0.0].into(),
        [1.0, -1.0, 0.0].into(),
        [1.0, 1.0, 0.0].into(),
        [-1.0, 1.0, 0.0].into(),
    ];

    let faces = vec![
        [0, 1, 2],
        [2, 3, 0],
    ];

    let mut quad = three::Geometry {
        faces,
        base: three::Shape {
            vertices,
            .. three::Shape::default()
        },
        .. three::Geometry::default()
    };

    for v in quad.base.vertices.iter_mut() {
        v.x *= scale[0];
        v.y *= scale[1];
    }

    let material = three::material::Basic {
        color: 0x000000,
        map: None,
    };

    let mesh = window.factory.mesh(quad, material);

    mesh
}
