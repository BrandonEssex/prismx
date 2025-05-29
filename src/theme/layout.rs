pub fn spacing_scale(zoom: f32) -> (i16, i16) {
    let x = (6.0 * zoom).round().max(4.0) as i16;
    let y = (3.0 * zoom).round().max(2.0) as i16;
    (x, y)
}

