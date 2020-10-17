pub trait Plottable {
    fn initialize(&mut self);
    fn finalize(&mut self);
    fn draw(&mut self, destx_mm: f64, desty_mm: f64);
    fn move_to(&mut self, destx_mm: f64, desty_mm: f64);
    fn draw_relative(&mut self, dx_mm: f64, dy_mm: f64) -> (f64, f64);
    fn move_relative(&mut self, dx_mm: f64, dy_mm: f64) -> (f64, f64);
    fn pen_up(&mut self);
    fn change_color(&mut self, color_name: &str);
}
