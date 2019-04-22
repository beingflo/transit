use rand;

use three;
use three::object::Object;

use cgmath::{Rad, Euler, Quaternion, Vector2};
use cgmath::prelude::*;

const AGENT_SIZE: f32 = 0.1;
const AGENT_SPEED: f32 = 0.01;
const MAX_SPAWN: f32 = 10.0;

#[derive(Clone)]
pub struct Map {
    agents: Vec<Agent>,
    agent_meshes: Vec<three::Mesh>,
}

impl Map {
    pub fn new(window: &mut three::Window, num_agent: u32) -> Self {
        let material = three::material::Line { color: 0x000000 };
        let quad = three::Geometry::cuboid(1.0, 2.0, 0.5);

        let agent_template = window.factory.mesh(quad, material);

        // Set up Agents
        let mut agents = Vec::new();
        let mut agent_meshes = Vec::new();
        for _ in 0..num_agent {
            let mesh = window.factory.mesh_instance(&agent_template);

            mesh.set_scale(AGENT_SIZE);
            window.scene.add(&mesh);

            let x = rand::random::<f32>() * 2.0 * MAX_SPAWN - MAX_SPAWN;
            let y = rand::random::<f32>() * 2.0 * MAX_SPAWN - MAX_SPAWN;

            let deg = rand::random::<f32>() * 2.0 * std::f32::consts::PI;

            agents.push(Agent::new(Vector2::new(x,y), deg));
            agent_meshes.push(mesh);
        }

        Self {
            agents,
            agent_meshes,
        }
    }

    pub fn update(&mut self) {
        let agents_copy = self.agents.clone();

        for i in 0..1 {
            let mut min_distance = std::f32::MAX;
            let mut min_distance_id = 0;
            let agenti = &agents_copy[i];

            for j in i..agents_copy.len() {
                if j == i {
                    continue;
                }
                let agentj = &agents_copy[j];
                let distance = agenti.position.distance(agentj.position);
                if distance < min_distance {
                    min_distance = distance;
                    min_distance_id = j;
                }
            }
        }

        for a in self.agents.iter_mut() {
            a.update();
        }
    }

    pub fn draw(&self) {
        for (agent, mesh) in self.agents.iter().zip(self.agent_meshes.iter()) {
            mesh.set_position([agent.position.x, agent.position.y, 0.0]);
            let rot = Quaternion::<f32>::from(Euler::new(Rad(0.0), Rad(0.0), Rad(-agent.rotation)));
            mesh.set_orientation(rot);
        }
    }
}

#[derive(Clone)]
struct Agent {
    position: Vector2<f32>,
    rotation: f32,
}

impl Agent {
    fn new(position: Vector2<f32>, rotation: f32) -> Self {
        Self {
            position,
            rotation,
        }
    }

    fn update(&mut self) {
        self.position.x += AGENT_SPEED * self.rotation.sin();
        self.position.y += AGENT_SPEED * self.rotation.cos();
    }
}
