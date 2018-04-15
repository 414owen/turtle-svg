extern crate getopts;
mod gen;
use getopts::Options;
use std::ops::Rem;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();

    opts.optopt("i",
                "iterations",
                "set iterations (aka the depth of branches to produce",
                "INT");
    opts.optopt("a",
                "angle",
                "set the angle (in degrees) between branches",
                "FLOAT");
    opts.optopt("b",
                "branches",
                "set the branching factor of the tree (the number of branches per level of the \
                 tree",
                "INT");
    opts.optopt("l",
                "length",
                "set the length of the trunk (branch lengths are calculated from this)",
                "FLOAT");
    opts.optopt("r",
                "ratio",
                "set the ratio of one branch to another",
                "FLOAT");
    opts.optflag("c", "color", "enable colorization");
    opts.optopt("", "branch-color", "set branch colour", "COL");
    opts.optopt("", "leaf-color", "set leaf color", "COL");
    opts.optopt("p",
                "starting-point",
                "set the starting point of the tree",
                "X,Y");
    opts.optopt("s", "skew", "set the amount to skew tree to the right", "DEG");
    opts.optflag("h", "help", "print out usage information");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("help") {
        print_usage(&args[0].clone(), opts);
        return;
    }
    let iterations = match matches.opt_str("i") {
        Some(n) => n.parse::<i32>().unwrap(),
        _ => 6,
    };
    let angle = match matches.opt_str("a") {
        Some(n) => n.parse::<f64>().unwrap(),
        _ => 30.0,
    };
    let branches = match matches.opt_str("b") {
        Some(n) => n.parse::<i32>().unwrap(),
        _ => 4,
    };
    let length = match matches.opt_str("l") {
        Some(n) => n.parse::<f64>().unwrap(),
        _ => 50.0,
    };
    let ratio = match matches.opt_str("r") {
        Some(n) => n.parse::<f64>().unwrap(),
        _ => 0.8,
    };
    let color = matches.opt_present("c");
    let starting_color: String = match matches.opt_str("branch-color") {
        Some(s) => s,
        _ => "#000".to_string(),
    };
    let final_color: String = match matches.opt_str("leaf-color") {
        Some(s) => s,
        _ => "#000".to_string(),
    };
    let skew = match matches.opt_str("s") {
        Some(n) => n.parse::<f64>().unwrap(),
        _ => 0.0,
    };
    match matches.opt_str("p") {
        Some(p) => {
            let mut args = p.split(",");
            let x = args.next().unwrap().parse::<f64>().unwrap();
            let y = args.next().unwrap().parse::<f64>().unwrap();
            gen::set_position(x, y);
        }
        _ => (),
    }
    let total_angle = angle * (branches - 1) as f64;
    let half_angle = total_angle / 2.0;

    gen::left_turn(90.0);
    branch_me(length,
              90.0,
              skew,
              1,
              iterations,
              branches,
              angle,
              total_angle,
              half_angle,
              ratio,
              color,
              &starting_color,
              &final_color);
}

#[inline(always)]
fn branch_me(length: f64,
             bearing: f64,
             skew: f64,
             iteration: i32,
             max: i32,
             branches: i32,
             angle: f64,
             total_angle: f64,
             half_angle: f64,
             ratio: f64,
             color: bool,
             starting_color: &str,
             final_color: &str) {
    if color {
        if iteration == max {
            gen::pen_color(final_color);
        } else {
            gen::pen_color(starting_color);
        }
    }
    let mut bearing = if bearing > 0.0 {bearing.rem(360.0)} else {(360.0 + bearing.rem(360.0)).rem(360.0)};
    let skew_ang = skew * bearing.to_radians().sin();
    gen::right_turn(skew_ang);
    bearing -= skew_ang;
    gen::forward(length);
    if iteration < max {
        gen::left_turn(half_angle);
        bearing += half_angle;
        for _ in 0..branches {
            branch_me(length * ratio,
                      bearing - skew_ang,
                      skew,
                      iteration + 1,
                      max,
                      branches,
                      angle,
                      total_angle,
                      half_angle,
                      ratio,
                      color,
                      starting_color,
                      final_color);
            gen::right_turn(angle);
            bearing -= angle;
        }
        gen::left_turn(angle);
        gen::left_turn(half_angle);
    }
    gen::pen_up();
    gen::left_turn(180.0);
    gen::forward(length);
    gen::pen_down();
    gen::left_turn(180.0);
    gen::left_turn(skew_ang);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]",
                        program);
    print!("{}", opts.usage(&brief));
}
