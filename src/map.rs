use std::collections::HashMap;

use three;
use three::object::Object;

use cgmath::{Deg, Euler, Quaternion};

use crate::util;

const GRID_SIZE: i32 = 5;
const BLOCK_SIZE: f32 = 0.45;
const CAR_SIZE: f32 = 0.03;
const STEPS_PER_ROAD: u32 = 240;

type Road = (i32, i32, Dir);

pub struct Map {
    buildings: Vec<Building>,
    roads: HashMap<Road, u32>,
    transporters: Vec<Transporter>,
}

impl Map {
    pub fn new(window: &mut three::Window) -> Self {
        let building_template = util::create_quad(window, [1.0, 1.0]);
        let transporter_template = util::create_quad(window, [0.5, 1.0]);


        // Set up buildings
        let mut buildings = Vec::new();
        for i in -GRID_SIZE .. GRID_SIZE {
            for j in -GRID_SIZE .. GRID_SIZE {
                let building = window.factory.mesh_instance(&building_template);
                building.set_scale(BLOCK_SIZE);
                building.set_position([i as f32, j as f32, 0.0]);

                buildings.push(Building::new([i,j]));

                window.scene.add(&building);
            }
        }

        // Set up Roads
        let mut roads = HashMap::new();
        for i in -GRID_SIZE .. GRID_SIZE {
            for j in -GRID_SIZE .. GRID_SIZE {
                roads.insert((i,j, Dir::Left), 0);
                roads.insert((i,j, Dir::Down), 0);
                roads.insert((i,j, Dir::Right), 0);
                roads.insert((i,j, Dir::Up), 0);
            }
        }

        // Set up Transporters
        let mut transporters = Vec::new();
        let transporter = window.factory.mesh_instance(&transporter_template);

        transporter.set_scale(CAR_SIZE);
        window.scene.add(&transporter);

        transporters.push(Transporter::new(transporter, (0,0,Dir::Right)));


        Self { buildings, roads, transporters }
    }

    pub fn update(&mut self) {
        for t in self.transporters.iter_mut() {
            t.update(&mut self.roads);
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Dir {
    Left,
    Down,
    Right,
    Up,
}

struct Building {
    pos: [i32; 2],
    wares: Vec<Item>,
}

impl Building {
    fn new(pos: [i32; 2]) -> Self {
        Self { pos, wares: Vec::new() }
    }
}

struct Transporter {
    mesh: three::Mesh,

    pos: [f32; 2],

    src: [i32; 2],
    target: [i32; 2],

    road: Road,
    steps: u32,

    cargo: Option<Item>,
}

impl Transporter {
    fn new(mesh: three::Mesh, road: Road) -> Self {
        let pos = road_to_spatial(road, 0).0;
        Self { mesh, pos, src: [0, 0], target: [0,0], road, steps: 0, cargo: None }
    }

    fn update(&mut self, roads: &mut HashMap<Road, u32>) {
        self.steps += 1;

        if self.steps >= STEPS_PER_ROAD {
            self.road.1 += 1;
            self.steps = 0;
        }

        let (pos, rot) = road_to_spatial(self.road, self.steps);
        self.mesh.set_position([pos[0], pos[1], 0.0]);
        self.mesh.set_orientation(rot);
    }
}

#[derive(Clone, Debug)]
struct Item {
    value: u32,
}

impl Item {
    fn new(value: u32) -> Self {
        Self { value }
    }
}

fn road_to_spatial(road: (i32, i32, Dir), steps: u32) -> ([f32; 2], Quaternion<f32>) {
    let mut pos = [road.0 as f32, road.1 as f32];

    let deg = match road.2 {
        Dir::Left => {
            pos[0] -= 0.475;
            pos[1] += steps as f32 / STEPS_PER_ROAD as f32 * 2.0 * BLOCK_SIZE - BLOCK_SIZE;

            0.0
        },
        Dir::Down => {
            pos[0] -= steps as f32 / STEPS_PER_ROAD as f32 * 2.0 * BLOCK_SIZE - BLOCK_SIZE;
            pos[1] -= 0.475;

            270.0
        },
        Dir::Right => {
            pos[0] += 0.475;
            pos[1] -= steps as f32 / STEPS_PER_ROAD as f32 * 2.0 * BLOCK_SIZE - BLOCK_SIZE;

            180.0
        },
        Dir::Up => {
            pos[0] += steps as f32 / STEPS_PER_ROAD as f32 * 2.0 * BLOCK_SIZE - BLOCK_SIZE;
            pos[1] += 0.475;

            90.0
        },
    };

    let rot = Quaternion::<f32>::from(Euler::new(Deg(0.0), Deg(0.0), Deg(deg)));

    (pos, rot)
}
