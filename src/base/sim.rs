use super::tuple::*;

struct Projectile {
    r: Tuple,
    v: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

pub struct Simulation {
    projectile: Projectile,
    env: Environment,
}

impl Simulation {
    pub fn new(r: Tuple, v: Tuple, gravity: f64, wind: f64) -> Simulation {
        let env = Environment {
            gravity: Tuple::vec(0., -gravity, 0.),
            wind: Tuple::vec(-wind, 0., 0.),
        };

        let projectile = Projectile { r, v };

        Simulation { projectile, env }
    }

    fn tick(&mut self) {
        self.projectile.r = self.projectile.r + self.projectile.v;
        self.projectile.v = self.projectile.v + self.env.gravity + self.env.wind;
    }

    fn print(&self) {
        println!("Position: ({}, {}, {})", self.projectile.r.x, self.projectile.r.y, self.projectile.r.z);
    }

    pub fn run(&mut self) {
        while self.projectile.r.y > 0. {
            self.print();
            self.tick();
        }
        self.print();
    }
}
