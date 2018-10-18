use std::collections::HashMap;
use svg::node::{element, Text};
use svg::{Document, Node};

pub trait IntoD2Image {
    fn trace(&self, tr: &mut D2Trace);
}

/// Trace 2D geometry object through the web service
pub struct D2Trace {
    document: Document,
    layers: Vec<element::Group>,
    scale: (f64, f64, f64, f64),
    text_map: HashMap<(i32, i32), usize>,
}

impl D2Trace {
    pub fn new() -> D2Trace {
        D2Trace {
            document: Document::new(),
            layers: Default::default(),
            scale: (1., 1., 0., 0.),
            text_map: Default::default(),
        }
    }

    pub fn push_layer(&mut self) {
        self.layers.push(element::Group::new());
    }

    pub fn pop_layer(&mut self) {
        let v = self.layers.pop().unwrap();
        self.add_node(v);
    }

    pub fn pop_all_layers(&mut self) {
        while !self.layers.is_empty() {
            self.pop_layer();
        }
    }

    pub fn set_scale(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64) {
        let w = maxx - minx;
        let h = maxy - miny;
        let w = if w == 0. { 1. } else { w };
        let h = if h == 0. { 1. } else { h };

        self.scale.0 = 2. / w;
        self.scale.1 = 2. / h;
        self.scale.2 = -(minx + maxx) / w;
        self.scale.3 = -(miny + maxy) / h;
        self.document.assign("width", "640");
        self.document.assign("height", "auto");
        self.document.assign("viewbox", "-1 -1 2 2");
    }

    pub fn scale_position(&self, p: &(f64, f64)) -> (f64, f64) {
        (p.0 * self.scale.0 + self.scale.2, p.1 * self.scale.1 + self.scale.3)
    }

    pub fn add_point(&mut self, p: &(f64, f64), color: String) {
        let p = self.scale_position(p);
        let node = element::Line::new()
            .set("x1", p.0)
            .set("y1", p.1)
            .set("x2", p.0)
            .set("y2", p.1)
            .set("vector-effect", "non-scaling-stroke")
            .set("stroke-linecap", "round")
            .set("stroke", color)
            .set("stroke-width", "4");
        self.add_node(node);
    }

    pub fn add_line(&mut self, a: &(f64, f64), b: &(f64, f64), color: String) {
        let a = self.scale_position(a);
        let b = self.scale_position(b);
        let node = element::Line::new()
            .set("x1", a.0)
            .set("y1", a.1)
            .set("x2", b.0)
            .set("y2", b.1)
            .set("vector-effect", "non-scaling-stroke")
            .set("stroke-linecap", "round")
            .set("stroke", color)
            .set("stroke-width", "2");
        self.add_node(node);
    }

    pub fn add_text(&mut self, p: &(f64, f64), msg: String, color: String) {
        let p = self.scale_position(p);

        let offset = {
            let key = ((p.0 * 65536.) as i32, (p.1 * 65536.) as i32);
            let count = self.text_map.entry(key).or_insert(0);
            *count += 1;
            *count as f64 * 0.05
        };

        let mut node = element::Text::new()
            .set("x", p.0)
            .set("y", p.1 + offset)
            .set("font-size", "0.05")
            .set("fill", color);
        node.append(Text::new(msg));
        self.add_node(node);
    }

    fn add_node<N: Node>(&mut self, node: N) {
        if let Some(p) = self.layers.last_mut() {
            p.append(node);
        } else {
            self.document.append(node);
        }
    }
}

impl Default for D2Trace {
    fn default() -> D2Trace {
        D2Trace::new()
    }
}

impl ToString for D2Trace {
    fn to_string(&self) -> String {
        self.document.to_string()
    }
}
