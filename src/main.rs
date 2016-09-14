use std::io::prelude::*;
use std::io;
use std::env;
use std::f64::consts::PI;

fn main() {
    let stdin = io::stdin();
    let mut line_num = 0;
    let width = 500;
    let height = 500;
    let mut turtle: Turtle = Turtle {
        position: Point { x: width / 2, y: height / 2 },
        bearing: 0.0f64,
        pen: Pen {
            thickness: 1,
            color: "#000".to_string(),
            down: true
        }
    };

    let mut outPort = io::stdout();
    let mut lastCmd = "";
    println!("<svg width={} height={} xmlns='http://www.w3.org/2000/svg'>", width, height);
    for line in stdin.lock().lines() {
        line_num = line_num + 1;
        let lineuw = line.unwrap();
        let mut elems = lineuw.split(' ');
        let cmd = elems.next().unwrap();
        match cmd {
            "fd" => {
                let start = Point { x: turtle.position.x, y: turtle.position.y };
                turtle.position = newPos(&turtle.position, turtle.bearing, elems.next().unwrap().parse::<i32>().unwrap());
                let end = &turtle.position;
                if turtle.pen.down {
                    write!(
                        outPort,
                        "<line x1='{}' y1='{}' x2='{}' y2='{} stroke='{}' stroke-width='{}'>\n",
                        start.x, start.y, end.x, end.y, turtle.pen.color, turtle.pen.thickness
                    );
                }
            },
            "pu" => turtle.pen.down = false,
            "pd" => turtle.pen.down = true,
            "ps" => turtle.pen.thickness = elems.next().unwrap().parse::<i32>().unwrap(),
            "pc" => turtle.pen.color = elems.next().unwrap().to_string(),
            _ => {
                write!(outPort, "Invalid input on line {}:\n{}\n", line_num, lineuw);
                outPort.flush();
                std::process::exit(0);
            },
        }
        write!(outPort, "{:?}\n", turtle);
    }
    write!(outPort, "</svg>\n");
    outPort.flush();
}

fn newPos(point: &Point, bearing: f64, amount: i32) -> Point {
    let dir = bearing / 180.0f64 * PI;
    Point { x: point.x + ((amount as f64 * dir.cos()) as i32), y: point.y + ((amount as f64 * dir.sin()) as i32) }
}

enum Cmd {
    penUp,
    penDown,
    penSize,
    penColor,
    forward,
    leftTurn,
    rightTurn,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
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

