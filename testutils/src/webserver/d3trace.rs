use actix_web::{error, Error as ActixWebError, HttpRequest, HttpResponse};
use gltf_json::Root;
use log::info;
use serde_json;
use webserver::appcontext::AppContext;

pub trait IntoD3Data {
    fn trace(&self, tr: &mut D3Trace);
}

/// Trace 3D geometry object through the web service
pub struct D3Trace {
    gltf: Root,
}

impl D3Trace {
    pub fn new() -> D3Trace {
        D3Trace {
            gltf: Default::default(),
        }
    }

    pub fn to_data(self) -> String {
        serde_json::to_string(&self.gltf).unwrap()
    }
}

impl Default for D3Trace {
    fn default() -> D3Trace {
        D3Trace::new()
    }
}

pub fn handle_d3data_request(req: &HttpRequest<AppContext>) -> Result<HttpResponse, ActixWebError> {
    let state = req.state();

    let id: usize = match req.query().get("id") {
        Some(id) => id
            .parse()
            .map_err(|_| error::ErrorBadRequest(format!("Invalid id: {}", id)))?,
        None => 0,
    };

    let data = {
        info!("Getting d3data for {}", id);
        let mut d3datas = state.d3datas.lock().unwrap();
        if id >= d3datas.len() {
            "".into()
        } else {
            d3datas[id].clone()
        }
    };

    Ok(HttpResponse::Ok().content_type("application/json").body(data))
}

pub fn handle_d3datas_request(req: &HttpRequest<AppContext>) -> Result<HttpResponse, ActixWebError> {
    let state = req.state();

    info!("Getting all d2datas");
    let data = {
        let d3datas = state.d3datas.lock().unwrap();
        d3datas.join(",")
    };
    let data = format!("[{}]", data);
    Ok(HttpResponse::Ok().content_type("application/json").body(data))
}
