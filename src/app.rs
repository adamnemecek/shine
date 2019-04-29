use crate::logic::LogicConfig;
use shine_ecs::world::World;
use std::time::Duration;

/// Trait to handle the update during the logic frame
pub trait AppLogicHandler {
    fn update(&mut self, world: &World);
    fn sync(&mut self, logic_world: &mut World, render_world: &mut World);
}

/// Trait to handle the update during the render frame
pub trait AppRenderHandler {
    fn update(&mut self, world: &World);
}

/// Trait for an application
pub trait App {
    type Logic: AppLogicHandler;
    type Render: AppRenderHandler;

    fn prepare_logic(&self, world: &mut World);
    fn prepare_render(&self, logic_world: &mut World, render: &mut World);

    fn create_logic_config(&self) -> LogicConfig {
        LogicConfig::default()
    }

    fn create_logic_handler(&self) -> Self::Logic;
    fn create_render_handler(&self) -> Self::Render;
}
