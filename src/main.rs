use std::f64::consts::PI;
use std::error::Error;
use std::io::{self, Write};
use std::time::Duration;
use serialport; // API documentation at https://docs.rs/serialport/3.3.0/serialport/, examples at https://gitlab.com/susurrus/serialport-rs
use turtle::*;
use serialport::DataBits::Eight;
use serialport::FlowControl::Hardware;
use serialport::StopBits::One; // Guide at https://turtle.rs, API documentation at https://docs.rs/turtle, and more examples at https://github.com/sunjay/turtle.

const SCALEX: f64 = 0.0251;   // mm per plotter unit. (When set at 0.025, a "150mm" line is 150.6mm long.)
const SCALEY: f64 = 0.024917; // mm per plotter unit. (When set at 0.025, a "150mm" line is 149.5mm long.)

// Function signature from the "hardware_check" example of serialport: https://gitlab.com/susurrus/serialport-rs/-/tree/master/examples
// Also, method of calling it in main.
fn plot(port: &mut dyn serialport::SerialPort, destx: i32, desty: i32){
    let s = format!("PD{},{};", destx, desty);
    match port.write(s.as_bytes()) {
        Ok(_) => {
            print!(".");
            std::io::stdout().flush().unwrap();
        }
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => eprintln!("Timeout during operations."),
        Err(e) => eprintln!("{:?}", e)
    }

}

fn main()  -> Result<(), Box<dyn Error>> {
//    let port_name = "COM4";  // FTDI cable through the docking station.
    let port_name = "COM12"; // Plotter through the docking station.

    let settings = serialport::SerialPortSettings{
        baud_rate: 9600,
        data_bits: Eight,
        flow_control: Hardware,
        parity: serialport::Parity::None,
        stop_bits: One,
        timeout: Duration::from_millis(10)
    };

    let mut port = serialport::open_with_settings(port_name, &settings)?;
//    let mut port = serialport::open(port_name)?;

    // Prepare to plot
    match port.write(b";:H A L0 ECN U ") {
        Ok(_) => {
            println!("Initializing");
        }
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => eprintln!("Timeout when initializing plotter."),
        Err(e) => eprintln!("{:?}", e)
    }

    // move the offset
    match port.write(b"PU25,25;") {
        Ok(_) => {
            print!(".");
            std::io::stdout().flush().unwrap();
        }
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => eprintln!("Timeout when moving the initial offset."),
        Err(e) => eprintln!("{:?}", e)
    }

    plot( &mut *port, 425, 25);  // Dereference to remove the Box?  Anyway, this works.
    plot( &mut *port, 425, 425);
    plot( &mut *port, 25, 425);
    plot( &mut *port, 25, 25);

//    // Plot a square 1
//    match port.write(b"PD425,25;") {
//        Ok(_) => {
//            print!(".");
//            std::io::stdout().flush().unwrap();
//        }
//        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => eprintln!("Timeout during operations."),
//        Err(e) => eprintln!("{:?}", e)
//    }
//
//    // Plot a square 2
//    match port.write(b"PD425,425;") {
//        Ok(_) => {
//            print!(".");
//            std::io::stdout().flush().unwrap();
//        }
//        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => eprintln!("Timeout during operations."),
//        Err(e) => eprintln!("{:?}", e)
//    }
//
//    // Plot a square 3
//    match port.write(b"PD25,425;") {
//        Ok(_) => {
//            print!(".");
//            std::io::stdout().flush().unwrap();
//        }
//        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => eprintln!("Timeout during operations."),
//        Err(e) => eprintln!("{:?}", e)
//    }
//
//    // Plot a square 4
//    match port.write(b"PD25,25;") {
//        Ok(_) => {
//            print!(".");
//            std::io::stdout().flush().unwrap();
//        }
//        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => eprintln!("Timeout during operations."),
//        Err(e) => eprintln!("{:?}", e)
//    }

    // Finish plot
    match port.write(b"PU0,0;!PG;") {
        Ok(_) => {
            println!("\nfinalizing.");
        }
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => eprintln!("Timeout when finalizing plotter."),
        Err(e) => eprintln!("{:?}", e)
    }

    // One way to format to a byte array, but it gives you something that is too long.
    // (per https://stackoverflow.com/questions/39488327/how-to-format-output-to-a-byte-array-with-no-std-and-no-allocator)
//    let x = 123;
//    let mut buf = [0 as u8; 20];
//    write!(&mut buf[..], "{}", x).expect("couldn't write");
//    assert_eq!(&buf[0..3], b"123");
    //Could also dd with s a string, assuming it is all ASCII:
    // s.as_bytes(); or s.into_bytes()

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
