use actix_web::{error, Error as ActixWebError, HttpRequest, HttpResponse};
use base64;
use bytes::{BufMut, BytesMut};
use shine_gltf;
use log::info;
use serde_json;
use webserver::appcontext::AppContext;

pub trait IntoD3Data {
    fn trace(&self, tr: &mut D3Trace);
}

/// Index of an added mesh to instantiate
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeshId(usize);

/// Location of a mesh instance
pub enum D3Location {
    Identity,
    Matrix([f64; 16]),
    Decomposed {
        translation: [f64; 3],
        rotation: [f64; 4],
        scale: [f64; 3],
    },
}

/// Trace 3D geometry object through the web service
pub struct D3Trace {
    gltf: shine_gltf::Root,
}

impl D3Trace {
    pub fn new() -> D3Trace {
        D3Trace {
            gltf: Default::default(),
        }
    }

    pub fn add_indexed_mesh<V, I>(&mut self, positions: V, indices: I) -> MeshId
    where
        V: IntoIterator<Item = (f64, f64, f64)>,
        I: IntoIterator<Item = usize>,
    {
        let mut data = BytesMut::new();

        for (x, y, z) in positions.into_iter() {
            if data.remaining_mut() < 3*8  {
                // reserve more
                data.reserve(3*8*1024);
            }

            data.put_f64_be(x);
            data.put_f64_be(y);
            data.put_f64_be(z);
        }

        for i in indices.into_iter() {
            if data.remaining_mut() < 4  {
                // reserve more
                data.reserve(4*1024);
            }

            data.put_u32_be(i as u32);
        }

        let encoded_data = base64::encode(&data);

        let buffer_id = {
            let buffer = shine_gltf::Buffer {   
                byte_length: data.len() as u32,
                uri: Some(format!("data:{}", encoded_data)),
                name: Default::default(),        
                extensions: Default::default(),
                extras: Default::default(),
            };
            let id = self.gltf.buffers.len();
            self.gltf.buffers.push(buffer);
            id
        };

        /*let vertexView = shine_gltf::BufferView {
            buffer: 
        }*/

        info!("{:?}", buffer);

        MeshId(0)
    }

    pub fn add_instance(&mut self, mesh: MeshId, location: D3Location) {
        //unimplemented!()
    }

    pub fn add_indexed_mesh_instance<V, I>(&mut self, positions: V, indices: I, location: D3Location) -> MeshId
    where
        V: IntoIterator<Item = (f64, f64, f64)>,
        I: IntoIterator<Item = usize>,
    {
        let id = self.add_indexed_mesh(positions, indices);
        self.add_instance(id, location);
        id
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
