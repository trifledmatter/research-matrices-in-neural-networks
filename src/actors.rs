use std::f64::consts::E;

#[derive(Clone)]
pub struct Actor<'a> {
    pub function: &'a dyn Fn(f64) -> f64,
    pub derivative: &'a dyn Fn(f64) -> f64,
}

pub const IDENTITY: Actor = Actor {
    function: &|x| x,
    derivative: &|_| 1.0,
};

pub const SIGMOID: Actor = Actor {
    function: &|x| 1.0 / (1.0 + E.powf(-x)),
    derivative: &|x| x * (1.0 - x),
};

pub const TANH: Actor = Actor {
    function: &|x| x.tanh(),
    derivative: &|x| 1.0 - (x.powi(2)),
};

pub const RELU: Actor = Actor {
    function: &|x| x.max(0.0),
    derivative: &|x| if x > 0.0 { 1.0 } else { 0.0 },
};
