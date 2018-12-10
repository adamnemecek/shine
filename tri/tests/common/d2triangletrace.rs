#![allow(dead_code)]

use shine_testutils::webserver::{D2Trace, Service};
use shine_tri::{Coloring, TraceControl, TraceCtx, TraceMapping, TraceRender};
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

pub struct D2TriTraceRender {
    service: Service,
    render: Option<D2Trace>,
}

impl TraceRender for D2TriTraceRender {
    fn begin(&mut self) {
        self.end();
        self.render = Some(D2Trace::new());
    }

    fn end(&mut self) {
        let tr = mem::replace(&mut self.render, None);
        if let Some(mut tr) = tr {
            tr.pop_all_layers();
            self.service.add_d2(tr);
        }
    }

    fn set_viewport(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64) {
        if let Some(ref mut tr) = self.render.as_mut() {
            tr.set_scale(minx, miny, maxx, maxy);
        }
    }

    fn push_layer(&mut self) {
        if let Some(ref mut tr) = self.render.as_mut() {
            tr.push_layer();
        }
    }

    fn pop_layer(&mut self) {
        if let Some(ref mut tr) = self.render.as_mut() {
            tr.pop_layer();
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

pub struct D2TriTraceControl {
    service: Service,
    coloring: Coloring,
    mapping: TraceMapping,
}

impl TraceControl for D2TriTraceControl {
    fn coloring(&self) -> &Coloring {
        &self.coloring
    }

    fn coloring_mut(&mut self) -> &mut Coloring {
        &mut self.coloring
    }

    fn mapping(&self) -> &TraceMapping {
        &self.mapping
    }

    fn mapping_mut(&mut self) -> &mut TraceMapping {
        &mut self.mapping
    }

    fn pause(&mut self) {
        self.service.wait_user();
    }
}

pub struct D2TriTrace {
    service: Service,
}

impl D2TriTrace {
    pub fn new(service: Service) -> Self {
        D2TriTrace { service }
    }
}

impl From<D2TriTrace> for TraceCtx<D2TriTraceControl, D2TriTraceRender> {
    fn from(trace: D2TriTrace) -> Self {
        let service = trace.service;
        TraceCtx {
            control: Rc::new(RefCell::new(D2TriTraceControl {
                service: service.clone(),
                coloring: Coloring::default(),
                mapping: TraceMapping::default(),
            })),
            render: Rc::new(RefCell::new(D2TriTraceRender {
                service: service,
                render: None,
            })),
        }
    }
}
