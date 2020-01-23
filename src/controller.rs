use crate::rover::Rover;
use crate::mars::{Mars, MarsRenderer};

pub struct Controller {
    rover: Rover,
    mars: Mars,
    renderer: Box<dyn MarsRenderer>,
}

impl Controller {
    pub fn new(rover: Rover, mars: Mars, renderer: Box<dyn MarsRenderer>) -> Controller {
        Controller { rover: rover, mars: mars, renderer: renderer }
    }

    pub fn execute_commands(&self, commands: String) -> Rover {
        let mut commands_chars = commands.chars();
        let mut current_rover = self.rover;
        println!("{}", self.renderer.render(&self.mars, &current_rover));
        while let Some(command) = commands_chars.next() {
            println!("Executing command {}", command);
            current_rover = self.try_and_execute_command(&self.mars, current_rover, command);
            println!("{}", self.renderer.render(&self.mars, &current_rover));
        }
        current_rover
    }

    fn try_and_execute_command(&self, mars: &Mars, rover: Rover, command: char) -> Rover {
        let new_rover = self.execute_command(command, rover);
        if mars.has_obstacle(new_rover.position) {
            println!("/!\\ Invalid move requested, skipping");
            return rover;
        } else {
            return new_rover;
        }
    }

    pub fn execute_command(&self, command: char, rover: Rover) -> Rover {
        match command {
            'F' => rover.move_forward(),
            'B' => rover.move_backward(),
            'R' => rover.turn_right(),
            'L' => rover.turn_left(),
            _ => rover
        }
    }
}

pub struct MockMarsRenderer {}

impl MarsRenderer for MockMarsRenderer {
    fn render(&self, _mars: &Mars, _rover: &Rover) -> String {
        "".to_string()
    }
}

#[test]
fn shall_control_rover() {
    // Given
    let mut rover = Rover::new(crate::rover::Orientation::North);
    let mut mars = Mars::new(5);
    mars.add_obstacle(crate::rover::Position { x: 1, y: 1 });
    let renderer = MockMarsRenderer{};
    let controller = Controller::new(rover, mars, Box::new(renderer));
    // When
    rover = controller.execute_commands("FFBFRFLLLF".to_string());
    // Then
    assert_eq!(crate::rover::Position { x: 1, y: 2 }, rover.position);
    assert_eq!(crate::rover::Orientation::South, rover.orientation);
}

