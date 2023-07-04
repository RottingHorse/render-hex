use std::env;
use svg::Document;
use svg::node::element::path::{Command, Position};
use crate::Operation::{Forvard, Home, Noop, TurnLeft, TurnRight};
use crate::Orientation::{North, South, East, West};

const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;
const HOME_X: isize = WIDTH / 2;
const HOME_Y: isize = HEIGHT / 2;


#[derive(Debug, Clone, Copy)]
enum Operation {
    Forvard(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    North,
    East,
    West,
    South,
}

#[derive(Debug)]
struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

impl Artist {
    fn new() -> Artist {
        Artist {
            heading: North,
            x: HOME_X,
            y: HOME_Y,
        }
    }

    fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    fn forward(&mut self, distance: isize) {
        match self.heading {
            North => self.y += distance,
            South => self.y -= distance,
            East => self.x -= distance,
            West => self.x += distance,
        }
    }

    fn turn_right(&mut self) {
        self.heading = match self.heading {
            North => East,
            South => West,
            West => North,
            East => South,
        }
    }

    fn turn_left(&mut self) {
        self.heading = match self.heading {
            North => West,
            South => East,
            West => South,
            East => North,
        }
    }

    fn wrap(&mut self) {
        if self.x < 0 {
            self.x = HOME_X;
            self.heading = West;
        } else if self.x > WIDTH {
            self.x = HOME_X;
            self.heading = East;
        }

        if self.y < 0 {
            self.y = HOME_Y;
            self.heading = North;
        } else if self.y > HEIGHT {
            self.y = HOME_Y;
            self.heading = South;
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = args.get(1).unwrap();
    let default = format!("{}.svg", input);
    let save_to = args.get(2).unwrap_or(&default);

    let operations = parse(input);
    let path_data = convert(&operations);
    let document = generate_svg(path_data);

    svg::save(save_to, &document).unwrap();
}

fn generate_svg(path_data: Vec<Command>) -> Document {
    unimplemented!()
}

fn convert(operations: &Vec<Operation>) -> Vec<Command> {
    let mut turtle = Artist::new();
    let mut path_data: Vec<Command> = vec![];
    let start_at_home = Command::Move(
        Position::Absolute, (HOME_X, HOME_Y).into(),
    );
    path_data.push(start_at_home);

    for op in operations {
        match *op {
            Forvard(distance) => turtle.forward(distance),
            TurnLeft => turtle.turn_left(),
            TurnRight => turtle.turn_right(),
            Home => turtle.home(),
            Noop(byte) => {
                eprintln!("warning: illegal byte encountered: {:?}", byte)
            }
        };
        let line = Command::Line(
            Position::Absolute,
            (turtle.x, turtle.y).into(),
        );
        path_data.push(line);
        turtle.wrap();
    }
    path_data
}

fn parse(input: &str) -> Vec<Operation> {
    let mut steps = Vec::<Operation>::new();
    for byte in input.bytes() {
        let step = match byte {
            b'0' => Home,
            b'1'..=b'9' => {
                let distance = { byte - 0x30 } as isize;
                Forvard(distance * (HEIGHT / 10))
            }
            b'a' | b'b' | b'c' => TurnLeft,
            b'd' | b'e' | b'f' => TurnRight,
            _ => Noop(byte),
        };
        steps.push(step);
    }
    steps
}
