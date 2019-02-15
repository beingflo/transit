use three;
use three::object::Object;

use cgmath::{Deg, Euler, Quaternion};

use crate::util;

pub struct Map {
    buildings: Vec<Building>,
    transporters: Vec<Transporter>,
}

impl Map {
    pub fn new(window: &mut three::Window) -> Self {
        let building = util::create_quad(window, [1.0, 1.0]);
        let transporter = util::create_quad(window, [0.5, 1.0]);

        let mut buildings = Vec::new();
        let mut transporters = Vec::new();

        for i in -10..10 {
            for j in -10..10 {
                let building = window.factory.mesh_instance(&building);
                building.set_scale(0.35);
                building.set_position([i as f32, j as f32, 0.0]);

                buildings.push(Building::new([i,j]));

                window.scene.add(&building);
            }
        }

        for i in -10..10 {
            for j in -10..10 {
                let transporter = window.factory.mesh_instance(&transporter);

                transporter.set_scale(0.1);
                let pos = [i as f32 - 0.45, j as f32 - 0.35];
                transporter.set_position([pos[0], pos[1], 0.0]);

                transporters.push(Transporter::new(pos));

                window.scene.add(&transporter);
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

struct Transporter {
    pos: [f32; 2],
    cargo: (),
}

impl Transporter {
    fn new(pos: [f32; 2]) -> Self {
        Self { pos, cargo: () }
    }
}
