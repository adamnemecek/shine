/// Color settings for the Trace
pub struct Coloring {
    pub vertex: String,
    pub vertex_text: (String, f32),
    pub edge: String,
    pub edge_text: (String, f32),
    pub face_text: (String, f32),

    pub constraint_edge: String,
    pub constraint_edge_text: (String, f32),

    pub infinite_vertex: String,
    pub infinite_vertex_text: (String, f32),
    pub infinite_edge: String,
    pub infinite_edge_text: (String, f32),
    pub infinite_face_text: (String, f32),

    pub error: String,
    pub error_text: (String, f32),
}

impl Coloring {
    pub fn new() -> Coloring {
        Coloring {
            vertex: "blueviolet".into(),
            vertex_text: ("yellow".into(), 0.05),
            edge: "yellow".into(),
            edge_text: ("yellow".into(), 0.03),
            face_text: ("yellow".into(), 0.05),

            constraint_edge: "green".into(),
            constraint_edge_text: ("green".into(), 0.03),

            infinite_vertex: "grey".into(),
            infinite_vertex_text: ("grey".into(), 0.05),
            infinite_edge: "grey".into(),
            infinite_edge_text: ("grey".into(), 0.03),
            infinite_face_text: ("grey".into(), 0.05),

            error: "red".into(),
            error_text: ("red".into(), 0.05),
        }
    }
}

impl Default for Coloring {
    fn default() -> Coloring {
        Coloring::new()
    }
}