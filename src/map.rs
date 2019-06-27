use rand;

use three;
use three::object::Object;

use cgmath::{Rad, Euler, Quaternion, Vector2};

const AGENT_SIZE: f32 = 0.1;
const DT: f32 = 0.01;
const MAX_SPAWN: f32 = 20.0;
const MAX_VEL: f32 = 1.0;

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

            let velocityx = rand::random::<f32>() * 2.0 * MAX_VEL - MAX_VEL;
            let velocityy = rand::random::<f32>() * 2.0 * MAX_VEL - MAX_VEL;

            agents.push(Agent::new(Vector2::new(x,y), Vector2::new(velocityx, velocityy)));
            agent_meshes.push(mesh);
        }

        Self {
            agents,
            agent_meshes,
        }
    }

    pub fn update(&mut self) {
        for a in self.agents.iter_mut() {
            a.update();
        }
    }

    pub fn draw(&self) {
        for (agent, mesh) in self.agents.iter().zip(self.agent_meshes.iter()) {
            mesh.set_position([agent.position.x, agent.position.y, 0.0]);
            let rot = Quaternion::from(Euler::new(Rad(0.0), Rad(0.0), Rad((agent.velocity.y/agent.velocity.x).atan() - std::f32::consts::PI / 2.0)));
            mesh.set_orientation(rot);
        }
    }
}

#[derive(Clone)]
struct Agent {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
}

impl Agent {
    fn new(position: Vector2<f32>, velocity: Vector2<f32>) -> Self {
        Self {
            position,
            velocity,
        }
    }

    fn update(&mut self) {
        self.position += self.velocity * DT;
    }
}
