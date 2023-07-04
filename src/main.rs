use std::env;
use svg::Document;
use svg::node::element::path::Command;
use crate::Operation::{Forvard, Home, Noop, TurnLeft, TurnRight};

const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Forvard(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(usize),
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
    unimplemented!()
}

fn parse(input: &str) -> Vec<Operation> {
    let mut steps = Vec::<Operation>::new();
    for byte in input.bytes() {
        let step = match byte {
            b'0' => Home,
            b'1'..=b'9' => {
                let distance = {byte - 0x30} as isize;
                Forvard(distance * (HEIGHT/10))
            },
            b'a'| b'b'|b'c' => TurnLeft,
            b'd'|b'e'|b'f' => TurnRight,
            _ => Noop(byte as usize),
        };
        steps.push(step);
    }
    steps
}
