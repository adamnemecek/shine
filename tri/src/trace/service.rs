use geometry::Predicates;
use graph::{Face, Graph, Vertex};
use std::thread;
use trace::actix_web::{middleware, server, App, HttpRequest};
use trace::Render;

pub fn start_service() {
    thread::spawn(move || {
        let sys = actix::System::new("shine-tri-debug");

        server::new(|| {
            App::new()
                // enable logger
                .resource("/index.html", |r| r.f(|_| "Hello world!"))
            //.resource("/", |r| r.f(index))
        }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

        let _ = sys.run();
    });
}

pub fn trace<'a, P, V, F>(tri: &'a Graph<P, V, F>)
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    let _r = Render::new(tri);
    loop {}
}
