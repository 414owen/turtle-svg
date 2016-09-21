extern crate getopts;
mod gen;
use getopts::Options;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optopt("g", "gap", "set the gap between parallel lines, affects non-parallel lines as well", "FLOAT");
    opts.optopt("a", "angle", "set the angle (in degrees) with which to change direction at every vertex", "FLOAT");
    opts.optopt("i", "iterations", "set the number of iterations to perform", "INT");
    opts.optopt("n", "node-radius", "set radius of node (vertex) circle (default is not to draw a node)", "FLOAT");
    opts.optflag("h", "help", "print usage information");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };
    let iterations = match matches.opt_str("i") {
        Some(n) => n.parse::<i32>().unwrap(),
        _ => 100
    };
    let (node, node_rad) = match matches.opt_str("n") {
        Some(n) => (true, n.parse::<f64>().unwrap()),
        _ => (false, 0.0)
    };
    let gap = match matches.opt_str("g") {
        Some(n) => n.parse::<f64>().unwrap(),
        _ => 3.0
    };
    let angle = match matches.opt_str("a") {
        Some(n) => n.parse::<f64>().unwrap(),
        _ => 91.0
    };
    if matches.opt_present("help") {
        print_usage(args[0].clone(), opts);
        return;
    }
    let mut length = 0.0;
    for i in 0..iterations {
        gen::left_turn(angle);
        gen::forward(length);
        if node {
            gen::circle(node_rad);
        }
        length = length + gap
    }
}

fn print_usage(program: String, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
