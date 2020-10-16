// This file contains the module uscutter due to its filename.

use std::io::{self, Write};
use std::time::Duration;

use serialport; // API documentation at https://docs.rs/serialport/3.3.0/serialport/, examples at https://gitlab.com/susurrus/serialport-rs
use serialport::DataBits::Eight;
use serialport::FlowControl::Hardware;
use serialport::StopBits::One; // Guide at https://turtle.rs, API documentation at https://docs.rs/turtle, and more examples at https://github.com/sunjay/turtle.

// Constants related to a USCutter LPII cutter/plotter.
const SCALEX: f64 = 0.0251;   // mm per plotter unit. (When set at 0.025, a "150mm" line is 150.6mm long.)
const SCALEY: f64 = 0.024917; // mm per plotter unit. (When set at 0.025, a "150mm" line is 149.5mm long.)
const OFFSETX: i32 = 25;      // pen offset in plotter units.
const OFFSETY: i32 = 25;      // plotter units.

pub struct USCutter {
    min_x_mm: f64, // Minimum value of the pen, in mm.
    min_y_mm: f64,
    pos_x_mm: f64, // Present position of the pen in mm.
    pos_y_mm: f64,
    // Dimensions that are in plotter units 0-n, where n is an integer.
    offset_x: i32, // Pen offset.
    offset_y: i32,
    max_x: i32,    // Maximum allowed position of the pen.
    max_y: i32,
//    pen_down: bool,
//    heading_radians: f64, // Heading in radians, 0 = East, positive is CCW.
                         // (to be compatible with turtle graphics when put in standard radians mode). TODO: check this.
    port: Box<dyn serialport::SerialPort>,
}

impl USCutter {
    /// Create a new USCutter struct.
    ///
    /// `port_name`: The text name for the COM port, e.g. COM12.
    /// `llx_mm, lly_mm`: The coordinates for the lower left corner of the plot, in mm.
    /// `urx_mm, ury_mm`: The coordinates for the upper right corner of the plot.
    pub fn new(port_name: &str, llx_mm: f64, lly_mm: f64, urx_mm: f64, ury_mm: f64) -> USCutter {
        // Check that the upper right is greater than the lower left.
        let size_x_mm = urx_mm - llx_mm;
        let size_y_mm = ury_mm - lly_mm;

        if (size_x_mm <= 0.0) || (size_y_mm <= 0.0) {
            panic!("Error: upper right is not greater than lower left.");  // TODO: better error handling.
        }

        // Get the serial port.
        let settings = serialport::SerialPortSettings{
            baud_rate: 9600,
            data_bits: Eight,
            flow_control: Hardware,
            parity: serialport::Parity::None,
            stop_bits: One,
            timeout: Duration::from_millis(10)
        };
        let port_obj = serialport::open_with_settings(port_name, &settings).expect("can't open serial port");

        // Create the struct and return it.
        USCutter {
            min_x_mm: llx_mm,
            min_y_mm: lly_mm,
            pos_x_mm: llx_mm,
            pos_y_mm: lly_mm,
            offset_x: OFFSETX,
            offset_y: OFFSETY,
            max_x: (size_x_mm / SCALEX) as i32 + OFFSETX, // In plotter units.
            max_y: (size_y_mm / SCALEY) as i32 + OFFSETY,
//            pen_down: false,
//            heading_radians: 0.0,
            port: port_obj,
        }
    }

