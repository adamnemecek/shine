use rendy::factory::Factory;
use std::intrinsics::type_name;
use std::marker::PhantomData;
use std::ops;

/// Helper to manage the lifetime of render resources.
/// As shred does not support removal of resources, this inner-nullable object
/// is used to make sure no resources out-lives the driver.
/// Eventough rust has lifetime infrastucutre, the rendering low level api is mainly unsafe and
/// lifetime gurantees are not always ensured.
pub struct DriverResource<T, B>
where
    B: gfx_hal::Backend,
{
    inner: Option<T>,
    phantom: PhantomData<fn() -> B>,
}

impl<T, B> DriverResource<T, B>
where
    B: gfx_hal::Backend,
{
    pub fn new() -> DriverResource<T, B> {
        DriverResource {
            inner: None,
            phantom: PhantomData,
        }
    }

    pub fn new_with(data: T) -> DriverResource<T, B> {
        DriverResource {
            inner: Some(data),
            phantom: PhantomData,
        }
    }

    pub fn dispose(&mut self, _factory: &mut Factory<B>) {
        log::trace!("disposing resource {}", unsafe { type_name::<T>() });
        self.inner.take();
    }
}

impl<T, B> Drop for DriverResource<T, B>
where
    B: gfx_hal::Backend,
{
    fn drop(&mut self) {
        assert!(
            self.inner.is_none(),
            "Driver resource is leaking, have to be disposed explicitly"
        )
    }
}

impl<T, B> ops::Deref for DriverResource<T, B>
where
    B: gfx_hal::Backend,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.inner.as_ref().unwrap()
    }
}

impl<T, B> ops::DerefMut for DriverResource<T, B>
where
    B: gfx_hal::Backend,
{
    fn deref_mut(&mut self) -> &mut T {
        self.inner.as_mut().unwrap()
    }
}

impl<T, B> From<T> for DriverResource<T, B>
where
    B: gfx_hal::Backend,
{
    fn from(data: T) -> Self {
        DriverResource::new_with(data)
    }
}
