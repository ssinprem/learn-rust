use rand::{Rng, RngExt};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.
#[derive(Clone)]
pub struct RobotFactory {
    set: Rc<RefCell<HashSet<String>>>,
}

pub struct Robot {
    name: String,
    factory: RobotFactory,
}

impl RobotFactory {
    pub fn new() -> Self {
        Self {
            set: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    pub fn new_robot<R: Rng>(&mut self, rng: &mut R) -> Robot {
        let name = self.random_name(rng);
        Robot::new(name, self.clone())
    }

    pub fn random_name<R: Rng>(&mut self, rng: &mut R) -> String {
        let mut set = self.set.borrow_mut();
        loop {
            let name = (0..2)
                .map(|_| rng.random_range(b'A'..=b'Z') as char)
                .collect::<String>()
                + (0..3)
                    .map(|_| rng.random_range(b'0'..=b'9') as char)
                    .collect::<String>()
                    .as_str();
            if set.insert(name.to_string()) {
                return name;
            }
        }
    }

    pub fn release_name(&mut self, name: String) {
        let mut set = self.set.borrow_mut();
        set.remove(&name);
    }
}

impl Robot {
    pub fn new(name: String, factory: RobotFactory) -> Self {
        Self { name, factory }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset<R: Rng>(&mut self, rng: &mut R) {
        self.factory.release_name(self.name.to_string());
        self.name = self.factory.random_name(rng)
    }
}
