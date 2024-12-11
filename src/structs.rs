use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn from_tuple(p: &(u32, u32)) -> Self {
        Self { x: p.0, y: p.1 }
    }

    pub fn from_array(p: &[u32; 2]) -> Self {
        Self { x: p[0], y: p[1] }
    }
}
