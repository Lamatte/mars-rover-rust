use crate::rover::{Position, Orientation, Rover};

pub struct Mars {
    size: u32,
    obstacles: Vec<Position>,
}

pub trait MarsRenderer {
    fn render(&self, mars: &Mars, rover: &Rover) -> String;
}

pub struct SimpleMarsRenderer {}

impl Mars {
    pub fn new(size: u32) -> Mars {
        Mars{size: size, obstacles: vec!()}
    }

    pub fn add_obstacle(&mut self, position: Position) {
        self.obstacles.push(position);
    }

    pub fn has_obstacle(&self, position: Position) -> bool {
        self.obstacles.contains(&position)
    }
}

impl SimpleMarsRenderer {
    pub fn new() -> SimpleMarsRenderer {
        SimpleMarsRenderer{}
    }

    fn render(&self, mars: &Mars, rover: &Rover) -> String {
        let mut to_string = "".to_string();
        for _y in (0..mars.size).rev() {
            for _x in 0..mars.size {
                to_string = to_string + self.render_cell(Position{x: _x as i32, y: _y as i32}, mars, rover);
            }
            to_string = to_string + "\n";
        }
        to_string
    }

    fn render_cell(&self, position: Position, mars: &Mars, rover: &Rover) -> &str {
        let mut to_string = ".";
        if position == rover.position {
            to_string = self.render_rover(&rover);
        } else if mars.has_obstacle(position) {
            to_string = "X";
        }
        to_string
    }

    fn render_rover(&self, rover: &Rover) -> &str {
        match rover.orientation {
            Orientation::North => "^",
            Orientation::East => ">",
            Orientation::West => "<",
            Orientation::South => "v",
        }
    }
}

impl MarsRenderer for SimpleMarsRenderer {
    fn render(&self, mars: &Mars, rover: &Rover) -> String {
        self.render(mars, rover)
    }
}

#[test]
fn shall_display_mars_with_obstacles() {
    // Given
    let mut mars = Mars::new(3);
    mars.add_obstacle(Position{x: 1, y: 1});
    let rover = Rover::new(Orientation::North);
    // When
    let representation = SimpleMarsRenderer::new().render(&mars, &rover);
    // Then
    assert_eq!("...\n.X.\n^..\n", representation);
}

#[test]
fn shall_display_mars_with_rover() {
    // Given
    let mars = Mars::new(3);
    let rover = Rover::new(Orientation::North);
    // When
    let representation = SimpleMarsRenderer::new().render(&mars, &rover);
    // Then
    assert_eq!("...\n...\n^..\n", representation);
}
