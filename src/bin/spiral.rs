mod gen;
fn main() {
    let end = 200;
    let mut length = 10;
    let turn = 90.8;
    gen::forward(length/2);
    for i in 0..end {
        gen::left_turn(turn);
        gen::forward(length);
        length = length + 2
    }
}
