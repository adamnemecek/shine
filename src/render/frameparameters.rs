use crate::render::{Backend, Buffer, DriverResource};
use gfx_hal::buffer::Offset as BufferOffset;
use nalgebra::Matrix4;
use rendy::factory::Factory;
use rendy::memory::MemoryUsageValue;
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
    buffer: Buffer,
}

pub struct FrameParameters {
    frame_count: usize,
    frame_ids: Vec<u32>,
    uniform_align: BufferOffset,
    frame_buffer_size: BufferOffset,
    resources: DriverResource<FrameParameterResources, Backend>,
}

impl FrameParameters {
    pub fn new(factory: &mut Factory<Backend>, frame_count: usize) -> FrameParameters {
        let uniform_align = gfx_hal::adapter::PhysicalDevice::limits(factory.physical()).min_uniform_buffer_offset_alignment;
        let frame_buffer_size = ((PROJECTION_ARGS_SIZE - 1) / uniform_align + 1) * uniform_align;

        let buffer = factory
            .create_buffer(
                uniform_align,
                frame_buffer_size * frame_count as BufferOffset,
                (gfx_hal::buffer::Usage::UNIFORM, MemoryUsageValue::Dynamic),
            )
            .unwrap();

        let mut frame_ids = Vec::with_capacity(frame_count);
        frame_ids.resize(frame_count, 0);

        FrameParameters {
            uniform_align,
            frame_count,
            frame_ids,
            frame_buffer_size,
            resources: DriverResource::from(FrameParameterResources { buffer }),
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

    pub fn update(&mut self, factory: &Factory<Backend>, index: usize, frame_id: u32, camera: &RenderCamera) {
        log::trace!("Updating frameparameters");
        if self.frame_ids[index] == frame_id {
            log::trace!("already up to date");
            return;
        }

        let offset = self.projection_args_offset(index);
        let mut buffer = &mut self.resources.buffer;
        unsafe {
            factory
                .upload_visible_buffer(
                    &mut buffer,
                    offset,
                    &[ProjectionArgs {
                        proj: nalgebra::Isometry3::identity().into(),//camera.projection(),
                        view: nalgebra::Isometry3::identity().into(),//camera.view(),
                    }],
                )
                .unwrap()
        };
    }
}
