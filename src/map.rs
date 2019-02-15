use three;
use three::object::Object;

use cgmath::{Deg, Euler, Quaternion};

use crate::util;

const GRID_SIZE: i32 = 5;

pub struct Map {
    buildings: Vec<Building>,
    roads: Vec<Road>,
    transporters: Vec<Transporter>,
}

impl Map {
    pub fn new(window: &mut three::Window) -> Self {
        let building_template = util::create_quad(window, [1.0, 1.0]);
        let transporter_template = util::create_quad(window, [0.5, 1.0]);

        let mut buildings = Vec::new();
        let mut roads = Vec::new();
        let mut transporters = Vec::new();

        for i in -GRID_SIZE .. GRID_SIZE {
            for j in -GRID_SIZE .. GRID_SIZE {
                let building = window.factory.mesh_instance(&building_template);
                building.set_scale(0.45);
                building.set_position([i as f32, j as f32, 0.0]);

                buildings.push(Building::new([i,j]));

                window.scene.add(&building);
            }
        }

        for i in -GRID_SIZE .. GRID_SIZE {
            for j in -GRID_SIZE .. GRID_SIZE {
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

        Self { buildings, roads, transporters }
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

struct Road {
    begin: [f32; 2],
    end: [f32; 2],

    load: u32,
}

impl Road {
    fn new(begin: [f32; 2], end: [f32; 2]) -> Self {
        Self { begin, end, load: 0 }
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
