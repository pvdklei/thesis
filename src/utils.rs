pub fn speed(pos: bool, neg: bool, speed_factor: f32) -> f32 {
    if pos ^ neg {
        if pos {
            speed_factor
        } else {
            -speed_factor
        }
    } else {
        0.
    }
}

pub fn add_padding(xx: &mut [f32], d: f32) {
    for x in xx.iter_mut() {
        let xabs = x.abs();
        if xabs < d {
            if *x < 0.0 {
                *x = -xabs.max(d);
            } else {
                *x = xabs.max(d);
            }
        }
    }
}
