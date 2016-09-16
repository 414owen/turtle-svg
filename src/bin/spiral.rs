extern crate getopts;
mod gen;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.reqopt("g", "gap", "gap between lines", "INT");
    opts.reqopt("a", "angle", "set the angle (in degrees) with which to change direction for every side", "FLOAT");
    opts.reqopt("i", "iterations", "set the number of iterations to perform", "INT");
     let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };
    let end = match matches.opt_str("i") {
        Some(n) => n.parse::<i32>().unwrap(),
        _ => 100
    };
    let mut length = 10;
    let gap = match matches.opt_str("g") {
        Some(n) => n.parse::<i32>().unwrap(),
        _ => 10
    };
    let angle = match matches.opt_str("a") {
        Some(n) => n.parse::<f32>().unwrap(),
        _ => 91.0
    };
    gen::forward(length/2);
    for i in 0..end {
        gen::left_turn(angle);
        gen::forward(length);
        length = length + gap
    }
}
