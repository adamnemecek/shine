use std::ops;

pub trait Trace2Render {
    fn begin(&mut self);
    fn end(&mut self);

    fn set_viewport(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64);

    fn push_group(&mut self, name: Option<String>);
    fn pop_group(&mut self);

    fn add_point(&mut self, p: &(f64, f64), color: String);
    fn add_line(&mut self, a: &(f64, f64), b: &(f64, f64), color: String);
    fn add_text(&mut self, p: &(f64, f64), msg: String, color: String, size: f32);
}

pub trait Trace2 {
    fn trace_begin(&self);
    fn trace_end(&self);
    fn trace_push_group<S: Into<String>>(&self, name: Option<S>);
    fn trace_pop_group(&self);
    fn trace_pause(&self);

    fn trace_document(&self) -> Trace2Document<'_, Self>
    where
        Self: Sized,
    {
        self.trace_begin();
        Trace2Document { trace: self }
    }

    fn trace_group<S: Into<String>>(&self, name: Option<S>) -> Trace2Group<'_, Self>
    where
        Self: Sized,
    {
        self.trace_push_group(name);
        Trace2Group { trace: self }
    }
}

pub struct Trace2Document<'a, T>
where
    T: Trace2,
{
    trace: &'a T,
}

impl<'a, T> ops::Deref for Trace2Document<'a, T>
where
    T: 'a + Trace2,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.trace
    }
}

impl<'a, T> Drop for Trace2Document<'a, T>
where
    T: 'a + Trace2,
{
    fn drop(&mut self) {
        self.trace.trace_end();
    }
}

pub struct Trace2Group<'a, T>
where
    T: Trace2,
{
    trace: &'a T,
}

impl<'a, T> ops::Deref for Trace2Group<'a, T>
where
    T: 'a + Trace2,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.trace
    }
}

impl<'a, T> Drop for Trace2Group<'a, T>
where
    T: 'a + Trace2,
{
    fn drop(&mut self) {
        self.trace.trace_pop_group();
    }
}
