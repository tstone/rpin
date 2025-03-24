pub(crate) fn round_to_nearest(value: f32) -> f32 {
    let floor = value.floor();
    let rem = value - floor;
    if rem >= 0.5 {
        value.ceil()
    } else {
        floor
    }
}
