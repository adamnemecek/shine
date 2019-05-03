use crate::camera::{Camera, RenderCamera};
use shine_ecs::entities::{es, IntoJoinExt};
use shine_ecs::shred::System;
use std::marker::PhantomData;

#[derive(Default)]
struct ConvertCameraToRenderCamera<C: Camera> {
    ph: PhantomData<Fn(C)>,
}

impl<'a, C> System<'a> for ConvertCameraToRenderCamera<C>
where
    C: es::Component + Camera,
{
    type SystemData = (es::ReadComponents<'a, C>, es::WriteComponents<'a, RenderCamera>);

    fn run(&mut self, data: Self::SystemData) {
        let (src, mut tgt) = data;

        (src.read(), tgt.update()).join_all(|_, (s, t)| {
            t.set_camera(s);
        });
    }
}
