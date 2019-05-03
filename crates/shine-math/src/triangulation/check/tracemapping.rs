use nalgebra_glm as glm;

/// Vertex trace visualization info
#[derive(Debug)]
pub enum TracePosition {
    Invisible,
    Virtual(glm::DVec2),
    Real(glm::DVec2),
}

impl TracePosition {
    pub fn is_visible(&self) -> bool {
        match *self {
            TracePosition::Virtual(_) => true,
            TracePosition::Real(_) => true,
            _ => false,
        }
    }

    pub fn is_virtual(&self) -> bool {
        match *self {
            TracePosition::Virtual(_) => true,
            _ => false,
        }
    }

    pub fn position(&self) -> &glm::DVec2 {
        match *self {
            TracePosition::Virtual(ref p) => p,
            TracePosition::Real(ref p) => p,
            _ => panic!("No position for {:?}", self),
        }
    }
}

/// Trace helper to map vertices into virtual positions
pub struct TriTraceMapping {
    pub virtual_positions: Vec<glm::DVec2>,
}

impl TriTraceMapping {
    pub fn new() -> TriTraceMapping {
        TriTraceMapping {
            virtual_positions: Default::default(),
        }
    }

    pub fn clear_virtual_position(&mut self) {
        self.virtual_positions.clear();
    }

    pub fn add_virtual_position<VP: Into<glm::DVec2>>(&mut self, p: VP) {
        self.virtual_positions.push(p.into());
    }

    pub fn set_virtual_positions<VP: IntoIterator<Item = glm::DVec2>>(&mut self, p: VP) {
        self.virtual_positions = p.into_iter().collect();
    }
}

impl Default for TriTraceMapping {
    fn default() -> TriTraceMapping {
        TriTraceMapping::new()
    }
}
