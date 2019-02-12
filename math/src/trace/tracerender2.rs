use std::ops;

pub trait TraceRender2 {
    fn begin(&mut self);
    fn end(&mut self);

    fn set_viewport(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64);

    fn push_group(&mut self, name: Option<String>);
    fn pop_group(&mut self);

    fn add_point(&mut self, p: &(f64, f64), color: String);
    fn add_line(&mut self, a: &(f64, f64), b: &(f64, f64), color: String);
    fn add_text(&mut self, p: &(f64, f64), msg: String, color: String, size: f32);
}

