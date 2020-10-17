//! Crate to create interesting plots with a USCutter LPII cutter/plotter, including
//! a method to preview the operations using turtle graphics.
//!
//! To use: edit the beginning of main() to set the window size and whether you want
//! to display on the screen or send to a plotter.
//! Then put the code to create the plot into generate_plot().
//!

mod plottable;
mod turtle_plot; // Load the modules from files of the same name.
mod uscutter;

use std::f64::consts::PI;
use std::error::Error;
use plottable::Plottable;
use uscutter::USCutter;
use crate::turtle_plot::TurtlePlotter;

struct Spirograph {
    inner: u32,
    outer: u32,
    radius: f64,
    pen: f64,
}

impl Spirograph {
    /// Returns the total radius of the pattern, so we can set paper/window size.
    fn get_plot_radius(&self) -> f64 {
        let ratio: f64 = self.inner as f64 / self.outer as f64;
        self.radius * (1.0 - ratio + self.pen * ratio)
    }

    /// Generates the plot.
    fn plot(&self, plotter: &mut impl Plottable) {
        let ratio: f64 = self.inner as f64 / self.outer as f64;
        let mut x = self.get_plot_radius();
        let mut y = 0.0;
        plotter.move_to(x, y);
        for i in 0 .. (self.inner * 40) {
            let t = 2.0 * PI * i as f64 / 40.0;
            x = self.radius * ((1.0 - ratio) * t.cos() + self.pen * ratio * ((1.0 - ratio) / ratio * t).cos() );
            y = self.radius * ((1.0 - ratio) * t.sin() - self.pen * ratio * ((1.0 - ratio) / ratio * t).sin() );
            plotter.draw(x, y);
        }
    }
}

fn main()  -> Result<(), Box<dyn Error>> {
    // Choose whether to display on screen or send to plotter.
    // TODO: add code to read the command line to get this value.
    let send_to_plotter = false;

    // Plot bounds, lower left corner
    // Change these when setting up a plot.
    let plot_minx_mm = -50.0;
    let plot_miny_mm = -50.0;
    // Upper right corner
    let plot_maxx_mm = 50.0;
    let plot_maxy_mm = 50.0;

    // Below here in this function should not have to change as plots are changed.

    // Choose which output device we are using.
    // Note that we might be able to use trait objects to create a generic plotter variable/struct that can hold either type of device
    // and thus move the "initialize, generate, finalize" parts of the code outside the "if" expression.  But that is more digging
    // than I want to do at the moment; the approach below works.  See these parts of the Rust Book:
    // https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
    // https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait
    if send_to_plotter {
        // Cutter/plotter.
//    let port_name = "COM4";  // FTDI cable through the docking station.
        let port_name = "COM12"; // Plotter through the docking station.
        let mut plotter = USCutter::new(port_name, plot_minx_mm, plot_miny_mm, plot_maxx_mm, plot_maxy_mm);
        plotter.initialize();
        generate_plot(&mut plotter);
        plotter.finalize();
    } else {
        // Turtle graphics plotting
        let mut plotter = TurtlePlotter::new(plot_minx_mm, plot_miny_mm, plot_maxx_mm, plot_maxy_mm);
        // Code below is duplicated because plotter lives only within the else block, and I haven't figured out how to make a generic for it.
        plotter.initialize();
        generate_plot(&mut plotter);
        plotter.finalize();
    }

    Ok(())
}

/// Function to hold plot-generating commands (outside of initialize() and finalize() ).
/// Call the plotter's move_to() and draw() methods, or write other functions that do.
fn generate_plot(plotter: &mut impl Plottable) {
    // Put plot-generating commands here.
    let spiro = Spirograph {
        inner: 17,
        outer: 20,
        radius: 50.0,
        pen: 0.7,
    };
    spiro.plot(plotter);
}
