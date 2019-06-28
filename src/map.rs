use rand;

use three;
use three::Mesh;
use three::object::Object;

use cgmath::{Rad, Euler, Quaternion, Vector2};

use super::util;

const AGENT_SIZE: f32 = 0.1;
const FOOD_SIZE: f32 = 0.05;
const DT: f32 = 0.01;
const MAX_SPAWN: f32 = 20.0;
const MAX_VEL: f32 = 1.0;

#[derive(Clone)]
pub struct Map {
    agents: Vec<Agent>,
    food: Vec<Food>,

    food_rate: u32,

    agent_template: three::Mesh,
    food_template: three::Mesh,
}

impl Map {
    pub fn new(window: &mut three::Window, num_agent: u32, food_rate: u32) -> Self {
        let material = three::material::Line { color: 0x000000 };
        let quad = three::Geometry::cuboid(1.0, 2.0, 0.5);

        let agent_template = window.factory.mesh(quad, material);

        // Set up Agents
        let mut agents = Vec::new();
        for _ in 0..num_agent {
            agents.push(Agent::new_random_from_template(&agent_template, window));
        }

        let food_material = three::material::Basic { color: three::color::BLUE, map: None };
        let food_quad = three::Geometry::cuboid(1.0, 1.0, 1.0);

        let food_template = window.factory.mesh(food_quad, food_material);

        // Set up initial food
        // Spawn 5 seconds worth of food
        let mut food = Vec::new();
        for _ in 0..5 {
            for _ in 0..food_rate {
                food.push(Food::new_random_from_template(&food_template, window));
            }
        }

        Self {
            agents,
            food,

            food_rate,

            agent_template,
            food_template,
        }
    }

    pub fn update(&mut self, window: &mut three::Window, tick: u32) {
        for a in self.agents.iter_mut() {
            a.update();
        }

        for f in self.food.iter_mut() {
            f.update();
        }

        if tick % 60 == 0 {
            for _ in 0..self.food_rate {
                self.food.push(Food::new_random_from_template(&self.food_template, window));
            }
        }
    }

    pub fn draw(&self) {
        for agent in self.agents.iter() {
            agent.draw();
        }

        for f in self.food.iter() {
            f.draw();
        }
    }
}

#[derive(Clone)]
struct Food {
    position: Vector2<f32>,
    energy: f32,

    mesh: Mesh,
}

impl Food {
    fn new(position: Vector2<f32>, energy: f32, mesh: Mesh) -> Self {
        Food { position, energy, mesh }
    }

    fn new_random(mesh: Mesh) -> Self {
        let (x, y) = util::random_tuple(MAX_SPAWN, MAX_SPAWN);
        let e = rand::random::<f32>();

        Food::new(Vector2::new(x,y), e, mesh)
    }

    fn new_random_from_template(template: &Mesh, window: &mut three::Window) -> Self {
        let mesh = window.factory.mesh_instance(&template);
        mesh.set_scale(FOOD_SIZE);
        window.scene.add(&mesh);

        Food::new_random(mesh)
    }

    fn update(&mut self) {

    }

    fn draw(&self) {
        self.mesh.set_position([self.position.x, self.position.y, 0.0]);
    }
}

#[derive(Clone)]
struct Agent {
    position: Vector2<f32>,
    velocity: Vector2<f32>,

    mesh: Mesh,
}

impl Agent {
    fn new(position: Vector2<f32>, velocity: Vector2<f32>, mesh: Mesh) -> Self {
        Self {
            position,
            velocity,
            mesh,
        }
    }

    fn new_random(mesh: Mesh) -> Self {
        let (x, y) = util::random_tuple(MAX_SPAWN, MAX_SPAWN);
        let (x_vel, y_vel) = util::random_tuple(MAX_VEL, MAX_VEL);

        Agent::new(Vector2::new(x,y), Vector2::new(x_vel, y_vel), mesh)
    }

    fn new_random_from_template(template: &Mesh, window: &mut three::Window) -> Self {
        let mesh = window.factory.mesh_instance(&template);
        mesh.set_scale(AGENT_SIZE);
        window.scene.add(&mesh);

        Agent::new_random(mesh)
    }

    fn update(&mut self) {
        self.position += self.velocity * DT;
    }

    fn draw(&self) {
        self.mesh.set_position([self.position.x, self.position.y, 0.0]);
        let rot = Quaternion::from(Euler::new(Rad(0.0), Rad(0.0), Rad((self.velocity.y / self.velocity.x).atan() - std::f32::consts::PI / 2.0)));
        self.mesh.set_orientation(rot);
    }
}
