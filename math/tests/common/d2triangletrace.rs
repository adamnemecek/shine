#![allow(dead_code)]

use shine_math::trace::Trace2Render;
use shine_math::triangulation::{Coloring, TraceCtx, TriTraceControl, TriTraceMapping};
use shine_testutils::webserver::{D2Trace, Service};
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

pub struct D2TraceRender {
    service: Service,
    render: Option<D2Trace>,
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

pub struct D2TriTraceControl {
    service: Service,
    coloring: Coloring,
    mapping: TriTraceMapping,
}

impl TriTraceControl for D2TriTraceControl {
    fn coloring(&self) -> &Coloring {
        &self.coloring
    }

    fn coloring_mut(&mut self) -> &mut Coloring {
        &mut self.coloring
    }

    fn mapping(&self) -> &TriTraceMapping {
        &self.mapping
    }

    fn mapping_mut(&mut self) -> &mut TriTraceMapping {
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

impl From<D2TriTrace> for TraceCtx<D2TriTraceControl, D2TraceRender> {
    fn from(trace: D2TriTrace) -> Self {
        let service = trace.service;
        TraceCtx {
            control: Rc::new(RefCell::new(D2TriTraceControl {
                service: service.clone(),
                coloring: Coloring::default(),
                mapping: TriTraceMapping::default(),
            })),
            render: Rc::new(RefCell::new(D2TraceRender {
                service: service,
                render: None,
            })),
        }
    }
}
