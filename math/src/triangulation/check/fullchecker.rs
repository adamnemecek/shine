use geometry::Predicates;
use triangulation::check::{GeometryChecker, TopologyChecker};
use triangulation::graph::{Face, PredicatesContext, Triangulation, Vertex};

pub trait FullChecker {
    fn check(&self, area_eps: Option<f64>) -> Result<(), String>;
}

impl<PR, V, F, C> FullChecker for Triangulation<PR::Position, V, F, C>
where
    PR: Predicates,
    V: Vertex<Position = PR::Position>,
    F: Face,
    C: PredicatesContext<Predicates = PR>,
{
    fn check(&self, area_eps: Option<f64>) -> Result<(), String> {
        self.check_dimension()?;
        self.check_topology()?;
        self.check_orientation()?;
        self.check_area(area_eps)?;
        Ok(())
    }
}
