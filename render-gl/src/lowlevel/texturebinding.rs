#![allow(dead_code)]

use libconfig::*;
use lowlevel::*;


/// Texture sampling parameters
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GLTextureFilter {
    pub mag_filter: GLenum,
    pub min_filter: GLenum,
    pub wrap_s: GLenum,
    pub wrap_t: GLenum,
}

impl GLTextureFilter {
    fn new() -> GLTextureFilter {
        GLTextureFilter {
            mag_filter: 0,
            min_filter: 0,
            wrap_s: 0,
            wrap_t: 0,
        }
    }
}


/// The current texture bound for the GL
#[derive(Clone, Copy)]
struct TextureUnit
{
    hw_id: GLuint,
    target: GLenum,
    filter: GLTextureFilter,
    last_access_time: usize,
}

impl TextureUnit {
    fn new() -> TextureUnit {
        TextureUnit {
            hw_id: 0,
            target: 0,
            filter: GLTextureFilter::new(),
            last_access_time: 0,
        }
    }
}


/// Handle texture binding states
pub struct TextureBinding {
    force: bool,
    time_stamp: usize,
    texture_units: [TextureUnit; MAX_USED_TEXTURE_COUNT],
    active_unit: usize,
}

impl TextureBinding {
    pub fn new() -> TextureBinding {
        TextureBinding {
            force: false,
            time_stamp: 1,
            texture_units: [TextureUnit::new(); MAX_USED_TEXTURE_COUNT],
            active_unit: 0,
        }
    }

    /// Finds the internal and upload enums:
    /// [0] internal format - Specifies the internal format of the stored texture
    /// [1] format - Specifies the format of the source texel data
    /// [2] type - Specifies the data type of the source texel data
    pub fn glenum_from_pixel_format(fmt: PixelFormat) -> (GLenum, GLenum, GLenum) {
        match fmt {
            PixelFormat::R8 => (gl::RED, gl::RED, gl::UNSIGNED_BYTE),
            PixelFormat::Rgb8 => (gl::RGB, gl::RGB, gl::UNSIGNED_BYTE),
            PixelFormat::Rgba8 => (gl::RGBA, gl::RGBA, gl::UNSIGNED_BYTE),
        }
    }

    /// Enables/Disables the forced state changed. When enabled, the cached state is ignored
    /// and gl commands are always generated.
    pub fn set_forced(&mut self, force: bool) {
        self.force = force;
    }

    /// Binds the given texture to a logical slot. Slots indices are logical not to confuse with gl slot ids (GL_TEXTURE0)
    pub fn bind_to_slot(&mut self, slot: usize, target: GLenum, hw_id: GLuint, filter: GLTextureFilter) {
        // make the texture active
        gl_check_error();
        if self.force || self.active_unit != slot {
            ffi!(gl::ActiveTexture(gl::TEXTURE0 + slot as u32));
            self.active_unit = slot;
        }

        let unit = &mut self.texture_units[slot];
        unit.last_access_time = self.time_stamp;

        // update texture binding
        gl_check_error();
        if self.force || unit.target != target || unit.hw_id != hw_id {
            ffi!(gl::BindTexture(target, hw_id));
            if hw_id == 0 {
                *unit = TextureUnit::new();
            } else {
                unit.target = target;
                unit.hw_id = hw_id;
            }
        }
        gl_check_error();

        // update texture parameters
        if hw_id != 0 {
            if self.force || unit.filter != filter {
                ffi!(gl::TexParameteri(unit.target, gl::TEXTURE_MAG_FILTER, filter.mag_filter as i32));
                ffi!(gl::TexParameteri(unit.target, gl::TEXTURE_MIN_FILTER, filter.min_filter as i32));
                ffi!(gl::TexParameteri(unit.target, gl::TEXTURE_WRAP_S, filter.wrap_s as i32));
                ffi!(gl::TexParameteri(unit.target, gl::TEXTURE_WRAP_T, filter.wrap_t as i32));
                unit.filter = filter;
            }
            gl_check_error();
        }
    }

    /// Binds the given texture to an arbitrary slot and returns the index of its slot.
    /// Slots are selected by LRU algorithm.
    pub fn bind(&mut self, target: GLenum, hw_id: GLuint, filter: GLTextureFilter) -> usize {
        // finds some slot to (re)use
        let mut slot = 0;
        // don't rebind any texture from this draw call (bindings since the last commit call)
        let mut worst_time = self.time_stamp;
        for (i, unit) in self.texture_units.iter().enumerate() {
            if unit.target == target && unit.hw_id == hw_id && unit.filter == filter {
                // this texture already bound
                slot = i;
                break;
            }

            if unit.last_access_time < worst_time {
                worst_time = unit.last_access_time;
                slot = i;
            }
        }

        self.bind_to_slot(slot, target, hw_id, filter);
        return slot;
    }

    /// Unbinds a texture if it is active. This function is mainly used during release.
    pub fn unbind_if_active(&mut self, hw_id: GLuint) {
        if hw_id == 0 {
            return;
        }

        let mut slot = usize::max_value();
        let mut target = 0;
        for (i, unit) in self.texture_units.iter().enumerate() {
            if unit.hw_id == hw_id {
                slot = i;
                target = unit.target;
                break;
            }
        }

        if slot != usize::max_value() {
            self.bind_to_slot(slot, target, 0, GLTextureFilter::new());
        }
    }

    /// Finalizes the texture binding and prepare for the (next) draw call.
    pub fn commit(&mut self) {
        self.time_stamp += 1;
    }
}
