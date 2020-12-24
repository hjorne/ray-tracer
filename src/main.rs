use ray_tracer::base::sim::*;
use ray_tracer::base::tuple::*;

fn main() {
    let r = Tuple::vec(0., 1., 0.);
    let v = Tuple::vec(1., 1., 0.);

    let mut sim = Simulation::new(r, v, 0.1, 0.01);

    sim.run();
}
