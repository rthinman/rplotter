mod uscutter;  // Load the module uscutter from file uscutter.rs.

//use std::f64::consts::PI;
use std::error::Error;
use uscutter::USCutter;


fn main()  -> Result<(), Box<dyn Error>> {
//    let port_name = "COM4";  // FTDI cable through the docking station.
    let port_name = "COM12"; // Plotter through the docking station.

    let mut plotter = USCutter::new(port_name, 0.0, 0.0, 25.0, 25.0);
    plotter.initialize();
    plotter.draw(10.0, 0.0);
    plotter.move_relative(5.0, 0.0);
    plotter.draw(10.0, 10.0);
    plotter.move_to(5.0, 15.0);
    plotter.draw(0.0, 10.0);
    plotter.pen_up();
    plotter.draw(0.0, 0.0);
    plotter.draw_relative(2.0, 2.0);
    // Test the bounds.
    plotter.draw(-2.0, -2.0);
    plotter.draw(-2.0, 30.0);
    plotter.draw(30.0, 30.0);
    plotter.draw(30.0, -2.0);
    plotter.draw(-2.0, -2.0);
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
