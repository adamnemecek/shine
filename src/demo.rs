use crate::app::{App, AppLogicHandler, AppRenderHandler};
use crate::render;
use shine_ecs::shred::{Dispatcher, DispatcherBuilder};
use shine_ecs::world::{EntityWorld, ResourceWorld, World};
use shine_shard::camera;
use std::marker::PhantomData;

pub struct DemoLogicHandler<'a> {
    task: Dispatcher<'a, 'a>,
}

impl<'a> DemoLogicHandler<'a> {
    pub fn new<'b>() -> DemoLogicHandler<'b> {
        DemoLogicHandler {
            task: DispatcherBuilder::new()
                //.with(VoxelMeshSystem, "VoxelMesherSystem", &[])
                .build(),
        }
    }
}

impl<'a> AppLogicHandler for DemoLogicHandler<'a> {
    fn update(&mut self, world: &World) {
        world.dispatch(&mut self.task);
    }

    fn sync(&mut self, logic_world: &mut World, render_world: &mut World) {
        render_world.sync_entities_to(logic_world, |_, _| {}, |_, _, _| {});
        //todo: sync camera
    }
}

pub struct DemoRenderHandler<'a> {
    task: Dispatcher<'a, 'a>,
}

impl<'a> DemoRenderHandler<'a> {
    pub fn new<'b>() -> DemoRenderHandler<'b> {
        DemoRenderHandler {
            task: DispatcherBuilder::new().build(),
        }
    }
}

impl<'a> AppRenderHandler for DemoRenderHandler<'a> {
    fn update(&mut self, world: &World) {
        world.dispatch(&mut self.task);

        /*{
            let elapsed_time = frame_stats.get_last_frame_time().as_micros() as f32 / 1_000_000.0_f32;

            let mut frame = world.resource_mut::<render::FrameInfo>();
            frame.frame_id = frame_stats.get_frame_count();
            frame.elapsed_time = elapsed_time;

            let input_manager = world.resource::<InputManager>();
            let input_state = input_manager.get_state();
            let mut cam = world.resource_mut::<FpsCamera>();

            let dist = elapsed_time;
            let angle_dist = elapsed_time;

            cam.move_forward(input_state.get_button(buttons::MOVE_FORWARD) * dist);
            cam.move_side(input_state.get_button(buttons::MOVE_SIDE) * dist);
            cam.move_up(input_state.get_button(buttons::MOVE_UP) * dist);
            cam.yaw(input_state.get_button(buttons::YAW) * angle_dist);
            cam.roll(input_state.get_button(buttons::ROLL) * angle_dist);
            cam.pitch(input_state.get_button(buttons::PITCH) * angle_dist);

            let mut rcam = world.resource_mut::<RenderCamera>();
            log::info!("update rcam {:?}", rcam.view_matrix());
            rcam.set_camera(&*cam);
        }*/
    }
}

#[derive(Default)]
pub struct Demo<'l, 'r> {
    ph: PhantomData<Fn() -> (&'l (), &'r ())>,
}

// GAT, 'l,'r shall be part of the associated Logic and Render types
unsafe impl<'l, 'r> Send for Demo<'l, 'r> {}
unsafe impl<'l, 'r> Sync for Demo<'l, 'r> {}

impl<'l, 'r> App for Demo<'l, 'r> {
    type Logic = DemoLogicHandler<'l>;
    type Render = DemoRenderHandler<'r>;

    fn prepare_logic(&self, world: &mut World) {
        world.register_resource_with(camera::RawCamera::new());
        //voxel::prepare_world(world);

        //let fun = sdf::capsule(glm::vec3(-0.8, -0.8, -0.8), glm::vec3(0.8, 0.7, 0.7), 0.2);

        /*world
        .create_entity()
        .with(VoxelCell::new_implicit(fun))
        .with(SimpleMeshData::new());*/
    }

    fn prepare_render(&self, _logic_world: &mut World, render_world: &mut World) {
        render_world.register_resource_with(camera::RenderCamera::new());
        render_world.register_resource_with(camera::FpsCamera::new());
        render_world.register_resource::<render::FrameParameters>();
        render_world.register_entity_component::<render::SimpleMeshData>();
        render_world.register_entity_component::<render::SimpleMesh>();
    }

    fn create_logic_handler(&self) -> Self::Logic {
        DemoLogicHandler::new()
    }

    fn create_render_handler(&self) -> Self::Render {
        DemoRenderHandler::new()
    }
}
