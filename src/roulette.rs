//! The roulette module contains code to plot various roulettes, the generalized
//! term for curves like cycloids, epicycloids, hypocycloids, and trochoids.
//! https://en.wikipedia.org/wiki/Roulette_(curve)
//! https://en.wikipedia.org/wiki/Trochoid

use std::f64::consts::PI;
use crate::plottable::Plottable;

const STEPS: i32 = 40; // Steps in one rotation of the rolling circle.

/// Circle that rolls along other curves and generates the roulette curves.
/// Fields are in mm.
struct Roller {
    circle_radius: f64,
    pen_radius: f64,
}

struct Translator {
    centerx_mm: f64,
    centery_mm: f64,
    rot_rad: f64,
}

impl Translator {
    fn translate(&self, inputx: f64, inputy: f64) -> (f64, f64) {
        let mut x = inputx * self.rot_rad.cos() - inputy * self.rot_rad.sin();
        let mut y = inputx * self.rot_rad.sin() + inputy * self.rot_rad.cos();
        x += self.centerx_mm;
        y += self.centery_mm;
        (x, y)
    }
}

/// Generate full hypotrochoid curves (like a Spirograph where you move the gear inside a larger circle).
/// The difference from a Spirograph is that the pen radius can be the same or or even larger than
/// the radius of the inner circle, and that creates different curves.
///
/// plotter: device to plot to.
/// rolling_radius: radius of rolling circle in mm.
/// pen_radius: radius of pen attached to roller in mm.
/// inner, outer: integers that allow you to set the relative sizes of the two circles.
///
/// If inner, outer are coprime (no common factors), there will be "outer" radial maxima/cusps.
///
pub fn full_hypotrochoid(plotter: &mut impl Plottable, rolling_radius_mm: f64, pen_radius_mm: f64,
                         inner: i32, outer: i32, centerx_mm: f64, centery_mm: f64, rot_rad: f64 ) {
    // Error checking.
    if inner > outer {
        panic!("Parameter `inner` must be greater than `outer`.")
    }
    // Setup.
    let ratio: f64 = inner as f64 / outer as f64;
    let outer_mm = rolling_radius_mm / ratio;
    let plot_radius = outer_mm - rolling_radius_mm + pen_radius_mm; // Max extent.
    println!("Plot radius is {} mm.", plot_radius);
    let pen2outer = pen_radius_mm / outer_mm;
    // Create the translator struct.
    let trans = Translator {centerx_mm, centery_mm, rot_rad};

    // Plotting.
    let (x, y ) = trans.translate(plot_radius, 0.0);
    plotter.move_to(x, y);
    for i in 0 .. (inner * STEPS + 1) { // Add one to get a complete curve.
        let t = 2.0 * PI * i as f64 / STEPS as f64;
        let x = outer_mm * ((1.0 - ratio) * t.cos() + pen2outer * ((1.0 - ratio) / ratio * t).cos() );
        let y = outer_mm * ((1.0 - ratio) * t.sin() - pen2outer * ((1.0 - ratio) / ratio * t).sin() );
        let (x, y) = trans.translate(x, y);
        plotter.draw(x, y);
    }

}