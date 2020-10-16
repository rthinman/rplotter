mod turtle_plot; // Load the modules from files of the same name.
mod uscutter;

use std::f64::consts::PI;
use std::error::Error;
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
    fn plot(&self, plotter: &mut TurtlePlotter) {
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
    // Cutter/plotter.
//    let port_name = "COM4";  // FTDI cable through the docking station.
//    let port_name = "COM12"; // Plotter through the docking station.
//    let mut plotter = USCutter::new(port_name, 0.0, 0.0, 25.0, 25.0);

    let spiro = Spirograph {
        inner: 17,
        outer: 20,
        radius: 50.0,
        pen: 0.7,
    };

    let plot_radius = spiro.get_plot_radius();

    // Turtle graphics plotting
    let mut plotter = TurtlePlotter::new(-plot_radius, -plot_radius, plot_radius, plot_radius);

    plotter.initialize();

    spiro.plot(&mut plotter);
//    plotter.draw(10.0, 0.0);
//    plotter.move_relative(5.0, 0.0);
//    plotter.draw(10.0, 10.0);
//    plotter.move_to(5.0, 15.0);
//    plotter.draw(0.0, 10.0);
//    plotter.pen_up();
//    plotter.draw(0.0, 0.0);
//    plotter.draw_relative(2.0, 2.0);
//    // Test the bounds.
//    plotter.draw(-2.0, -2.0);
//    plotter.draw(-2.0, 30.0);
//    plotter.draw(30.0, 30.0);
//    plotter.draw(30.0, -2.0);
//    plotter.draw(-2.0, -2.0);
    plotter.finalize();

//    Turtle graphics stuff for future use.
//    let mut drawing = Drawing::new();  // Get the drawing first, so we can set its size.
//    drawing.set_size([400, 400]);
//    let mut turtle = drawing.add_turtle();  // Get the turtle struct from the drawing.
//    println!("drawing size: {:?}", drawing.size());
//    turtle.use_radians();
//
//    for _ in 0..360 {
//        turtle.forward(3.0);
//        turtle.right(2.0 * PI / 360.0);
//    }
//    turtle.go_to(Point {x: -190.0, y: -190.0});
//    turtle.go_to(Point {x: -190.0, y: 190.0});
//    turtle.go_to(Point {x: 190.0, y: 190.0});
//    turtle.go_to(Point {x: 190.0, y: -190.0});
    Ok(())
}
