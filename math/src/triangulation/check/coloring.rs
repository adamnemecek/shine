pub struct VertexColoring {
    pub color: String,
    pub infinite: String,

    pub text: (String, f32),
    pub infinite_text: (String, f32),
}

impl VertexColoring {
    pub fn new() -> VertexColoring {
        VertexColoring {
            color: "blueviolet".into(),
            infinite: "grey".into(),

            text: ("yellow".into(), 0.05),
            infinite_text: ("grey".into(), 0.05),
        }
    }

    pub fn with_color<S: Into<String>>(mut self, c: S) -> Self {
        self.color = c.into();
        self
    }

    pub fn with_text<S: Into<String>>(mut self, c: S, s: f32) -> Self {
        self.text = (c.into(), s);
        self
    }

    pub fn with_infinite_color<S: Into<String>>(mut self, c: S) -> Self {
        self.infinite = c.into();
        self
    }

    pub fn with_infinite_text<S: Into<String>>(mut self, c: S, s: f32) -> Self {
        self.infinite_text = (c.into(), s);
        self
    }
}

impl Default for VertexColoring {
    fn default() -> VertexColoring {
        VertexColoring::new()
    }
}

pub struct EdgeColoring {
    pub color: String,
    pub constraint: String,
    pub infinite: String,

    pub text: (String, f32),
    pub constraint_text: (String, f32),
    pub infinite_text: (String, f32),
}

impl EdgeColoring {
    pub fn new() -> EdgeColoring {
        EdgeColoring {
            color: "yellow".into(),
            constraint: "green".into(),
            infinite: "grey".into(),

            text: ("yellow".into(), 0.03),
            constraint_text: ("green".into(), 0.03),
            infinite_text: ("grey".into(), 0.03),
        }
    }

    pub fn with_color<S: Into<String>>(mut self, c: S) -> Self {
        self.color = c.into();
        self
    }

    pub fn with_text<S: Into<String>>(mut self, c: S, s: f32) -> Self {
        self.text = (c.into(), s);
        self
    }

    pub fn with_infinite_color<S: Into<String>>(mut self, c: S) -> Self {
        self.infinite = c.into();
        self
    }

    pub fn with_infinite_text<S: Into<String>>(mut self, c: S, s: f32) -> Self {
        self.infinite_text = (c.into(), s);
        self
    }

    pub fn with_constraint_color<S: Into<String>>(mut self, c: S) -> Self {
        self.constraint = c.into();
        self
    }

    pub fn with_constraint_text<S: Into<String>>(mut self, c: S, s: f32) -> Self {
        self.constraint_text = (c.into(), s);
        self
    }
}

impl Default for EdgeColoring {
    fn default() -> EdgeColoring {
        EdgeColoring::new()
    }
}

pub struct FaceColoring {
    pub text: (String, f32),
    pub infinite_text: (String, f32),
}

impl FaceColoring {
    pub fn new() -> FaceColoring {
        FaceColoring {
            text: ("yellow".into(), 0.05),
            infinite_text: ("grey".into(), 0.05),
        }
    }

    pub fn with_text<S: Into<String>>(mut self, c: S, s: f32) -> Self {
        self.text = (c.into(), s);
        self
    }

    pub fn with_infinite_text<S: Into<String>>(mut self, c: S, s: f32) -> Self {
        self.infinite_text = (c.into(), s);
        self
    }
}

impl Default for FaceColoring {
    fn default() -> FaceColoring {
        FaceColoring::new()
    }
}

/// Color settings for the Trace
pub struct Coloring {
    pub vertex: VertexColoring,
    pub edge: EdgeColoring,
    pub face: FaceColoring,

    pub error: String,
    pub error_text: (String, f32),
}

impl Coloring {
    pub fn new() -> Coloring {
        Coloring {
            vertex: VertexColoring::default(),
            edge: EdgeColoring::default(),
            face: FaceColoring::default(),
            error: "red".into(),
            error_text: ("red".into(), 0.05),
        }
    }

    pub fn with_vertex_color(mut self, color: VertexColoring) -> Self {
        self.vertex = color;
        self
    }

    pub fn with_edge_color(mut self, color: EdgeColoring) -> Self {
        self.edge = color;
        self
    }

    pub fn with_face_color(mut self, color: FaceColoring) -> Self {
        self.face = color;
        self
    }
}

impl Default for Coloring {
    fn default() -> Coloring {
        Coloring::new()
    }
}
