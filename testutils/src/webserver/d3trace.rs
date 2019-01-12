use actix_web::{error, Error as ActixWebError, HttpRequest, HttpResponse};
use base64;
use bytes::{BufMut, BytesMut};
use log::info;
use serde_json;
use shine_gltf::{buffer, Get, GetMut, Buffer, Root, Mesh, Node, Scene, Index};
use webserver::appcontext::AppContext;

pub trait IntoD3Data {
    fn trace(&self, tr: &mut D3Trace);
}

/// Index of an added mesh to instantiate
#[derive(Clone, Debug, PartialEq)]
pub struct MeshId(Index<Mesh>);

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
    root: Root,
}

impl D3Trace {
    pub fn new() -> D3Trace {        
        let mut root = Root::default();
        let scene_id = root.add_scene(Scene::default());
        root.scene = Some(scene_id);
        
        D3Trace { root }
    }

    pub fn add_indexed_mesh<V, I>(&mut self, positions: V, indices: I) -> MeshId
    where
        V: IntoIterator<Item = (f64, f64, f64)>,
        I: IntoIterator<Item = usize>,
    {
        let mut data = BytesMut::new();

        for (x, y, z) in positions.into_iter() {
            if data.remaining_mut() < 3 * 8 {
                // reserve more
                data.reserve(3 * 8 * 1024);
            }

            data.put_f64_be(x);
            data.put_f64_be(y);
            data.put_f64_be(z);
        }
        let position_byte_count = data.len();
        let position_byte_offset = 0;
        let position_byte_stride = 3 * 8;

        for i in indices.into_iter() {
            if data.remaining_mut() < 4 {
                // reserve more
                data.reserve(4 * 1024);
            }

            data.put_u32_be(i as u32);
        }
        let index_byte_count = data.len() - position_byte_count;
        let index_byte_offset = position_byte_count;
        let index_byte_stride = 4;

        let encoded_data = base64::encode(&data);

        let buffer_id = {
            let buffer = Buffer {
                byte_length: data.len() as u32,
                uri: Some(format!("data:{}", encoded_data)),
                ..Default::default()
            };
            self.root.add_buffer(buffer)
        };

        let position_view_id = {
            let buffer_view = buffer::View {
                byte_length: position_byte_count as u32,
                byte_offset: Some(position_byte_offset as u32),
                byte_stride: Some(buffer::ByteStride(position_byte_stride as u32)),
                ..buffer::View::with_buffer(buffer_id.clone())
            };
            self.root.add_buffer_view(buffer_view)
        };

        let index_view_id = {
            let buffer_view = buffer::View {
                byte_length: index_byte_count as u32,
                byte_offset: Some(index_byte_offset as u32),
                byte_stride: Some(buffer::ByteStride(index_byte_stride as u32)),
                ..buffer::View::with_buffer(buffer_id.clone())
            };
            self.root.add_buffer_view(buffer_view)
        };

        let mesh_id = {
            let mesh = Mesh {
                ..Default::default()
            };
            self.root.add_mesh(mesh)
        };

        info!("{:?}", self.root.to_string_pretty());

        MeshId(mesh_id)
    }

    pub fn add_instance(&mut self, mesh: MeshId, location: D3Location) {
        let node_id = {
            let node = Node {
                mesh: Some(mesh.0),
                ..Default::default()
            };
            self.root.add_node(node)
        };

        let scene = self.root.scene.as_ref().unwrap().clone();
        let scene = self.root.get_mut(&scene).unwrap();
        scene.nodes.push(node_id);
    }

    pub fn add_indexed_mesh_instance<V, I>(&mut self, positions: V, indices: I, location: D3Location) -> MeshId
    where
        V: IntoIterator<Item = (f64, f64, f64)>,
        I: IntoIterator<Item = usize>,
    {
        let id = self.add_indexed_mesh(positions, indices);
        self.add_instance(id.clone(), location);
        id
    }

    pub fn to_data(self) -> String {
        serde_json::to_string(&self.root).unwrap()
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

    info!("Getting all d3datas");
    let data = {
        let d3datas = state.d3datas.lock().unwrap();
        d3datas.join(",")
    };
    let data = format!("[{}]", data);
    Ok(HttpResponse::Ok().content_type("application/json").body(data))
}

pub fn handle_d3view_request(req: &HttpRequest<AppContext>) -> Result<HttpResponse, ActixWebError> {
    let state = req.state();

    let all_data = {
        let img = state.d3datas.lock().unwrap();
        serde_json::to_string(&*img).unwrap()
    };

    let mut ctx = tera::Context::new();
    ctx.insert("model_list", &all_data);

    let body = state.template.render("d3view.html", &ctx).map_err(|e| {
        println!("Template error: {}", e);
        error::ErrorInternalServerError(format!("Template error: {}", e))
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
