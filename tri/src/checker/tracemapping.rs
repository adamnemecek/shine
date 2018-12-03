use geometry::Posf64;

/// Vertex trace visualization info
#[derive(Debug)]
pub enum TracePosition {
    Invisible,
    Virtual(Posf64),
    Real(Posf64),
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

    pub fn position(&self) -> &Posf64 {
        match *self {
            TracePosition::Virtual(ref p) => p,
            TracePosition::Real(ref p) => p,
            _ => panic!("No position for {:?}", self),
        }
    }
}

/// Trace helper to map vertices into virtual positions
pub struct TraceMapping {
    pub virtual_positions: Vec<Posf64>,
}

impl TraceMapping {
    pub fn new() -> TraceMapping {
        TraceMapping {
            virtual_positions: Default::default(),
        }
    }

    pub fn clear_virtual_position(&mut self) {
        self.virtual_positions.clear();
    }

    pub fn add_virtual_position<VP: Into<Posf64>>(&mut self, p: VP) {
        self.virtual_positions.push(p.into());
    }

    pub fn set_virtual_positions<VP: IntoIterator<Item = Posf64>>(&mut self, p: VP) {
        self.virtual_positions = p.into_iter().collect();
    }
}

impl Default for TraceMapping {
    fn default() -> TraceMapping {
        TraceMapping::new()
    }
}
