//! turtle_plot module contains the TurtlePlotter struct, used as a wrapper for turtle graphics
//! to make it compatible with the cutter/plotter driver module uscutter.
//!

// Some helpful web sites:
// https://docs.rs/turtle/1.0.0-rc.3/turtle/index.html
// https://github.com/sunjay/turtle
// https://turtle.rs/

use turtle::*;
use crate::plottable::Plottable;

const SCREENX_PX: i32 = 1200; // Size that fits OK on laptop main screen with menu on the left.
const SCREENY_PX: i32 = 600;  // Could do 1280 x 640 with menu and icons on the bottom.

pub struct TurtlePlotter {
    min_x_mm: f64, // Minimum value of the pen, in mm.
    min_y_mm: f64,
    max_x_mm: f64,
    max_y_mm: f64,
    pos_x_mm: f64, // Present position of the pen in mm.
    pos_y_mm: f64,
    scale: f64,    // in mm/px.  Applies to both x and y dimensions.
    drawing: Drawing,
    turtle: Turtle,
}

impl TurtlePlotter {
    pub fn new(llx_mm: f64, lly_mm: f64, urx_mm: f64, ury_mm: f64) -> TurtlePlotter {
        // Check that the upper right is greater than the lower left.
        let size_x_mm = urx_mm - llx_mm;
        let size_y_mm = ury_mm - lly_mm;

        if (size_x_mm <= 0.0) || (size_y_mm <= 0.0) {
            panic!("Error: upper right is not greater than lower left.");  // TODO: better error handling.
        }

        // Figure out scaling for the window.
        let scalex = size_x_mm / SCREENX_PX as f64;
        let scaley = size_y_mm / SCREENY_PX as f64;
        let scale = if scalex > scaley {
            // In this case, the plot's x dimension in mm will span the width of the window.
            scalex // Everything will be scaled by this factor.
        } else {
            // In this case, the plot's y dimension in mm will span the height of the window.
            scaley // Everything will be scaled by this factor.
        };

        // Calculate offset to give to the turtle package.
        let offsetx = (urx_mm + llx_mm) / 2.0 / scale;  // Center is in the average of the limits given, scaled.
        let offsety = (ury_mm + lly_mm) / 2.0 / scale;

        let mut drawing = Drawing::new();  // Get the drawing first, so we can set its size and center.
        drawing.set_size([SCREENX_PX as u32, SCREENY_PX as u32]);
        drawing.set_center([offsetx, offsety]);
        let mut turtle = drawing.add_turtle();  // Get the turtle struct from the drawing.
        println!("drawing size: {:?}", drawing.size());
        turtle.use_radians();
        turtle.set_speed("faster");

        TurtlePlotter {
            min_x_mm: llx_mm,
            min_y_mm: lly_mm,
            max_x_mm: urx_mm,
            max_y_mm: ury_mm,
            pos_x_mm: llx_mm,
            pos_y_mm: lly_mm,
            scale: scale,
            drawing: drawing,
            turtle: turtle
        }
    }
}

impl Plottable for TurtlePlotter {

    /// Provided for compatibility with the cutter/plotter.
    fn initialize(&mut self) {
        println!("Initializing...");
    }

    /// Provided for compatibility with the cutter/plotter.
    fn finalize(&mut self) {
        self.move_to(0.0, 0.0);
        println!("Finalizing.");
    }

    /// Draw a straight line from present position to absolute position (destx_mm, desty_mm), in units of mm.
    fn draw(&mut self, destx_mm: f64, desty_mm: f64) {
        self.turtle.pen_down();
        self.turtle.go_to(Point {x: destx_mm/self.scale, y: desty_mm / self.scale});
        self.pos_x_mm = destx_mm; // Update position.
        self.pos_y_mm = desty_mm;
    }

    /// Move pen without drawing to absolute position (destx_mm, desty_mm), in units of mm.
    fn move_to(&mut self, destx_mm: f64, desty_mm: f64) {
        self.turtle.pen_up();
        self.turtle.go_to(Point {x: destx_mm/self.scale, y: desty_mm / self.scale});
        self.pos_x_mm = destx_mm; // Update position.
        self.pos_y_mm = desty_mm;
    }

    /// Draw from present position (dx, dy) mm.
    /// Returns the new position of the pen.
    fn draw_relative(&mut self, dx_mm: f64, dy_mm: f64) -> (f64, f64) {
        self.draw(self.pos_x_mm + dx_mm, self.pos_y_mm + dy_mm);
        (self.pos_x_mm, self.pos_y_mm)
    }

    /// Move the pen without drawing from present position (dx, dy) mm.
    /// Returns the new position of the pen.
    fn move_relative(&mut self, dx_mm: f64, dy_mm: f64) -> (f64, f64) {
        self.move_to(self.pos_x_mm + dx_mm, self.pos_y_mm + dy_mm);
        (self.pos_x_mm, self.pos_y_mm)
    }

    /// Raise the pen.
    fn pen_up(&mut self) {
        self.turtle.pen_up();
    }

    /// Sets the color of the pen.  Wraps the turtle command primarily so we can do something
    /// manual with the USCutter struct.
    /// See this documentation for pre-defined color string names.
    /// https://docs.rs/turtle/1.0.0-rc.3/turtle/color/index.html
    /// Those that match HP and other manufacturer pen colors are:
    /// black, blue, brown, cyan (HP aqua), green, magenta, orange, purple (HP violet), red, yellow
    fn change_color(&mut self, color_name: &str) {
        self.turtle.set_pen_color(color_name);
    }
}