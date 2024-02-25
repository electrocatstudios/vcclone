use js_sys::Math::atan2;
use std::cmp::Ordering;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point {
            x: x,
            y: y
        }
    }
}

const TWO_PI: f64 = std::f64::consts::PI * 2.0;

// Returns as rotation from 0.0 - will always be between 0.0 and 2 * pi
pub fn get_angle_between_points(start: &Point, end: &Point ) -> f64 {
    let diff_x = end.x - start.x;
    let diff_y = start.y - end.y;
    normalize_angle((std::f64::consts::PI/2.0) - atan2(diff_y, diff_x))
}

// Return angle between 0 and 2 * PI
pub fn normalize_angle(angle: f64) -> f64 {
    // Make sure angle is between 0 and 2*pi
    if angle < 0.0 {
        angle + TWO_PI
    } else if angle > TWO_PI {
        angle - TWO_PI
    } else {
        angle
    }
}