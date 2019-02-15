use three;
use three::object::Object;

use cgmath::{Deg, Euler, Quaternion};

use crate::util;

pub struct Map {
    buildings: Vec<Building>,
    transporters: Vec<Tranporter>,
}

impl Map {
    pub fn new(window: &mut three::Window) -> Self {
        let building = util::create_quad(window, [1.0, 1.0]);

        let mut buildings = Vec::new();
        let transporters = Vec::new();

        for i in -10..10 {
            for j in -10..10 {
                let building = window.factory.mesh_instance(&building);
                building.set_scale(0.4);
                building.set_position([i as f32, j as f32, 0.0]);

                buildings.push(Building::new([i,j]));

                window.scene.add(&building);
            }
        }

        Self { buildings, transporters }
    }
}

struct Building {
    pos: [i32; 2],
    wares: (),
}

impl Building {
    fn new(pos: [i32; 2]) -> Self {
        Self { pos, wares: () }
    }
}

struct Tranporter {
    pos: [f32; 2],
    cargo: (),
}

impl Tranporter {
    fn new(pos: [f32; 2]) -> Self {
        Self { pos, cargo: () }
    }
}
