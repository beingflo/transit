use super::agent::Agent;
use super::food::Food;

use cgmath::MetricSpace;

use super::util;
use super::timer::Timer;

pub const MAX_SPAWN: f32 = 50.0;
const EATING_RANGE: f32 = 0.1;
pub const INIT_ENERGY: f32 = 10.0;
pub const BUDGET: f32 = 3.0;
pub const FOOD_ENERGY: f32 = 2.0;
pub const REPRO_RATE: f32 = 1.0;
const FOOD_RATE: u32 = 10;

#[derive(Clone)]
pub struct Map {
    agents: Vec<Agent>,
    food: Vec<Food>,

    population_timer: Timer,

    agent_template: three::Mesh,
    range_template: three::Mesh,
    food_template: three::Mesh,
}

impl Map {
    pub fn new(window: &mut three::Window, num_agent: u32) -> Self {
        let material = three::material::Line { color: 0x000000 };
        let quad = three::Geometry::cuboid(1.0, 2.0, 0.5);

        let agent_template = window.factory.mesh(quad, material);
        let range_template = util::create_circle(window, 12);

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
        for _ in 0..1000 {
            for _ in 0..FOOD_RATE {
                food.push(Food::new_random_from_template(&food_template, window));
            }
        }

        Self {
            agents,
            food,

            population_timer: Timer::new(1.0),

            agent_template,
            range_template,
            food_template,
        }
    }

    pub fn update(&mut self, window: &mut three::Window, dt: f32) {
        if self.population_timer.tick(dt) {
            println!("Agents: {}\t Food: {}", self.agents.len(), self.food.len());
        }

        let mut agent_idx = 0;
        while agent_idx < self.agents.len() {
            let agent = &mut self.agents[agent_idx];
            let food_idx = agent.nearest_food(&self.food);
            let dist = agent.position.distance(self.food[food_idx].position);

            if dist < agent.range {
                agent.accelerate_towards(&self.food[food_idx].position);
            }

            if dist < EATING_RANGE {
                agent.energy += self.food[food_idx].energy;
                self.food[food_idx].remove(window);
                self.food.remove(food_idx);
            }

            let spawn = agent.update(dt);

            if spawn {
                let mut offspring = Agent::new_from_copy(agent, &self.agent_template, &self.range_template, window);
                agent.energy -= INIT_ENERGY;

                let mutation = util::random_in_range(0.9, 1.1);
                offspring.fertility *= mutation;
                if offspring.fertility > 1.0 {
                    offspring.fertility = 1.0;
                }

                let mutation = util::random_in_range(-0.1, 0.1);
                if mutation >= 0.0 && offspring.range >= mutation {
                    offspring.vel += mutation;
                    offspring.range -= mutation;
                }
                if mutation < 0.0 && offspring.vel >= mutation {
                    offspring.vel += mutation;
                    offspring.range -= mutation;
                }


                self.agents.push(offspring);
            }


            if self.agents[agent_idx].energy <= 0.0 {
                self.agents[agent_idx].remove(window);
                self.agents.remove(agent_idx);
            }

            agent_idx += 1;
        }

        for f in self.food.iter_mut() {
            f.update(dt);
        }

        for _ in 0..FOOD_RATE {
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
