use nannou::prelude::*;

mod model;
use model::{Model, Player};

const GRID_SIZE: usize = 3;
const CELL_SIZE: f32 = 100.0;

fn main() {
    nannou::app(model::model).update(model::update).run();
}
