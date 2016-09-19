extern crate getopts;
mod gen;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.reqopt("i", "iterations", "set iterations (aka the depth of branches to produce", "INT");
    opts.reqopt("a", "angle", "set the angle (in degrees) between branches", "FLOAT|INT");
    opts.reqopt("b", "branches", "set the branching factor of the tree (the number of branches per level of the tree", "INT");
    opts.reqopt("l", "length", "set the length of the trunk (branch lengths are calculated from this)", "FLOAT|INT");
    opts.reqopt("r", "ratio", "set the ratio of one branch to another", "FLOAT|INT");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };
    let iterations = match matches.opt_str("i") {
        Some(n) => n.parse::<i32>().unwrap(),
        _ => 100
    };
    let angle = match matches.opt_str("a") {
        Some(n) => n.parse::<f64>().unwrap(),
        _ => 20.0
    };
    let branches = match matches.opt_str("b") {
        Some(n) => n.parse::<i32>().unwrap(),
        _ => 4
    };
    let length = match matches.opt_str("l") {
        Some(n) => n.parse::<f64>().unwrap(),
        _ => 50.0
    };
    let ratio = match matches.opt_str("r") {
        Some(n) => n.parse::<f64>().unwrap(),
        _ => 0.6
    };

    gen::left_turn(90.0);
    branch_me(length, 1, iterations, branches, angle, ratio);
}

fn branch_me(length: f64, iteration: i32, max: i32, branches: i32, angle: f64, ratio: f64) {
    gen::forward(length);    
    if iteration < max {
        let total_angle = angle * (branches - 1) as f64;
        let half_angle = total_angle / 2.0;
        gen::left_turn(half_angle);
        for _ in 0..branches {
            branch_me(length * ratio, iteration + 1, max, branches, angle, ratio);
            gen::right_turn(angle);
        }
        gen::left_turn(angle);
        gen::left_turn(half_angle);
    }
    gen::pen_up();
    gen::left_turn(180.0);
    gen::forward(length);
    gen::pen_down();
    gen::left_turn(180.0);
}
