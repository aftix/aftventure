use noise::{NoiseFn, OpenSimplex};

use std::i32;

pub fn height((x, y): (i32, i32), gen: &OpenSimplex) -> i32 {
    let scale = 0.001;
    let scaled_x = scale * (x as f64);
    let scaled_y = scale * (y as f64);

    let mut val = gen.get([scaled_x, scaled_y]);
    val += gen.get([scaled_x / 2.0, scaled_y / 2.0]) / 2.0;
    val += gen.get([scaled_x / 4.0, scaled_y / 4.0]) / 4.0;
    val += gen.get([scaled_x / 8.0, scaled_y / 8.0]) / 8.0;
    let height = ((val + 1.0) / 2.0) * (200.0 - 50.0) + 50.0;
    height as i32
}
