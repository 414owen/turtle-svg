use std::io::prelude::*;
use std::io;
use std::env;

fn main() {
    let stdin = io::stdin();
    let mut line_num = 0;
    use std::fs::File;
    let width = 500;
    let height = 500;

    let mut turtle: Turtle = Turtle {
        position: Point { x: width / 2, y: height / 2 },
        bearing: 0,
        pen: Pen {
            thickness: 1,
            color: "#000".to_string(),
            down: true
        }
    };

    println!("<svg width={} height={}>", width, height);
    for line in stdin.lock().lines() {
        line_num = line_num + 1;
        let lineuw = line.unwrap();
        let mut elems = lineuw.split(' ');
        match elems.next().unwrap() {
            "pu" => turtle.pen.down = false,
            "pd" => turtle.pen.down = true,
            "ps" => turtle.pen.thickness = elems.next().unwrap().parse::<i32>().unwrap(),
            bad_input => {
                println!("Invalid input on line {}:\n{}", line_num, bad_input);
                std::process::exit(0);
            },
        }
        println!("{:?}", turtle);
    }
    println!("</svg>");
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
    bearing: i32,
    pen: Pen,
}
