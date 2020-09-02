



fn add_numbers_f64(x: f64, y: f64) -> f64 {
    x + y
}

fn add_numbers_i32(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    // floats
    let a = 5.0;
    let mut b = 10.0;

    // ints
    let mut c = 6;
    let d = 11;

    b = add_numbers_f64(a, b);
    c = add_numbers_i32(c, d);
}