    pub fn initialize(&mut self) {
        // Prepare to plot
        match self.port.write(b";:H A L0 ECN U ") {
            Ok(_) => {
                println!("Initializing");
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => eprintln!("Timeout when initializing plotter."),
            Err(e) => eprintln!("{:?}", e)
        }
        // move the offset
        match self.port.write(b"PU25,25;") {
            Ok(_) => {
                print!(".");
                std::io::stdout().flush().unwrap();
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => eprintln!("Timeout when moving the initial offset."),
            Err(e) => eprintln!("{:?}", e)
        }
    }

    pub fn finalize(&mut self) {
        // Finish plot
        match self.port.write(b"PU0,0;!PG;") {
            Ok(_) => {
                println!("\nfinalizing.");
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => eprintln!("Timeout when finalizing plotter."),
            Err(e) => eprintln!("{:?}", e)
        }
    }

    /// Draw a straight line from present position to absolute position (destx_mm, desty_mm), in units of mm.
    pub fn draw(&mut self, destx_mm: f64, desty_mm: f64) {
        self.pos_x_mm = destx_mm;
        self.pos_y_mm = desty_mm;
        let x = self.clip_x(self.mm2plt_x(destx_mm) + self.offset_x); // Convert and clip
        let y = self.clip_y(self.mm2plt_y(desty_mm) + self.offset_y); // Convert and clip

        let s = format!("PD{},{};", x, y);
        match self.port.write(s.as_bytes()) {
            Ok(_) => {
                print!(".");
                std::io::stdout().flush().unwrap();
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => eprintln!("Timeout during operations."),
            Err(e) => eprintln!("{:?}", e)
        }
    }

    /// Move pen without drawing to absolute position (destx_mm, desty_mm), in units of mm.
    pub fn move_to(&mut self, destx_mm: f64, desty_mm: f64) {
        self.pos_x_mm = destx_mm;
        self.pos_y_mm = desty_mm;
        let x = self.clip_x(self.mm2plt_x(destx_mm) + self.offset_x); // Convert and clip
        let y = self.clip_y(self.mm2plt_y(desty_mm) + self.offset_y); // Convert and clip

        let s = format!("PU{},{};", x, y);
        match self.port.write(s.as_bytes()) {
            Ok(_) => {
                print!(".");
                std::io::stdout().flush().unwrap();
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => eprintln!("Timeout during operations."),
            Err(e) => eprintln!("{:?}", e)
        }
    }

    /// Draw from present position (dx, dy) mm.
    /// Returns the new position of the pen.
    pub fn draw_relative(&mut self, dx_mm: f64, dy_mm: f64) -> (f64, f64) {
        self.draw(self.pos_x_mm + dx_mm, self.pos_y_mm + dy_mm);
        (self.pos_x_mm, self.pos_y_mm)
    }

    /// Move the pen without drawing from present position (dx, dy) mm.
    /// Returns the new position of the pen.
    pub fn move_relative(&mut self, dx_mm: f64, dy_mm: f64) -> (f64, f64) {
        self.move_to(self.pos_x_mm + dx_mm, self.pos_y_mm + dy_mm);
        (self.pos_x_mm, self.pos_y_mm)
    }

    // Raise the pen.
    pub fn pen_up(&mut self) {
        match self.port.write(b"PU;") {
            Ok(_) => {
                print!(".");
                std::io::stdout().flush().unwrap();
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => eprintln!("Timeout during operations."),
            Err(e) => eprintln!("{:?}", e)
        }

    }

    // Helper methods.

    /// Convert x dimension in mm to plotter units.
    fn mm2plt_x(&self, xmm:f64) -> i32 {
        ((xmm - self.min_x_mm) / SCALEX) as i32
    }
    /// Convert y dimension in mm to plotter units.
    fn mm2plt_y(&self, ymm:f64) -> i32 {
        ((ymm - self.min_y_mm) / SCALEY) as i32
    }
    /// Convert x dimension in plotter units to mm.
    fn plt2mm_x(&self, xplt: i32) -> f64 {
        xplt as f64 * SCALEX + self.min_x_mm
    }
    /// Convert y dimension in plotter units to mm.
    fn plt2mm_y(&self, yplt: i32) -> f64 {
        yplt as f64 * SCALEY + self.min_y_mm
    }
    /// Clip x dimension in plotter units to [0, max].
    fn clip_x(&self, x: i32) -> i32 {
        if x < 0 {
            0
        } else if x > self.max_x {
            self.max_x
        } else {
            x
        }
    }
    /// Clip y dimension in plotter units to [0, max].
    fn clip_y(&self, y: i32) -> i32 {
        if y < 0 {
            0
        } else if y > self.max_y {
            self.max_y
        } else {
            y
        }
    }
}
