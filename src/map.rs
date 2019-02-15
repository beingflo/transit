use std::collections::HashMap;

use three;
use three::object::Object;

use cgmath::{Deg, Euler, Quaternion};

use crate::util;

const GRID_SIZE: i32 = 5;
const BLOCK_SIZE: f32 = 0.45;
const CAR_SIZE: f32 = 0.03;

pub struct Map {
    buildings: Vec<Building>,
    roads: HashMap<(i32, i32, Dir), u32>,
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
        let pos = [- 0.475, 0.0];
        transporter.set_position([pos[0], pos[1], 0.0]);

        window.scene.add(&transporter);

        transporters.push(Transporter::new(transporter, pos));


        Self { buildings, roads, transporters }
    }

    pub fn update(&mut self) {
        for t in self.transporters.iter_mut() {
            t.update();
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
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

    cargo: Option<Item>,
}

impl Transporter {
    fn new(mesh: three::Mesh, pos: [f32; 2]) -> Self {
        Self { mesh, pos, src: [0, 0], target: [0,0], cargo: None }
    }

    fn update(&mut self) {
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
