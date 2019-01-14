use crate::webserver::appcontext::AppContext;
use actix_web::{error, Error as ActixWebError, HttpRequest, HttpResponse};
use log::info;
use serde_json;
use std::collections::HashMap;
use svg::node::{self, element};
use svg::{Document, Node};
use tera;

pub trait IntoD2Data {
    fn trace(&self, tr: &mut D2Trace);
}

enum Container {
    Root(Document),
    Group(element::Group),
}

impl Container {
    fn add_node<N: Node>(&mut self, node: N) {
        match *self {
            Container::Group(ref mut group) => group.append(node),
            Container::Root(ref mut doc) => doc.append(node),
        }
    }
}

struct Text {
    text: String,
    color: String,
    size: f32,
}

struct Group {
    container: Container,
    texts: HashMap<(i32, i32), Vec<Text>>,
}

impl Group {
    fn new_root() -> Group {
        let doc = Document::new().set("group-name", "root");
        Group {
            container: Container::Root(doc),
            texts: HashMap::new(),
        }
    }

    fn new_group(name: Option<String>) -> Group {
        let mut group = element::Group::new();
        if let Some(name) = name {
            group = group.set("group-name", name);
        }
        Group {
            container: Container::Group(group),
            texts: HashMap::new(),
        }
    }

    fn add_text(&mut self, p: (f64, f64), text: String, color: String, size: f32) {
        let key = ((p.0 * 65536.) as i32, (p.1 * 65546.) as i32);
        let size = size * 0.05;
        self.texts.entry(key).or_insert(Vec::new()).push(Text { text, color, size });
    }

    fn add_node<N: Node>(&mut self, node: N) {
        self.container.add_node(node);
    }

    fn finalize(mut self) -> Container {
        if !self.texts.is_empty() {
            for (pos, texts) in self.texts.iter() {
                let p = (pos.0 as f32 / 65536., pos.1 as f32 / 65536.);
                let mut group = element::Group::new()
                    .set("preserve-size", "true")
                    .set("group-name", "*")
                    .set("transform", format!("translate({},{}) scale(1)", p.0, p.1));

                let mut y = 0.;
                for text in texts.iter() {
                    for line in text.text.split("\n") {
                        y += text.size;
                        let mut node = element::Text::new()
                            .set("x", 0)
                            .set("y", y)
                            .set("font-size", text.size)
                            .set("xml:space", "preserve")
                            .set("fill", text.color.clone());
                        node.append(node::Text::new(line));
                        group.append(node);
                    }
                }
                self.container.add_node(group);
            }
        }

        self.container
    }
}

/// Trace 2D geometry object through the web service
pub struct D2Trace {
    groups: Vec<Group>,
    scale: (f64, f64, f64, f64),
}

impl D2Trace {
    pub fn new() -> D2Trace {
        D2Trace {
            groups: vec![Group::new_root()],
            scale: (1., -1., 0., 0.),
        }
    }

    pub fn push_group(&mut self) {
        self.groups.push(Group::new_group(None));
    }

    pub fn push_group_with_name<S: Into<String>>(&mut self, name: S) {
        self.groups.push(Group::new_group(Some(name.into())));
    }

    pub fn pop_group(&mut self) {
        let group = self.groups.pop().unwrap();
        match group.finalize() {
            Container::Group(group) => self.add_node(group),
            _ => panic!("Poping root group"),
        }
    }

    pub fn pop_all_groups(&mut self) {
        while self.groups.len() > 1 {
            self.pop_group();
        }
    }

    pub fn to_data(mut self) -> String {
        self.pop_all_groups();

        let group = self.groups.pop().unwrap();
        match group.finalize() {
            Container::Root(mut document) => {
                //document.assign("width", "640");
                document.assign("viewbox", "-1 -1 2 2");
                document.to_string()
            }

            _ => panic!("Poping root group"),
        }
    }

    pub fn set_scale(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64) {
        let w = maxx - minx;
        let h = maxy - miny;
        let w = if w == 0. { 1. } else { w };
        let h = if h == 0. { 1. } else { h };

        self.scale.0 = 2. / w;
        self.scale.1 = -2. / h;
        self.scale.2 = -(minx + maxx) / w;
        self.scale.3 = (miny + maxy) / h;
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

    pub fn add_text<S: Into<String>>(&mut self, p: &(f64, f64), msg: S, color: String, size: f32) {
        let p = self.scale_position(p);

        let group = self.groups.last_mut().unwrap();
        group.add_text(p, msg.into(), color, size);
    }

    fn add_node<N: Node>(&mut self, node: N) {
        let group = self.groups.last_mut().unwrap();
        group.add_node(node);
    }
}

impl Default for D2Trace {
    fn default() -> D2Trace {
        D2Trace::new()
    }
}

pub fn handle_d2data_request(req: &HttpRequest<AppContext>) -> Result<HttpResponse, ActixWebError> {
    let state = req.state();

    // input is 1 based
    let id: usize = match req.query().get("id") {
        Some(id) => id
            .parse()
            .map_err(|_| error::ErrorBadRequest(format!("Invalid id: {}", id)))?,
        None => 1,
    };

    // convert to 0 based
    let id = if id == 0 { usize::max_value() } else { id - 1 };
    let image = {
        info!("Getting d2data for {}", id);
        let img = state.d2datas.lock().unwrap();
        if id >= img.len() {
            "<svg xmlns=\"http://www.w3.org/2000/svg\" group-name=\"root\" viewbox=\"-1 -1 2 2\"></svg>".into()
        } else {
            img[id].clone()
        }
    };

    Ok(HttpResponse::Ok().content_type("image/svg+xml").body(image))
}

pub fn handle_d2datas_request(req: &HttpRequest<AppContext>) -> Result<HttpResponse, ActixWebError> {
    let state = req.state();

    info!("Getting all d2datas");
    let data = {
        let img = state.d2datas.lock().unwrap();
        serde_json::to_string(&*img).unwrap()
    };

    Ok(HttpResponse::Ok().content_type("application/json").body(data))
}

pub fn handle_d2view_request(req: &HttpRequest<AppContext>) -> Result<HttpResponse, ActixWebError> {
    let state = req.state();

    let all_data = {
        let img = state.d2datas.lock().unwrap();
        serde_json::to_string(&*img).unwrap()
    };

    let mut ctx = tera::Context::new();
    ctx.insert("svg_list", &all_data);

    let body = state.template.render("d2view.html", &ctx).map_err(|e| {
        println!("Template error: {}", e);
        error::ErrorInternalServerError(format!("Template error: {}", e))
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
