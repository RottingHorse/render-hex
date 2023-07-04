use std::env;
use svg::Document;
use svg::node::element::path::Command;

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
    unimplemented!()
}
