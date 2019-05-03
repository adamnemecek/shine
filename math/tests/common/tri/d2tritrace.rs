#![allow(dead_code)]

use crate::common::D2TraceRender;
use shine_math::triangulation::{Coloring, TraceCtx, TriTraceControl, TriTraceMapping};
use shine_testutils::webserver::Service;
use std::cell::RefCell;
use std::rc::Rc;

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
            render: Rc::new(RefCell::new(D2TraceRender::new(service))),
        }
    }
}
