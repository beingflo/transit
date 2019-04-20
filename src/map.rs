use rand;

use three;
use three::object::Object;

use cgmath::{Rad, Euler, Quaternion};

const AGENT_SIZE: f32 = 0.1;
const AGENT_SPEED: f32 = 0.01;
const MAX_SPAWN: f32 = 100.0;

pub struct Map {
    agents: Vec<Agent>,
}

impl Map {
    pub fn new(window: &mut three::Window, num_agent: u32) -> Self {
        let material = three::material::Line { color: 0x000000 };
        let quad = three::Geometry::cuboid(1.0, 2.0, 0.5);

        let agent_template = window.factory.mesh(quad, material);

        // Set up Agents
        let mut agents = Vec::new();
        for _ in 0..num_agent {
            let agent = window.factory.mesh_instance(&agent_template);

            agent.set_scale(AGENT_SIZE);
            window.scene.add(&agent);

            let x = rand::random::<f32>() * 2.0 * MAX_SPAWN - MAX_SPAWN;
            let y = rand::random::<f32>() * 2.0 * MAX_SPAWN - MAX_SPAWN;

            let deg = rand::random::<f32>() * 2.0 * std::f32::consts::PI;

            agents.push(Agent::new(agent, (x,y), deg));
        }

        Self {
            agents,
        }
    }

    pub fn update(&mut self) {
        for a in self.agents.iter_mut() {
            a.update();
            a.draw();
        }
    }
}

struct Agent {
    mesh: three::Mesh,
    position: (f32, f32),
    rotation: f32,
}

impl Agent {
    fn new(mesh: three::Mesh, position: (f32, f32), rotation: f32) -> Self {
        let mut agent = Self {
            mesh,
            position,
            rotation,
        };
        agent.draw();
        agent
    }

    fn update(&mut self) {
        self.position.0 += AGENT_SPEED * self.rotation.sin();
        self.position.1 += AGENT_SPEED * self.rotation.cos();
    }

    fn draw(&mut self) {
        self.mesh.set_position([self.position.0, self.position.1, 0.0]);
        let rot = Quaternion::<f32>::from(Euler::new(Rad(0.0), Rad(0.0), Rad(-self.rotation)));
        self.mesh.set_orientation(rot);
    }
}
