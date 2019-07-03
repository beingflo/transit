use three::Mesh;
use three::object::Object;

use cgmath::{Rad, Euler, Quaternion, Vector2};
use cgmath::{MetricSpace, InnerSpace};

use super::util;
use super::food::Food;
use super::map::MAX_SPAWN;

const AGENT_SIZE: f32 = 0.1;
const INIT_ENERGY: f32 = 10.0;
const MAX_VEL: f32 = 1.0;
const MAX_RANGE: f32 = 1.0;

const ACCEL: f32 = 0.3;

#[derive(Clone)]
pub struct Agent {
    pub position: Vector2<f32>,
    pub energy: f32,

    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,

    pub range: f32,
    pub vel: f32,

    mesh: Mesh,
    range_mesh: Mesh,
}

impl Agent {
    pub fn new(position: Vector2<f32>, velocity: Vector2<f32>, acceleration: Vector2<f32>, range: f32, vel: f32, mesh: Mesh, range_mesh: Mesh) -> Self {
        range_mesh.set_scale(range);
        Self {
            position,
            energy: INIT_ENERGY,
            velocity,
            acceleration,
            range,
            vel,
            mesh,
            range_mesh,
        }
    }

    pub fn new_random(mesh: Mesh, range_mesh: Mesh) -> Self {
        let (x, y) = util::random_tuple(MAX_SPAWN, MAX_SPAWN);
        let (x_vel, y_vel) = util::random_tuple(1.0, 1.0);

        let range = util::random_in_range(0.0, MAX_RANGE);
        let vel = util::random_in_range(0.0, MAX_VEL);

        Agent::new(Vector2::new(x,y), Vector2::new(x_vel, y_vel), Vector2::new(0.0, 0.0), range, vel, mesh, range_mesh)
    }

    pub fn new_random_from_template(agent: &Mesh, range: &Mesh, window: &mut three::Window) -> Self {
        let mesh = window.factory.mesh_instance(&agent);
        mesh.set_scale(AGENT_SIZE);
        window.scene.add(&mesh);

        let range = window.factory.mesh_instance(&range);
        window.scene.add(&range);

        Agent::new_random(mesh, range)
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.normalize();
        self.position += self.velocity * self.vel * dt;

        self.energy -= dt;
    }

    pub fn draw(&self, draw_range: bool) {
        self.mesh.set_position([self.position.x, self.position.y, 0.0]);
        let rot = Quaternion::from(Euler::new(Rad(0.0), Rad(0.0), Rad((self.velocity.y / self.velocity.x).atan() - std::f32::consts::PI / 2.0)));
        self.mesh.set_orientation(rot);

        if draw_range {
            self.range_mesh.set_position([self.position.x, self.position.y, 0.0]);
        }
    }

    pub fn nearest_food(&self, food: &Vec<Food>) -> usize {
        let mut min_dist = std::f32::MAX;
        let mut min_idx = 0;

        for (i, f) in food.iter().enumerate() {
            let dist = self.position.distance(f.position);
            if dist < min_dist {
                min_dist = dist;
                min_idx = i;
            }
        }

        min_idx
    }

    pub fn accelerate_towards(&mut self, target: &Vector2<f32>) {
        let diff = target - self.position;

        self.acceleration = diff.normalize() * ACCEL;
    }

    pub fn remove(&self, window: &mut three::Window) {
        window.scene.remove(&self.mesh);
        window.scene.remove(&self.range_mesh);
    }
}
