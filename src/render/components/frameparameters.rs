use crate::render::{Backend, Buffer, DriverResource};
use gfx_hal::buffer::Offset as BufferOffset;
use nalgebra::Matrix4;
use rendy::factory::Factory;
use rendy::memory::MemoryUsageValue;
use rendy::resource::{BufferInfo, Escape};
use shine_math::camera::RenderCamera;
use std::mem;
use std::ops::Range;

#[derive(Clone, Copy)]
#[repr(C, align(16))]
struct ProjectionArgs {
    proj: Matrix4<f32>,
    view: Matrix4<f32>,
}

const PROJECTION_ARGS_SIZE: BufferOffset = mem::size_of::<ProjectionArgs>() as BufferOffset;

struct FrameParameterResources {
    buffer: Escape<Buffer>,
}

pub struct FrameParameters {
    frame_count: usize,
    frame_ids: Vec<u32>,
    uniform_align: BufferOffset,
    frame_buffer_size: BufferOffset,
    resources: DriverResource<FrameParameterResources>,
}

impl FrameParameters {
    pub fn new() -> FrameParameters {
        FrameParameters {
            frame_count: 0,
            frame_ids: vec![],
            uniform_align: BufferOffset::default(),
            frame_buffer_size: BufferOffset::default(),
            resources: DriverResource::default(),
        }
    }

    pub fn frame_count(&self) -> usize {
        self.frame_count
    }

    fn projection_args_offset(&self, index: usize) -> BufferOffset {
        self.frame_buffer_size * index as BufferOffset
    }

    pub fn projection_args_range(&self, index: usize) -> Range<Option<BufferOffset>> {
        Some(self.projection_args_offset(index))..Some(self.projection_args_offset(index) + PROJECTION_ARGS_SIZE)
    }

    pub fn buffer(&self) -> &Buffer {
        &self.resources.buffer
    }

    pub fn dispose(&mut self, factory: &mut Factory<Backend>) {
        self.resources.dispose(factory);
    }

    pub fn init(&mut self, factory: &Factory<Backend>, frame_count: usize) {
        assert!(self.resources.is_disposed());

        //log::trace!("Init frameparameters");
        self.frame_count = frame_count;
        self.uniform_align = gfx_hal::adapter::PhysicalDevice::limits(factory.physical()).min_uniform_buffer_offset_alignment;
        self.frame_buffer_size = ((PROJECTION_ARGS_SIZE - 1) / self.uniform_align + 1) * self.uniform_align;
        self.frame_ids.resize(self.frame_count, 0);

        let buffer = factory
            .create_buffer(
                BufferInfo {
                    size: self.frame_buffer_size * self.frame_count as BufferOffset,
                    usage: gfx_hal::buffer::Usage::UNIFORM,
                },
                MemoryUsageValue::Dynamic,
            )
            .unwrap();
        self.resources.replace(FrameParameterResources { buffer });
    }

    pub fn update(&mut self, factory: &Factory<Backend>, index: usize, frame_id: u32, camera: &RenderCamera) {
        //log::trace!("Updating frameparameters");
        if self.frame_ids[index] == frame_id {
            return;
        }

        self.frame_ids[index] = frame_id;

        let offset = self.projection_args_offset(index);
        let mut buffer = &mut self.resources.buffer;
        unsafe {
            factory
                .upload_visible_buffer(
                    &mut buffer,
                    offset,
                    &[ProjectionArgs {
                        proj: camera.projection(),
                        view: camera.view(),
                    }],
                )
                .unwrap()
        };
    }
}

impl Default for FrameParameters {
    fn default() -> Self {
        FrameParameters::new()
    }
}
