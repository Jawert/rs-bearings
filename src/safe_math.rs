pub fn asin_safe(x: f64) -> f64 {
    let clamped_x = x.clamp(-1.0, 1.0);
    clamped_x.asin()
}

pub fn acos_safe(x: f64) -> f64 {
    let clamped_x = x.clamp(-1.0, 1.0);
    clamped_x.acos()
}

pub fn euclidean_modulo(y: f64, x: f64) -> f64 {
    /*
    Modulo division that follows the sign of the divisor, x.
    Needed for intersect formula, because fmod() follows the sign of the dividend.

    Args:
        - y: the dividend (number to be divided, aka top of the fraction)
        - x: the divisor (number that the dividend is divided by, aka bottom of the fraction)

    Returns:
        - the remainder when dividing y by x
    */
    y - (x * (y / x).floor())
}
