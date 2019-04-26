use crate::logic;
use crate::render::{self, SimpleMeshData};
use crate::voxel::{self, VoxelCell, VoxelMeshSystem};
use nalgebra_glm as glm;
use shine_ecs::shred::{Dispatcher, DispatcherBuilder};
use shine_ecs::world::{EntityWorld, World};
use shine_math::voxel::implicit::function::*;
use std::marker::PhantomData;

pub trait AppLogic {
    fn update(&mut self, world: &World);
    fn sync(&mut self, logic_world: &mut World, render_world: &mut World);
}

pub trait AppRender {
    fn update(&mut self, world: &World);
}

pub trait App {
    type Logic: AppLogic;
    type Render: AppRender;

    fn prepare_logic(&self, world: &mut World);
    fn prepare_render(&self, logic_world: &mut World, render: &mut World);

    fn create_logic(&self) -> Self::Logic;
    fn create_render(&self) -> Self::Render;
}

pub struct DemoLogic<'a> {
    task: Dispatcher<'a, 'a>,
}

impl<'a> DemoLogic<'a> {
    pub fn new<'b>() -> DemoLogic<'b> {
        DemoLogic {
            task: DispatcherBuilder::new()
                .with(VoxelMeshSystem, "VoxelMesherSystem", &[])
                .build(),
        }
    }
}

impl<'a> AppLogic for DemoLogic<'a> {
    fn update(&mut self, world: &World) {
        world.dispatch(&mut self.task);
    }

    fn sync(&mut self, logic_world: &mut World, render_world: &mut World) {
        render_world.sync_entities_to(logic_world, |_, _| {}, |_, _, _| {});
        //todo: sync camera
    }
}

pub struct DemoRender<'a> {
    task: Dispatcher<'a, 'a>,
}

impl<'a> DemoRender<'a> {
    pub fn new<'b>() -> DemoRender<'b> {
        DemoRender {
            task: DispatcherBuilder::new().build(),
        }
    }
}

impl<'a> AppRender for DemoRender<'a> {
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

// GAT, 'l,'r shall be part of the associated type
unsafe impl<'l, 'r> Send for Demo<'l, 'r> {}
unsafe impl<'l, 'r> Sync for Demo<'l, 'r> {}

impl<'l, 'r> App for Demo<'l, 'r> {
    type Logic = DemoLogic<'l>;
    type Render = DemoRender<'r>;

    fn prepare_logic(&self, world: &mut World) {
        logic::prepare_world(world);
        render::prepare_world(world);
        voxel::prepare_world(world);

        let fun = sdf::capsule(glm::vec3(-0.8, -0.8, -0.8), glm::vec3(0.8, 0.7, 0.7), 0.2);

        world
            .create_entity()
            .with(VoxelCell::new_implicit(fun))
            .with(SimpleMeshData::new());
    }

    fn prepare_render(&self, _logic_world: &mut World, render: &mut World) {
        render::prepare_world(render);
    }

    fn create_logic(&self) -> Self::Logic {
        DemoLogic::new()
    }

    fn create_render(&self) -> Self::Render {
        DemoRender::new()
    }
}
