#![allow(dead_code)]

use render::opengl::lowlevel::*;

#[derive(Clone, Copy)]
struct TextureUnit
{
    hw_id: GLuint,
    target: GLenum,
    //sampling : TextureSampling,
    last_access_time: usize,
}

impl TextureUnit {
    fn new() -> TextureUnit {
        TextureUnit {
            hw_id: 0,
            target: 0,
            //sampling:
            last_access_time: 0,
        }
    }
}


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

    /// Enables/Disables the forced state changed. When enabled, the cached state is ignored
    /// and gl commands are always generated.
    pub fn set_forced(&mut self, force: bool) {
        self.force = force;
    }

    /// Binds the given texture to a logical slot. Slots indices are logical not to ?? with gl slot ids (GL_TEXTURE0)
    pub fn bind_to_slot(&mut self, slot: usize, target: GLenum, hw_id: GLuint/*, TextureSampling sampling*/) {
        // make the texture active
        gl_check_error();
        if self.force || self.active_unit != slot {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + slot as u32);
                self.active_unit = slot;
            }
        }

        let unit = &mut self.texture_units[slot];
        unit.last_access_time = self.time_stamp;

        // update texture binding
        gl_check_error();
        if self.force || unit.target != target || unit.hw_id != hw_id {
            unsafe {
                gl::BindTexture(target, hw_id);
            }
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
            /*if self.force || unit.sampling != sampling {
                unsafe {
                    gl::TexParameteri(unit.target, GL_TEXTURE_MAG_FILTER, sampling.magFilter);
                    gl::TexParameteri(unit.target, GL_TEXTURE_MIN_FILTER, sampling.minFilter);
                    gl::TexParameteri(unit.target, GL_TEXTURE_WRAP_S, sampling.wrapS);
                    gl::TexParameteri(unit.target, GL_TEXTURE_WRAP_T, sampling.wrapT);
                }
                unit.sampling = sampling;
            }*/
            gl_check_error();
        }
    }

    /// Binds the given texture to an arbitrary slot and returns its (GL) id.
    /// Slots are selected by LRU algorithm.
    fn bind(&mut self, target: GLenum, hw_id: GLuint/*, sampling: TextureSampling */) -> usize {
        // finds some slot to (re)use
        let mut slot = 0;
        // don't rebind any texture from this draw call (bindings since the last commit call)
        let mut worst_time = self.time_stamp;
        for (i, unit) in self.texture_units.iter().enumerate() {
            if unit.target == target && unit.hw_id == hw_id /*&& unit.sampling == sampling*/ {
                // this texture already bound
                slot = i;
                break;
            }

            if unit.last_access_time < worst_time {
                worst_time = unit.last_access_time;
                slot = i;
            }
        }

        self.bind_to_slot(slot, target, hw_id);
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
            self.bind_to_slot(slot, target, 0/*, TextureSampling()*/);
        }
    }

    /// Finalizes the texture binding and prepare for the (next) draw call.
    pub fn commit(&mut self) {
        self.time_stamp += 1;
    }
}
