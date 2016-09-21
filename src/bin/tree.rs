extern crate getopts;
extern crate palette;
mod gen;
use getopts::Options;
use palette::Rgb;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optopt("i", "iterations", "set iterations (aka the depth of branches to produce", "INT");
    opts.optopt("a", "angle", "set the angle (in degrees) between branches", "FLOAT|INT");
    opts.optopt("b", "branches", "set the branching factor of the tree (the number of branches per level of the tree", "INT");
    opts.optopt("l", "length", "set the length of the trunk (branch lengths are calculated from this)", "FLOAT|INT");
    opts.optopt("r", "ratio", "set the ratio of one branch to another", "FLOAT|INT");
    opts.optflag("c", "color", "enable colorization");
    opts.optflag("", "continuous", "enable continuous colour mode");
    opts.optopt("s", "starting-color", "set starting colour", "R,G,B");
    opts.optopt("f", "final-color", "set final color", "R,G,B");
    opts.optflag("h", "help", "print out usage information");


    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };
    if matches.opt_present("help") {
        print_usage(&args[0].clone(), opts);
        return;
    }
    let iterations = match matches.opt_str("i") {
        Some(n) => n.parse::<i32>().unwrap(),
        _ => 6
    };
    let angle = match matches.opt_str("a") {
        Some(n) => n.parse::<f64>().unwrap(),
        _ => 30.0
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
        _ => 0.8
    };
    let color = matches.opt_present("c");
    let continuous = matches.opt_present("continuous");
    let starting_color: Rgb = match matches.opt_str("s") {
        Some(s) => parse_col(&s),
        _ => Rgb::new(0.0,0.0,0.0)
    };
    let final_color: Rgb = match matches.opt_str("f") {
        Some(s) => parse_col(&s),
        _ => Rgb::new(0.0,0.6,0.0)
    };

    gen::left_turn(90.0);
    branch_me(length, 1, iterations, branches, angle, ratio, color, continuous, starting_color, final_color);
}

fn rgb_to_string(col: Rgb) -> String {
    let (red, green, blue) = channels(col);
    format!("rgb({},{},{})", red, green, blue)
}

fn channels(col: Rgb) -> (i32, i32, i32) {
    let (red, green, blue): (f32, f32, f32) = col.to_pixel();
    ((red * 255.0).round() as i32, (green * 255.0).round() as i32, (blue * 255.0).round() as i32)
}

fn parse_col(col_str: &str) -> Rgb {
    let mut channels = col_str.split(',');
    Rgb::new(
        get_chan(channels.next()),
        get_chan(channels.next()),
        get_chan(channels.next())
    )
}

fn get_chan(chan_str: Option<&str>) -> f32 {
    chan_str.expect("Not enough channels supplied in color argument").parse::<f32>().expect("Color channels must be a float between 0.0 and 1.0")
}

#[inline]
fn branch_me(length: f64, iteration: i32, max: i32, branches: i32, angle: f64, ratio: f64, color: bool, continuous: bool, starting_color: Rgb, final_color: Rgb) {
    if color {
        if continuous {
            
        } else {
            if iteration == max {
                gen::pen_color(&rgb_to_string(final_color));
            } else {
                gen::pen_color(&rgb_to_string(starting_color));
            }
        }
    }
    gen::forward(length);    
    if iteration < max {
        let total_angle = angle * (branches - 1) as f64;
        let half_angle = total_angle / 2.0;
        gen::left_turn(half_angle);
        for _ in 0..branches {
            branch_me(length * ratio, iteration + 1, max, branches, angle, ratio, color, continuous, starting_color, final_color);
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
    let brief = format!("Usage: {} [options]\nPlease not that all color channels are between 0.0 and 1.0, for example R,G,B = 0.0,0.4,0.5", program);
    print!("{}", opts.usage(&brief));
}
