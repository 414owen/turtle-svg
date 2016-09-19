pub fn forward(amount: f64) {
    println!("fd {}", amount);
}

pub fn pen_color(col: &str) {
    println!("pc {}", col);
}

pub fn pen_size(size: f64) {
    println!("ps {}", size);
}

pub fn left_turn(deg: f64) {
    println!("lt {}", deg);
}

pub fn right_turn(deg: f64) {
    println!("rt {}", deg);
}

pub fn circle(rad: f64) {
    println!("ci {}", rad);
}

pub fn set_position(x: f64, y: f64) {
    println!("sp {} {}", x, y);
}

pub fn pen_up() {
    println!("pu");
}

pub fn pen_down() {
    println!("pd");
}
