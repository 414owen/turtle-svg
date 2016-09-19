extern crate getopts;
use getopts::Options;
use std::fs::File;
use std::io::{self, Read, Write};
use std::f64::consts::PI;
use std::string::String;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("o", "output", "write svg output to file", "NAME");
    opts.optopt("i", "input", "read turtle script from file", "NAME");
    opts.optopt("w", "width", "set canvas width", "INT");
    opts.optopt("h", "height", "set canvas height", "INT");
    opts.optflag("", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("help") {
        print_usage(&program, opts);
        return;
    }

    init_in(matches);
}

fn init_in(matches: getopts::Matches) {
    match matches.opt_str("i") {
        Some(filename) =>  init_out(File::open(filename).expect("Couldn't open input file."), matches),
        _ => init_out(io::stdin(), matches),
    };
}

fn init_out<R: Read>(mut in_port: R, matches: getopts::Matches) {
    match matches.opt_str("o") {
        Some(filename) => run(in_port, File::create(filename).expect("Couln't open output file"), matches),
        _ => run(in_port, io::stdout(), matches),
    };
}

fn run<R: Read, W: Write>(mut in_port: R, mut out_port: W, matches: getopts::Matches) {
    let mut line_num = 0;
    let width = match matches.opt_str("w") {
        Some(num) => num.parse::<i32>().expect("Cannot parse width"),
        _ => 500
    };
    let height = match matches.opt_str("h") {
        Some(num) => num.parse::<i32>().expect("Cannot parse height"),
        _ => 500
    };
    let mut turtle: Turtle = Turtle {
        position: Point { x: width as f64 / 2.0, y: height as f64 / 2.0 },
        bearing: 0.0f64,
        pen: Pen {
            thickness: 1.0,
            color: "#000".to_string(),
            down: true
        }
    };

    let mut input = String::new();
    in_port.read_to_string(&mut input);
    write!(out_port, "<svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>\n", width, height);
    let mut polyline = false;
    let mut poly_points = Vec::new();
    let NEEDED_NUM = "Expected a number as an argument";
    for line in input.lines() {
        line_num = line_num + 1;
        let mut elems = line.split(' ');
        let cmd = elems.next().expect(&helpful_error(
                      "Every line should start with a command", 
                      line, 
                      line_num
                  ));
        match cmd {

            // Forward
            "fd" => {
                let start = Point { x: turtle.position.x, y: turtle.position.y };
                turtle.position = new_pos(
                    &turtle.position, 
                    turtle.bearing, 
                    get_arg(&mut elems, line, line_num)
                );
                if turtle.pen.down {
                    if !polyline {
                        polyline = true;
                        poly_points.push(start);
                    }
                    poly_points.push(Point { x: turtle.position.x, y: turtle.position.y } );
                }
            },

            // Left Turn
            "lt" => turtle.bearing = turtle.bearing + get_arg(&mut elems, line, line_num),

            // Right Turn
            "rt" => turtle.bearing = turtle.bearing - get_arg(&mut elems, line, line_num),

            // Circle
            "ci" => {
                if polyline {
                    polyline = false;
                    write_polyline(&mut poly_points, &mut out_port, &mut turtle.pen);
                }
                write!(out_port, "<circle cx='{}' cy='{}' r='{}' fill='{}' />\n",
                       turtle.position.x, 
                       turtle.position.y, 
                       get_arg(&mut elems, line, line_num),
                       turtle.pen.color);
            },

            // Pen Up
            "pu" => {
                if polyline {
                    polyline = false;
                    write_polyline(&mut poly_points, &mut out_port, &mut turtle.pen);
                }
                turtle.pen.down = false;
            },

            // Pen Down
            "pd" => {
                turtle.pen.down = true
            },

            // Pen Size
            "ps" => {
                if polyline {
                    polyline = false;
                    write_polyline(&mut poly_points, &mut out_port, &mut turtle.pen);
                }
                turtle.pen.thickness = get_arg(&mut elems, line, line_num);
            },

            // Pen Color
            "pc" => {
                if polyline {
                    write_polyline(&mut poly_points, &mut out_port, &mut turtle.pen);
                    polyline = false;
                }
                turtle.pen.color = elems.next()
                    .expect(&helpful_error("Expected a string as argument to 'pc'", line, line_num))
                    .to_string();
            },

            // Set Position
            "sp" => {
                if polyline {
                    write_polyline(&mut poly_points, &mut out_port, &mut turtle.pen);
                    polyline = false;
                }
                turtle.position = Point {
                    x: get_arg(&mut elems, line, line_num),
                    y: get_arg(&mut elems, line, line_num)
                };
            }

            _ => {
                write!(out_port, "Invalid input on line {}:\n{}\n", line_num, line);
                out_port.flush();
                std::process::exit(0);
            },
        }
    }
    if polyline {
        polyline = false;
        write_polyline(&mut poly_points, &mut out_port, &mut turtle.pen);
    }
    write!(out_port, "</svg>\n");
    out_port.flush();
}

#[inline]
fn get_arg(mut line_iter: &mut std::iter::Iterator<Item=&str>, line: &str, num: i32) -> f64 {
    let err = "Expected a number as an argument";
    line_iter.next().expect(&helpful_error(err, line, num))
        .parse::<f64>().expect(&helpful_error(err, line, num))
}

fn helpful_error(err: &str, line: &str, num: i32) -> String {
    let mut result = String::from("Error on line ");
    result.push_str(&format!("{}:\n{}\n{}\n", num, line, err));
    result
}

fn write_polyline(points: &mut Vec<Point>, out_port: &mut Write, pen: &Pen) {
    write!(out_port, "<polyline points='");
    {
        let mut iter = points.iter();
        let first = iter.next().expect("Error: polyline has no first value");
        write!(out_port, "{:.2},{:.2}", first.x, first.y);
        for point in iter {
            write!(out_port, " {:.2},{:.2}", point.x, point.y);
        }
    }
    write!(out_port, "' stroke='{}' fill='rgba(0,0,0,0)' stroke-width='{}' />\n", pen.color, pen.thickness);
    points.clear();
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}


fn new_pos(point: &Point, bearing: f64, amount: f64) -> Point {
    let dir = bearing / 180.0f64 * PI;
    Point { 
        x: point.x + amount as f64 * dir.cos(),
        y: point.y - amount as f64 * dir.sin()
    }
}

/*enum Cmd {
    PenUp,
    PenDown,
    PenSize,
    PenColor,
    Forward,
    LeftTurn,
    RightTurn,
}*/

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Pen {
    thickness: f64,
    color: String,
    down: bool,
}

#[derive(Debug)]
struct Turtle {
    position: Point,
    bearing: f64,
    pen: Pen,
}

