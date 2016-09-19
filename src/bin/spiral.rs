extern crate getopts;
mod gen;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.reqopt("g", "gap", "gap between lines", "FLOAT|INT");
    opts.reqopt("a", "angle", "set the angle (in degrees) with which to change direction for every side", "FLOAT|INT");
    opts.reqopt("i", "iterations", "set the number of iterations to perform", "INT");
    opts.optopt("n", "node-radius", "set radius of node (vertex) circle", "FLOAT|INT");
     let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };
    let iterations = match matches.opt_str("i") {
        Some(n) => n.parse::<i32>().unwrap(),
        _ => 100
    };
    let mut length = 0.0;
    let mut node = false;
    let node_rad = match matches.opt_str("n") {
        Some(n) => {
            node = true;
            n.parse::<f64>().unwrap()
        },
        _ => 0.0
    };
    let gap = match matches.opt_str("g") {
        Some(n) => n.parse::<f64>().unwrap(),
        _ => 10.0
    };
    let angle = match matches.opt_str("a") {
        Some(n) => n.parse::<f64>().unwrap(),
        _ => 91.0
    };
    for i in 0..iterations {
        gen::left_turn(angle);
        gen::forward(length);
        if node {
            gen::circle(node_rad);
        }
        length = length + gap
    }
}
