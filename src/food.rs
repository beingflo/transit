use three::Mesh;
use three::object::Object;

use cgmath::Vector2;

use super::util;
use super::map::MAX_SPAWN;

const FOOD_SIZE: f32 = 0.05;

#[derive(Clone)]
pub struct Food {
    pub position: Vector2<f32>,
    pub energy: f32,

    mesh: Mesh,
}

impl Food {
    pub fn new(position: Vector2<f32>, energy: f32, mesh: Mesh) -> Self {
        Food { position, energy, mesh }
    }

    pub fn new_random(mesh: Mesh) -> Self {
        let (x, y) = util::random_tuple(MAX_SPAWN, MAX_SPAWN);
        let e = rand::random::<f32>();

        Food::new(Vector2::new(x,y), e, mesh)
    }

    pub fn new_random_from_template(template: &Mesh, window: &mut three::Window) -> Self {
        let mesh = window.factory.mesh_instance(&template);
        mesh.set_scale(FOOD_SIZE);
        window.scene.add(&mesh);

        Food::new_random(mesh)
    }

    pub fn update(&mut self, _dt: f32) {

    }

    pub fn draw(&self) {
        self.mesh.set_position([self.position.x, self.position.y, 0.0]);
    }
}

