extern crate getopts;
use getopts::Options;
use std::fs::File;
use std::io::{self, Read, Write};
use std::f64::consts::PI;

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
        Some(num) => num.parse::<i32>().unwrap(),
        _ => 500
    };
    let height = match matches.opt_str("h") {
        Some(num) => num.parse::<i32>().unwrap(),
        _ => 500
    };
    let mut turtle: Turtle = Turtle {
        position: Point { x: width as f64 / 2.0, y: height as f64 / 2.0 },
        bearing: 0.0f64,
        pen: Pen {
            thickness: 1,
            color: "#000".to_string(),
            down: true
        }
    };

    let mut input = String::new();
    in_port.read_to_string(&mut input);
    write!(out_port, "<svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>\n", width, height);
    let mut polyline = false;
    let mut poly_points = Vec::new();
    for line in input.lines() {
        line_num = line_num + 1;
        let mut elems = line.split(' ');
        let cmd = elems.next().unwrap();
        match cmd {
            "fd" => {
                let start = Point { x: turtle.position.x, y: turtle.position.y };
                turtle.position = new_pos(&turtle.position, turtle.bearing, elems.next().unwrap().parse::<i32>().unwrap());
                if turtle.pen.down {
                    if !polyline {
                        polyline = true;
                        poly_points.push(start);
                    }
                    poly_points.push(Point { x: turtle.position.x, y: turtle.position.y } );
                }
            },
            "lt" => turtle.bearing = turtle.bearing + elems.next().unwrap().parse::<f64>().unwrap(),
            "rt" => turtle.bearing = turtle.bearing - elems.next().unwrap().parse::<f64>().unwrap(),
            "pu" => {
                turtle.pen.down = false;
                if polyline {
                    polyline = false;
                    finalise_polyline(&mut poly_points, &mut out_port, &mut turtle.pen);
                }
            },
            "pd" => {
                turtle.pen.down = true
            },
            "ps" => {
                turtle.pen.thickness = elems.next().unwrap().parse::<i32>().unwrap();
                if polyline {
                    polyline = false;
                    finalise_polyline(&mut poly_points, &mut out_port, &mut turtle.pen);
                }
            },
            "pc" => {
                turtle.pen.color = elems.next().unwrap().to_string();
                if polyline {
                    polyline = false;
                    finalise_polyline(&mut poly_points, &mut out_port, &mut turtle.pen);
                }
            },
            _ => {
                if polyline {
                    polyline = false;
                    finalise_polyline(&mut poly_points, &mut out_port, &mut turtle.pen);
                }
                write!(out_port, "Invalid input on line {}:\n{}\n", line_num, line);
                out_port.flush();
                std::process::exit(0);
            },
        }
    }
    if polyline {
        polyline = false;
        finalise_polyline(&mut poly_points, &mut out_port, &mut turtle.pen);
    }
    write!(out_port, "</svg>\n");
    out_port.flush();
}

fn finalise_polyline(points: &mut Vec<Point>, out_port: &mut Write, pen: &mut Pen) {
    write!(out_port, "<polyline points='");
    {
        let mut iter = points.iter();
        let first = iter.next().unwrap();
        write!(out_port, "{:.2},{:.2}", first.x, first.y);
        for point in iter {
            write!(out_port, " {:.2},{:.2}", point.x, point.y);
        }
    }
    write!(out_port, "' stroke='{}' fill='rgba(0,0,0,0)' stroke-width='{}' />", pen.color, pen.thickness);
    points.clear();
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}


fn new_pos(point: &Point, bearing: f64, amount: i32) -> Point {
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
    thickness: i32,
    color: String,
    down: bool,
}

#[derive(Debug)]
struct Turtle {
    position: Point,
    bearing: f64,
    pen: Pen,
}

