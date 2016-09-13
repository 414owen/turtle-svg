use std::io::prelude::*;
use std::io;

fn main() {
    println!("Initialising turtle.");
    let stdin = io::stdin();
    let mut line_num = 0;
    for line in stdin.lock().lines() {
        line_num = line_num + 1;
        match &line.unwrap()[..] {
            "hi" => println!("MATCH!"),
            bad_input => println!("Invalid input on line {}:\n{}", line_num, bad_input),
        }
    }
}

struct Point(i32,i32);

struct Pen {
    thickness: i32,
    color: String,
}

struct Turtle {
    position: Point,
    breaing: i32,
    pen: Pen,
}
