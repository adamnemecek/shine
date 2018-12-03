#![allow(dead_code)]

use shine_testutils::webserver::{D2Trace, Service};
use shine_tri::TraceRender;
use std::mem;

pub struct D2TriTrace {
    service: Service,
    trace_render: Option<D2Trace>,
}

impl D2TriTrace {
    pub fn new(service: Service) -> D2TriTrace {
        D2TriTrace {
            service,
            trace_render: None,
        }
    }
}

impl TraceRender for D2TriTrace {
    fn begin(&mut self) {
        self.end();
        self.trace_render = Some(D2Trace::new());
    }

    fn end(&mut self) {
        let tr = mem::replace(&mut self.trace_render, None);
        if let Some(mut tr) = tr {
            tr.pop_all_layers();
            self.service.add_d2(tr);
        }
    }

    fn set_viewport(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64) {
        if let Some(ref mut tr) = self.trace_render.as_mut() {
            tr.set_scale(minx, miny, maxx, maxy);
        }
    }

    fn push_layer(&mut self) {
        if let Some(ref mut tr) = self.trace_render.as_mut() {
            tr.push_layer();
        }
    }

    fn pop_layer(&mut self) {
        if let Some(ref mut tr) = self.trace_render.as_mut() {
            tr.pop_layer();
        }
    }

    fn add_point(&mut self, p: &(f64, f64), color: String) {
        if let Some(ref mut tr) = self.trace_render.as_mut() {
            tr.add_point(p, color);
        }
    }

    fn add_line(&mut self, a: &(f64, f64), b: &(f64, f64), color: String) {
        if let Some(ref mut tr) = self.trace_render.as_mut() {
            tr.add_line(a, b, color);
        }
    }

    fn add_text(&mut self, p: &(f64, f64), msg: String, color: String, size: f32) {
        if let Some(ref mut tr) = self.trace_render.as_mut() {
            tr.add_text(p, msg, color, size);
        }
    }
}
