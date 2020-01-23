use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rover {
    pub position: Position,
    pub orientation: Orientation,
}

impl Rover {
    pub fn new(orientation: Orientation) -> Rover {
        Rover{position: Position{x: 0, y: 0}, orientation: orientation}
    }

    pub fn move_forward(self: &Rover) -> Rover {
        let new_position = match self.orientation {
            Orientation::North => Position{x: self.position.x, y: self.position.y+1},
            Orientation::South => Position{x: self.position.x, y: self.position.y-1},
            Orientation::West => Position{x: self.position.x-1, y: self.position.y},
            Orientation::East => Position{x: self.position.x+1, y: self.position.y},
        };
        Rover{position: new_position, orientation: self.orientation}
    }

    pub fn move_backward(self: &Rover) -> Rover {
        let new_position = match self.orientation {
            Orientation::North => Position{x: self.position.x, y: self.position.y-1},
            Orientation::South => Position{x: self.position.x, y: self.position.y+1},
            Orientation::West => Position{x: self.position.x+1, y: self.position.y},
            Orientation::East => Position{x: self.position.x-1, y: self.position.y},
        };
        Rover{position: new_position, orientation: self.orientation}
    }

    pub fn turn_right(self: &Rover) -> Rover {
        Rover{position: self.position, orientation: self.orientation.right()}
    }

    pub fn turn_left(self: &Rover) -> Rover {
        Rover{position: self.position, orientation: self.orientation.left()}
    }
}

impl fmt::Display for Rover {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}), {:?}", self.position.x, self.position.y, self.orientation)
    }
}

impl Orientation {
    fn right(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        }
    }

    fn left(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::West,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
            Orientation::East => Orientation::North,
        }
    }
}

#[test]
fn shall_turn_right() {
    // Given
    let mut rover = Rover::new(Orientation::North);
    // When
    rover = rover.turn_right();
    // Then
    assert_eq!(Orientation::East, rover.orientation);
    
    // When
    rover = rover.turn_right();
    // Then
    assert_eq!(Orientation::South, rover.orientation);
    
    // When
    rover = rover.turn_right();
    // Then
    assert_eq!(Orientation::West, rover.orientation);
    
    // When
    rover = rover.turn_right();
    // Then
    assert_eq!(Orientation::North, rover.orientation);
}

#[test]
fn shall_turn_left() {
    // Given
    let mut rover = Rover::new(Orientation::North);
    // When
    rover = rover.turn_left();
    // Then
    assert_eq!(Orientation::West, rover.orientation);
    
    // When
    rover = rover.turn_left();
    // Then
    assert_eq!(Orientation::South, rover.orientation);
    
    // When
    rover = rover.turn_left();
    // Then
    assert_eq!(Orientation::East, rover.orientation);
    
    // When
    rover = rover.turn_left();
    // Then
    assert_eq!(Orientation::North, rover.orientation);
}

#[test]
fn shall_move_backward_in_any_direction() {
    // When
    let rover = Rover::new(Orientation::North).move_backward();
    // Then
    assert_eq!(Position{x: 0, y: -1}, rover.position);

    // When
    let rover = Rover::new(Orientation::South).move_backward();
    // Then
    assert_eq!(Position{x: 0, y: 1}, rover.position);
    
    // When
    let rover = Rover::new(Orientation::West).move_backward();
    // Then
    assert_eq!(Position{x: 1, y: 0}, rover.position);
    
    // When
    let rover = Rover::new(Orientation::East).move_backward();
    // Then
    assert_eq!(Position{x: -1, y: 0}, rover.position);
}

#[test]
fn shall_move_forward_in_any_direction() {
    // When
    let rover = Rover::new(Orientation::North).move_forward();
    // Then
    assert_eq!(Position{x: 0, y: 1}, rover.position);

    // When
    let rover = Rover::new(Orientation::South).move_forward();
    // Then
    assert_eq!(Position{x: 0, y: -1}, rover.position);

    // When
    let rover = Rover::new(Orientation::West).move_forward();
    // Then
    assert_eq!(Position{x: -1, y: 0}, rover.position);

    // When
    let rover = Rover::new(Orientation::East).move_forward();
    // Then
    assert_eq!(Position{x: 1, y: 0}, rover.position);

}

#[test]
fn shall_new_rover_be_at_origin() {
    let rover = Rover::new(Orientation::North);
    assert_eq!(Orientation::North, rover.orientation);
    assert_eq!(Position{x: 0, y: 0}, rover.position);
}
