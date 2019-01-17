use shine_math::trace::Trace2Render;
use shine_testutils::webserver::{D2Trace, Service};
use std::mem;

/// Alow to debug 2d geometry using the D2Trace.
pub struct D2TraceRender {
    service: Service,
    render: Option<D2Trace>,
}

impl D2TraceRender {
    pub fn new(service: Service) -> D2TraceRender {
        D2TraceRender { service, render: None }
    }
}

impl Trace2Render for D2TraceRender {
    fn begin(&mut self) {
        self.end();
        self.render = Some(D2Trace::new());
    }

    fn end(&mut self) {
        let tr = mem::replace(&mut self.render, None);
        if let Some(mut tr) = tr {
            tr.pop_all_groups();
            self.service.add_d2(tr);
        }
    }

    fn set_viewport(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64) {
        if let Some(ref mut tr) = self.render.as_mut() {
            tr.set_scale(minx, miny, maxx, maxy);
        }
    }

    fn push_group(&mut self, name: Option<String>) {
        if let Some(ref mut tr) = self.render.as_mut() {
            if let Some(name) = name {
                tr.push_group_with_name(name);
            } else {
                tr.push_group();
            }
        }
    }

    fn pop_group(&mut self) {
        if let Some(ref mut tr) = self.render.as_mut() {
            tr.pop_group();
        }
    }

    fn add_point(&mut self, p: &(f64, f64), color: String) {
        if let Some(ref mut tr) = self.render.as_mut() {
            tr.add_point(p, color);
        }
    }

    fn add_line(&mut self, a: &(f64, f64), b: &(f64, f64), color: String) {
        if let Some(ref mut tr) = self.render.as_mut() {
            tr.add_line(a, b, color);
        }
    }

    fn add_text(&mut self, p: &(f64, f64), msg: String, color: String, size: f32) {
        if let Some(ref mut tr) = self.render.as_mut() {
            tr.add_text(p, msg, color, size);
        }
    }
}
