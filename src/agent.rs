use three::Mesh;
use three::object::Object;

use cgmath::{Rad, Euler, Quaternion, Vector2};

use super::util;
use super::map::MAX_SPAWN;

const AGENT_SIZE: f32 = 0.1;
const MAX_VEL: f32 = 1.0;
const MAX_RANGE: f32 = 2.0;

#[derive(Clone)]
pub struct Agent {
    position: Vector2<f32>,

    velocity: Vector2<f32>,

    range: f32,
    vel: f32,

    mesh: Mesh,
}

impl Agent {
    pub fn new(position: Vector2<f32>, velocity: Vector2<f32>, range: f32, vel: f32, mesh: Mesh) -> Self {
        Self {
            position,
            velocity,
            range,
            vel,
            mesh,
        }
    }

    pub fn new_random(mesh: Mesh) -> Self {
        let (x, y) = util::random_tuple(MAX_SPAWN, MAX_SPAWN);
        let (x_vel, y_vel) = util::random_tuple(1.0, 1.0);

        let range = util::random_in_range(0.0, MAX_RANGE);
        let vel = util::random_in_range(0.0, MAX_VEL);

        Agent::new(Vector2::new(x,y), Vector2::new(x_vel, y_vel), range, vel, mesh)
    }

    pub fn new_random_from_template(template: &Mesh, window: &mut three::Window) -> Self {
        let mesh = window.factory.mesh_instance(&template);
        mesh.set_scale(AGENT_SIZE);
        window.scene.add(&mesh);

        Agent::new_random(mesh)
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * self.vel * dt;
    }

    pub fn draw(&self) {
        self.mesh.set_position([self.position.x, self.position.y, 0.0]);
        let rot = Quaternion::from(Euler::new(Rad(0.0), Rad(0.0), Rad((self.velocity.y / self.velocity.x).atan() - std::f32::consts::PI / 2.0)));
        self.mesh.set_orientation(rot);
    }
}
