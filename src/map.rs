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
        let building_template = util::create_quad(window, [1.0, 1.0]);
        let transporter_template = util::create_quad(window, [0.5, 1.0]);

        let mut buildings = Vec::new();
        let mut transporters = Vec::new();

        for i in -10..10 {
            for j in -10..10 {
                let building = window.factory.mesh_instance(&building_template);
                building.set_scale(0.45);
                building.set_position([i as f32, j as f32, 0.0]);

                buildings.push(Building::new([i,j]));

                window.scene.add(&building);
            }
        }

        for i in -10..10 {
            for j in -10..10 {
                let transporter = window.factory.mesh_instance(&transporter_template);

                transporter.set_scale(0.02);
                let pos = [i as f32 - 0.462, j as f32 - 0.0];
                transporter.set_position([pos[0], pos[1], 0.0]);

                window.scene.add(&transporter);

                transporters.push(Transporter::new(transporter, pos));
                let transporter = window.factory.mesh_instance(&transporter_template);

                transporter.set_scale(0.02);
                let pos = [i as f32 - 0.486, j as f32 - 0.0];
                transporter.set_position([pos[0], pos[1], 0.0]);

                window.scene.add(&transporter);

                transporters.push(Transporter::new(transporter, pos));
                let transporter = window.factory.mesh_instance(&transporter_template);

                transporter.set_scale(0.02);
                let pos = [i as f32 - 0.514, j as f32 - 0.0];
                transporter.set_position([pos[0], pos[1], 0.0]);

                window.scene.add(&transporter);

                transporters.push(Transporter::new(transporter, pos));
                let transporter = window.factory.mesh_instance(&transporter_template);

                transporter.set_scale(0.02);
                let pos = [i as f32 - 0.538, j as f32 - 0.0];
                transporter.set_position([pos[0], pos[1], 0.0]);

                window.scene.add(&transporter);

                transporters.push(Transporter::new(transporter, pos));
            }
        }

        Self { buildings, transporters }
    }

    pub fn update(&mut self) {
        for t in self.transporters.iter_mut() {
            t.update();
        }
    }
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

struct Item {
    value: u32,
}

impl Item {
    fn new(value: u32) -> Self {
        Self { value }
    }
}
