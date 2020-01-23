mod rover;
mod mars;
mod controller;

use rover::{Position, Orientation, Rover};
use mars::{Mars, SimpleMarsRenderer};
use controller::{Controller, Logger};

struct ConnsoleLogger {}

impl ConnsoleLogger {
    fn new() -> ConnsoleLogger {
        ConnsoleLogger {}
    }
}

impl Logger for ConnsoleLogger {
    fn log(&self, message: String) {
        println!("{}", message);
    }
}

fn main() {
    let rover = Rover::new(Orientation::North);
    let mut mars = Mars::new(5);
    mars.add_obstacle(Position{x: 1, y: 1});
    let renderer = SimpleMarsRenderer::new();
    let logger = ConnsoleLogger::new();
    let controller = Controller::new(rover, mars, Box::new(renderer), Box::new(logger));

    controller.execute_commands("FFBFRFLLLF".to_string());
}
