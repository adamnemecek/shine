use std::time::Duration;

/// Common configuration for the logic frames
pub struct LogicConfig {
    logic_frame_length: Duration,
    world_frame_length: Duration,
}

impl LogicConfig {
    pub fn new() -> LogicConfig {
        LogicConfig {
            logic_frame_length: Duration::from_micros(100),
            world_frame_length: Duration::from_micros(100),
        }
    }

    pub fn logic_frame_length(&self) -> Duration {
        self.logic_frame_length
    }

    pub fn world_frame_length(&self) -> Duration {
        self.world_frame_length
    }
}

impl Default for LogicConfig {
    fn default() -> Self {
        LogicConfig::new()
    }
}
