use super::agent::Agent;
use super::food::Food;

use cgmath::MetricSpace;

use super::util;

pub const MAX_SPAWN: f32 = 20.0;
const EATING_RANGE: f32 = 0.1;

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
        let range_template = util::create_circle(window, 36);

        // Set up Agents
        let mut agents = Vec::new();
        for _ in 0..num_agent {
            agents.push(Agent::new_random_from_template(&agent_template, &range_template, window));
        }

        let food_material = three::material::Basic { color: three::color::BLUE, map: None };
        let food_quad = three::Geometry::cuboid(1.0, 1.0, 1.0);

        let food_template = window.factory.mesh(food_quad, food_material);

        // Set up initial food
        // Spawn 100 ticks worth of food
        let mut food = Vec::new();
        for _ in 0..100 {
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

    pub fn update(&mut self, window: &mut three::Window, dt: f32) {
        let mut max_energy = 0.0;
        for a in self.agents.iter_mut() {
            let food_idx = a.nearest_food(&self.food);
            let dist = a.position.distance(self.food[food_idx].position);

            if dist < a.range {
                a.accelerate_towards(&self.food[food_idx].position);
            }

            a.update(dt);

            if dist < EATING_RANGE {
                a.energy += self.food[food_idx].energy;
                self.food.remove(food_idx);
            }

            max_energy = if a.energy > max_energy { a.energy } else { max_energy };
        }

        println!("{}", max_energy);

        for f in self.food.iter_mut() {
            f.update(dt);
        }

        for _ in 0..self.food_rate {
            self.food.push(Food::new_random_from_template(&self.food_template, window));
        }
    }

    pub fn draw(&self) {
        for agent in self.agents.iter() {
            agent.draw(true);
        }

        for f in self.food.iter() {
            f.draw();
        }
    }
}
