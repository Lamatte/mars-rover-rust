use crate::rover::Rover;
use crate::mars::{Mars, MarsRenderer};

pub trait Logger {
    fn log(&self, message: String);
}

pub struct Controller {
    initial_rover: Rover,
    mars: Mars,
    renderer: Box<dyn MarsRenderer>,
    logger: Box<dyn Logger>,
}

impl Controller {
    pub fn new(rover: Rover, mars: Mars, renderer: Box<dyn MarsRenderer>, logger: Box<dyn Logger>) -> Controller {
        Controller { initial_rover: rover, mars: mars, renderer: renderer, logger: logger }
    }

    pub fn execute_commands(&self, commands: String) -> Rover {
        let mut commands_chars = commands.chars();
        let mut current_rover = self.initial_rover;
        self.logger.log(format!("{}", self.renderer.render(&self.mars, &current_rover)));
        while let Some(command) = commands_chars.next() {
            self.logger.log(format!("Executing command {}", command));
            current_rover = self.try_and_execute_command(command, current_rover);
            self.logger.log(format!("{}", self.renderer.render(&self.mars, &current_rover)));
        }
        current_rover
    }

    fn try_and_execute_command(&self, command: char, rover: Rover) -> Rover {
        let new_rover = self.execute_command(command, rover);
        if self.mars.has_obstacle(new_rover.position) {
            self.logger.log(format!("/!\\ Invalid move requested, skipping"));
            rover
        } else {
            new_rover
        }
    }

    fn execute_command(&self, command: char, rover: Rover) -> Rover {
        match command {
            'F' => rover.move_forward(),
            'B' => rover.move_backward(),
            'R' => rover.turn_right(),
            'L' => rover.turn_left(),
            _ => rover
        }
    }
}

struct MockMarsRenderer {}

impl MarsRenderer for MockMarsRenderer {
    fn render(&self, _mars: &Mars, _rover: &Rover) -> String {
        "".to_string()
    }
}

struct MockLogger {}

impl Logger for MockLogger {
    fn log(&self, _message: String) {
    }
}

#[test]
fn shall_control_rover() {
    // Given
    let mut rover = Rover::new(crate::rover::Orientation::North);
    let mut mars = Mars::new(5);
    mars.add_obstacle(crate::rover::Position { x: 1, y: 1 });
    let renderer = MockMarsRenderer{};
    let logger = MockLogger{};
    let controller = Controller::new(rover, mars, Box::new(renderer), Box::new(logger));
    // When
    rover = controller.execute_commands("FFBFRFLLLF".to_string());
    // Then
    assert_eq!(crate::rover::Position { x: 1, y: 2 }, rover.position);
    assert_eq!(crate::rover::Orientation::South, rover.orientation);
}

