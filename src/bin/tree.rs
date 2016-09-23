extern crate getopts;
mod gen;
use getopts::Options;

/* TODO: (maybe) Add an option for skewing all angles towards left or right.
 * This will hopefully produce output of a tree swaying in the wind.
 * Would also allow me to produce an animation similar to the spiral one (and reuse
 * pretty much all of the animation script)
 */

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();

    /*
     * Still not entirely sold on getopts. It seems to check for defined parameters
     * (and fail if they don't exist) at runtime. Also, it won't let me define
     * the single-dash notation with more than one character. For example, I would
     * like to be able to specify --branch-color as -bc, but it won't let me. This
     * might be standard, but I have seen it before.
     *
     * TODO: Check the GNU standard for single-dash multiple-character argument
     * specification (see above).
     */

    opts.optopt("i",
                "iterations",
                "set iterations (aka the depth of branches to produce",
                "INT");
    opts.optopt("a",
                "angle",
                "set the angle (in degrees) between branches",
                "FLOAT|INT");
    opts.optopt("b",
                "branches",
                "set the branching factor of the tree (the number of branches per level of the \
                 tree",
                "INT");
    opts.optopt("l",
                "length",
                "set the length of the trunk (branch lengths are calculated from this)",
                "FLOAT|INT");
    opts.optopt("r",
                "ratio",
                "set the ratio of one branch to another",
                "FLOAT|INT");
    opts.optflag("c", "color", "enable colorization");
    opts.optopt("", "branch-color", "set branch colour", "COL");
    opts.optopt("", "leaf-color", "set leaf color", "COL");
    opts.optopt("p",
                "starting-point",
                "set the starting point of the tree",
                "X,Y");
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
    match matches.opt_str("p") {
        Some(p) => {
            let mut args = p.split(",");
            let x = args.next().unwrap().parse::<f64>().unwrap();
            let y = args.next().unwrap().parse::<f64>().unwrap();
            gen::set_position(x, y);
        }
        _ => (),
    }

    gen::left_turn(90.0);
    branch_me(length,
              1,
              iterations,
              branches,
              angle,
              ratio,
              color,
              &starting_color,
              &final_color);
}

#[inline]
fn branch_me(length: f64,
             iteration: i32,
             max: i32,
             branches: i32,
             angle: f64,
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
    gen::forward(length);
    if iteration < max {
        let total_angle = angle * (branches - 1) as f64;
        let half_angle = total_angle / 2.0;
        gen::left_turn(half_angle);
        for _ in 0..branches {
            branch_me(length * ratio,
                      iteration + 1,
                      max,
                      branches,
                      angle,
                      ratio,
                      color,
                      starting_color,
                      final_color);
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

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]\nPlease not that all color channels are between 0.0 \
                         and 1.0, for example R,G,B = 0.0,0.4,0.5",
                        program);
    print!("{}", opts.usage(&brief));
}
