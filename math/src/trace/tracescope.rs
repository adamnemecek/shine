use std::ops;

pub trait Trace {
    fn trace_begin(&self);
    fn trace_end(&self);
    fn trace_push_group<S: Into<String>>(&self, name: Option<S>);
    fn trace_pop_group(&self);
    fn trace_pause(&self);

    fn trace_document(&self) -> TraceDocument<'_, Self>
    where
        Self: Sized,
    {
        self.trace_begin();
        TraceDocument { trace: self }
    }

    fn trace_group<S: Into<String>>(&self, name: Option<S>) -> TraceGroup<'_, Self>
    where
        Self: Sized,
    {
        self.trace_push_group(name);
        TraceGroup { trace: self }
    }
}

pub struct TraceDocument<'a, T>
where
    T: Trace,
{
    trace: &'a T,
}

impl<'a, T> ops::Deref for TraceDocument<'a, T>
where
    T: 'a + Trace,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.trace
    }
}

impl<'a, T> Drop for TraceDocument<'a, T>
where
    T: 'a + Trace,
{
    fn drop(&mut self) {
        self.trace.trace_end();
    }
}

pub struct TraceGroup<'a, T>
where
    T: Trace,
{
    trace: &'a T,
}

impl<'a, T> ops::Deref for TraceGroup<'a, T>
where
    T: 'a + Trace,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.trace
    }
}

impl<'a, T> Drop for TraceGroup<'a, T>
where
    T: 'a + Trace,
{
    fn drop(&mut self) {
        self.trace.trace_pop_group();
    }
}
